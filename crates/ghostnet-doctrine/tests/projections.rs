use ghostnet_doctrine::{
    OmissionReason, ProjectionError, ProjectionForm, ProjectionForms, ProjectionPermission,
    ProjectionProfile, ProjectionSource, project,
};

fn source() -> ProjectionSource {
    ProjectionSource {
        artifact_id: "01JAZ6G5P7Q8R9S0T1V2W3X4Y5".into(),
        body_hash: format!("sha256:{}", "a".repeat(64)),
        report_reference: "RPT-ALPHA-7".into(),
        priority: "immediate".into(),
        created_at_ms: 1_735_689_600_123,
        area_class: "district".into(),
        summary: "Bridge clear at checkpoint alpha; two lanes remain passable for emergency and logistics traffic; approach from the east and maintain reduced speed near debris".into(),
        details: "Two lanes are passable; debris remains on the shoulder. Survey team confirmed the western expansion joint is stable, marked the damaged guardrail, and requested a follow-up engineering inspection before heavy convoy use. Photographic evidence and route notes are attached for the receiving operations cell.".into(),
        attachment_references: vec![
            "attachment:photo-1-long-content-addressed-reference".into(),
            "attachment:route-notes-long-content-addressed-reference".into(),
        ],
        compact_code: "CLR-BRG".into(),
    }
}

fn profile(limit: usize) -> ProjectionProfile {
    ProjectionProfile {
        audience: "net:alpha".into(),
        projector: "styrene:identity:gateway".into(),
        created_at_ms: 1_735_689_700_000,
        maximum_serialized_bytes: limit,
        forms: ProjectionForms {
            details: ProjectionPermission::Allowed,
            attachment_references: ProjectionPermission::Allowed,
            summary: ProjectionPermission::Allowed,
            compact_code: ProjectionPermission::Allowed,
        },
        requires_projection_signature: true,
    }
}

fn size_for(form: ProjectionForm) -> usize {
    let mut generous = profile(usize::MAX);
    generous.forms.attachment_references = if form == ProjectionForm::Complete {
        ProjectionPermission::Allowed
    } else {
        ProjectionPermission::Forbidden
    };
    generous.forms.details = if matches!(form, ProjectionForm::Complete | ProjectionForm::TextOnly)
    {
        ProjectionPermission::Allowed
    } else {
        ProjectionPermission::Forbidden
    };
    generous.forms.summary = if form == ProjectionForm::Summary {
        ProjectionPermission::Allowed
    } else {
        ProjectionPermission::Forbidden
    };
    generous.forms.compact_code = if form == ProjectionForm::CompactCode {
        ProjectionPermission::Allowed
    } else {
        ProjectionPermission::Forbidden
    };
    project(&source(), &generous)
        .expect("single form projection")
        .canonical_body
        .len()
}

#[test]
fn generous_bearer_selects_complete_projection() {
    let plan = project(&source(), &profile(4_096)).expect("projection");
    assert_eq!(plan.form, ProjectionForm::Complete);
    assert!(plan.omitted_fields.is_empty());
    assert!(plan.canonical_body.len() <= 4_096);
    assert_eq!(plan.source_body_hash, source().body_hash);
    assert!(plan.requires_projection_signature);
}

#[test]
fn degradation_order_is_deterministic_and_every_result_fits() {
    let complete = size_for(ProjectionForm::Complete);
    let text = size_for(ProjectionForm::TextOnly);
    let summary = size_for(ProjectionForm::Summary);
    let compact = size_for(ProjectionForm::CompactCode);
    assert!(complete > text && text > summary && summary > compact);

    let cases = [
        (complete, ProjectionForm::Complete),
        (complete - 1, ProjectionForm::TextOnly),
        (text - 1, ProjectionForm::Summary),
        (summary - 1, ProjectionForm::CompactCode),
    ];
    for (limit, expected) in cases {
        let plan = project(&source(), &profile(limit)).expect("safe projection");
        assert_eq!(plan.form, expected);
        assert!(plan.canonical_body.len() <= limit);
    }
}

#[test]
fn omission_metadata_distinguishes_policy_from_bearer_limit() {
    let mut policy = profile(4_096);
    policy.forms.attachment_references = ProjectionPermission::Forbidden;
    let plan = project(&source(), &policy).expect("text projection");
    assert_eq!(plan.form, ProjectionForm::TextOnly);
    assert_eq!(
        plan.omitted_fields,
        vec![OmissionReason::Policy {
            field: "attachment_references"
        }]
    );

    let summary_limit = size_for(ProjectionForm::Summary);
    let plan = project(&source(), &profile(summary_limit + 64)).expect("summary projection");
    assert_eq!(plan.form, ProjectionForm::Summary);
    assert!(plan.omitted_fields.contains(&OmissionReason::BearerLimit {
        field: "attachment_references"
    }));
    assert!(
        plan.omitted_fields
            .contains(&OmissionReason::BearerLimit { field: "details" })
    );
}

#[test]
fn no_form_is_ever_truncated_to_fit() {
    let compact = size_for(ProjectionForm::CompactCode);
    assert_eq!(
        project(&source(), &profile(compact - 1)),
        Err(ProjectionError::NoSafeProjection)
    );
}

#[test]
fn policy_can_forbid_degraded_forms() {
    let mut strict = profile(size_for(ProjectionForm::Summary));
    strict.forms.summary = ProjectionPermission::Forbidden;
    strict.forms.compact_code = ProjectionPermission::Forbidden;
    assert_eq!(
        project(&source(), &strict),
        Err(ProjectionError::NoSafeProjection)
    );
}

#[test]
fn malformed_source_hash_and_zero_limit_fail_closed() {
    let mut malformed = source();
    malformed.body_hash = "sha256:ABC".into();
    assert_eq!(
        project(&malformed, &profile(4_096)),
        Err(ProjectionError::InvalidSourceHash)
    );
    assert_eq!(
        project(&source(), &profile(0)),
        Err(ProjectionError::ZeroLimit)
    );
}

#[test]
fn identical_inputs_produce_identical_bytes_and_hash() {
    let left = project(&source(), &profile(4_096)).expect("left");
    let right = project(&source(), &profile(4_096)).expect("right");
    assert_eq!(left.canonical_body, right.canonical_body);
    assert_eq!(left.projection_body_hash, right.projection_body_hash);
}
