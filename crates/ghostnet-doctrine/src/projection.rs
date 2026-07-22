//! Deterministic bounded disclosure projections.

use thiserror::Error;

use crate::{CanonicalError, body_hash, canonicalize};
use serde_json::{Value, json};

const COMPLETE_SCHEMA: &str = "ghostnet-projection-complete-v1";
const TEXT_SCHEMA: &str = "ghostnet-projection-text-v1";
const SUMMARY_SCHEMA: &str = "ghostnet-projection-summary-v1";
const COMPACT_SCHEMA: &str = "ghostnet-projection-compact-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionSource {
    pub artifact_id: String,
    pub body_hash: String,
    pub report_reference: String,
    pub priority: String,
    pub created_at_ms: u64,
    pub area_class: String,
    pub summary: String,
    pub details: String,
    pub attachment_references: Vec<String>,
    pub compact_code: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionPermission {
    Allowed,
    Forbidden,
}

impl ProjectionPermission {
    const fn is_allowed(self) -> bool {
        matches!(self, Self::Allowed)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectionForms {
    pub details: ProjectionPermission,
    pub attachment_references: ProjectionPermission,
    pub summary: ProjectionPermission,
    pub compact_code: ProjectionPermission,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionProfile {
    pub audience: String,
    pub projector: String,
    pub created_at_ms: u64,
    pub maximum_serialized_bytes: usize,
    pub forms: ProjectionForms,
    pub requires_projection_signature: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionForm {
    Complete,
    TextOnly,
    Summary,
    CompactCode,
}

impl ProjectionForm {
    const fn schema(self) -> &'static str {
        match self {
            Self::Complete => COMPLETE_SCHEMA,
            Self::TextOnly => TEXT_SCHEMA,
            Self::Summary => SUMMARY_SCHEMA,
            Self::CompactCode => COMPACT_SCHEMA,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OmissionReason {
    Policy { field: &'static str },
    BearerLimit { field: &'static str },
}

impl OmissionReason {
    fn as_value(&self) -> Value {
        match self {
            Self::Policy { field } => json!({"field": field, "reason": "policy"}),
            Self::BearerLimit { field } => json!({"field": field, "reason": "bearer_limit"}),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionPlan {
    pub source_artifact_id: String,
    pub source_body_hash: String,
    pub projection_schema: &'static str,
    pub audience: String,
    pub projector: String,
    pub form: ProjectionForm,
    pub canonical_body: Vec<u8>,
    pub projection_body_hash: String,
    pub omitted_fields: Vec<OmissionReason>,
    pub requires_projection_signature: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ProjectionError {
    #[error("projection byte limit must be positive")]
    ZeroLimit,
    #[error("source hash must use the sha256:<lowercase hex> contract form")]
    InvalidSourceHash,
    #[error("no safe projection fits the bearer limit")]
    NoSafeProjection,
    #[error("projection cannot be canonicalized: {0}")]
    Canonical(#[from] CanonicalError),
}

/// Selects the richest policy-allowed projection whose canonical bytes fit the
/// bearer limit. Canonical bytes are never truncated.
///
/// # Errors
///
/// Rejects zero limits, malformed source hashes, canonicalization failures, and
/// cases where no policy-allowed safe projection fits.
pub fn project(
    source: &ProjectionSource,
    profile: &ProjectionProfile,
) -> Result<ProjectionPlan, ProjectionError> {
    if profile.maximum_serialized_bytes == 0 {
        return Err(ProjectionError::ZeroLimit);
    }
    if !valid_hash(&source.body_hash) {
        return Err(ProjectionError::InvalidSourceHash);
    }

    for form in candidates(profile) {
        let omissions = omissions(form, profile);
        let body = projection_value(source, profile, form, &omissions);
        let canonical_body = canonicalize(&body)?;
        if canonical_body.len() <= profile.maximum_serialized_bytes {
            return Ok(ProjectionPlan {
                source_artifact_id: source.artifact_id.clone(),
                source_body_hash: source.body_hash.clone(),
                projection_schema: form.schema(),
                audience: profile.audience.clone(),
                projector: profile.projector.clone(),
                form,
                projection_body_hash: body_hash(&canonical_body),
                canonical_body,
                omitted_fields: omissions,
                requires_projection_signature: profile.requires_projection_signature,
            });
        }
    }
    Err(ProjectionError::NoSafeProjection)
}

fn candidates(profile: &ProjectionProfile) -> Vec<ProjectionForm> {
    let mut forms = Vec::with_capacity(4);
    if profile.forms.details.is_allowed() && profile.forms.attachment_references.is_allowed() {
        forms.push(ProjectionForm::Complete);
    }
    if profile.forms.details.is_allowed() {
        forms.push(ProjectionForm::TextOnly);
    }
    if profile.forms.summary.is_allowed() {
        forms.push(ProjectionForm::Summary);
    }
    if profile.forms.compact_code.is_allowed() {
        forms.push(ProjectionForm::CompactCode);
    }
    forms
}

fn omissions(form: ProjectionForm, profile: &ProjectionProfile) -> Vec<OmissionReason> {
    let mut values = Vec::new();
    if form != ProjectionForm::Complete {
        values.push(if profile.forms.attachment_references.is_allowed() {
            OmissionReason::BearerLimit {
                field: "attachment_references",
            }
        } else {
            OmissionReason::Policy {
                field: "attachment_references",
            }
        });
    }
    if matches!(form, ProjectionForm::Summary | ProjectionForm::CompactCode) {
        values.push(if profile.forms.details.is_allowed() {
            OmissionReason::BearerLimit { field: "details" }
        } else {
            OmissionReason::Policy { field: "details" }
        });
    }
    if form == ProjectionForm::CompactCode {
        values.push(OmissionReason::BearerLimit { field: "summary" });
    }
    values
}

fn projection_value(
    source: &ProjectionSource,
    profile: &ProjectionProfile,
    form: ProjectionForm,
    omissions: &[OmissionReason],
) -> Value {
    let mut body = json!({
        "audience": profile.audience,
        "created_at_ms": profile.created_at_ms,
        "form": match form {
            ProjectionForm::Complete => "complete",
            ProjectionForm::TextOnly => "text_only",
            ProjectionForm::Summary => "summary",
            ProjectionForm::CompactCode => "compact_code",
        },
        "omitted_fields": omissions.iter().map(OmissionReason::as_value).collect::<Vec<_>>(),
        "projector": profile.projector,
        "report_reference": source.report_reference,
        "source_artifact_id": source.artifact_id,
        "source_body_hash": source.body_hash,
    });
    let object = body.as_object_mut().expect("literal is an object");
    match form {
        ProjectionForm::Complete => {
            object.insert("priority".into(), json!(source.priority));
            object.insert("reported_at_ms".into(), json!(source.created_at_ms));
            object.insert("area_class".into(), json!(source.area_class));
            object.insert("summary".into(), json!(source.summary));
            object.insert("details".into(), json!(source.details));
            object.insert(
                "attachment_references".into(),
                json!(source.attachment_references),
            );
        }
        ProjectionForm::TextOnly => {
            object.insert("priority".into(), json!(source.priority));
            object.insert("reported_at_ms".into(), json!(source.created_at_ms));
            object.insert("area_class".into(), json!(source.area_class));
            object.insert("summary".into(), json!(source.summary));
            object.insert("details".into(), json!(source.details));
        }
        ProjectionForm::Summary => {
            object.insert("priority".into(), json!(source.priority));
            object.insert("reported_at_ms".into(), json!(source.created_at_ms));
            object.insert("area_class".into(), json!(source.area_class));
            object.insert("summary".into(), json!(source.summary));
        }
        ProjectionForm::CompactCode => {
            object.insert("code".into(), json!(source.compact_code));
        }
    }
    body
}

fn valid_hash(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
