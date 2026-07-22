//! Deterministic report correction, retraction, and expiry derivation.

use std::collections::BTreeMap;

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportOperation {
    Publish,
    Correct,
    Retract,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportArtifact {
    pub artifact_id: String,
    pub body_hash: String,
    pub operation: ReportOperation,
    pub target_body_hash: Option<String>,
    pub created_at_ms: u64,
    pub expires_at_ms: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportStatus {
    Current,
    Corrected,
    Retracted,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportView {
    pub body_hash: String,
    pub status: ReportStatus,
    pub superseded_by: Option<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ReportError {
    #[error("artifact identifier was reused with different canonical bytes")]
    ArtifactConflict,
    #[error("publish operations cannot specify a target")]
    UnexpectedTarget,
    #[error("correction or retraction requires an existing target")]
    MissingTarget,
    #[error("target report is already retracted")]
    TargetRetracted,
    #[error("correction or retraction artifact is expired")]
    OperationExpired,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReportLedger {
    artifacts: BTreeMap<String, String>,
    reports: BTreeMap<String, ReportView>,
    expiries: BTreeMap<String, u64>,
}

impl ReportLedger {
    /// Applies one verified report artifact idempotently.
    ///
    /// # Errors
    ///
    /// Rejects conflicting artifact IDs, malformed target relationships,
    /// missing/retracted targets, and expired corrective operations.
    pub fn apply(&mut self, artifact: ReportArtifact, now_ms: u64) -> Result<bool, ReportError> {
        if let Some(hash) = self.artifacts.get(&artifact.artifact_id) {
            return if hash == &artifact.body_hash {
                Ok(false)
            } else {
                Err(ReportError::ArtifactConflict)
            };
        }
        if artifact.operation != ReportOperation::Publish
            && artifact
                .expires_at_ms
                .is_some_and(|expiry| now_ms >= expiry)
        {
            return Err(ReportError::OperationExpired);
        }

        match artifact.operation {
            ReportOperation::Publish => {
                if artifact.target_body_hash.is_some() {
                    return Err(ReportError::UnexpectedTarget);
                }
                self.reports.insert(
                    artifact.body_hash.clone(),
                    ReportView {
                        body_hash: artifact.body_hash.clone(),
                        status: if artifact
                            .expires_at_ms
                            .is_some_and(|expiry| now_ms >= expiry)
                        {
                            ReportStatus::Expired
                        } else {
                            ReportStatus::Current
                        },
                        superseded_by: None,
                    },
                );
                if let Some(expiry) = artifact.expires_at_ms {
                    self.expiries.insert(artifact.body_hash.clone(), expiry);
                }
            }
            ReportOperation::Correct | ReportOperation::Retract => {
                let target_hash = artifact
                    .target_body_hash
                    .as_ref()
                    .ok_or(ReportError::MissingTarget)?;
                let target = self
                    .reports
                    .get_mut(target_hash)
                    .ok_or(ReportError::MissingTarget)?;
                if target.status == ReportStatus::Retracted {
                    return Err(ReportError::TargetRetracted);
                }
                target.status = if artifact.operation == ReportOperation::Correct {
                    ReportStatus::Corrected
                } else {
                    ReportStatus::Retracted
                };
                target.superseded_by = Some(artifact.body_hash.clone());
                self.reports.insert(
                    artifact.body_hash.clone(),
                    ReportView {
                        body_hash: artifact.body_hash.clone(),
                        status: ReportStatus::Current,
                        superseded_by: None,
                    },
                );
            }
        }

        self.artifacts
            .insert(artifact.artifact_id, artifact.body_hash);
        Ok(true)
    }

    pub fn evaluate_expiry(&mut self, now_ms: u64) {
        for (hash, expiry) in &self.expiries {
            if now_ms >= *expiry
                && let Some(report) = self.reports.get_mut(hash)
                && report.status == ReportStatus::Current
            {
                report.status = ReportStatus::Expired;
            }
        }
    }

    #[must_use]
    pub fn get(&self, body_hash: &str) -> Option<&ReportView> {
        self.reports.get(body_hash)
    }
}
