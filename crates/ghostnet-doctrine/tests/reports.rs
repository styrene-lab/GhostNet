use ghostnet_doctrine::{ReportArtifact, ReportError, ReportLedger, ReportOperation, ReportStatus};

fn report(
    artifact: &str,
    hash: &str,
    operation: ReportOperation,
    target: Option<&str>,
) -> ReportArtifact {
    ReportArtifact {
        artifact_id: artifact.into(),
        body_hash: hash.into(),
        operation,
        target_body_hash: target.map(str::to_owned),
        created_at_ms: 10,
        expires_at_ms: None,
    }
}

#[test]
fn correction_preserves_source_and_links_replacement() {
    let mut ledger = ReportLedger::default();
    ledger
        .apply(report("a", "h1", ReportOperation::Publish, None), 10)
        .expect("publish");
    ledger
        .apply(report("b", "h2", ReportOperation::Correct, Some("h1")), 20)
        .expect("correct");

    let original = ledger.get("h1").expect("original");
    assert_eq!(original.status, ReportStatus::Corrected);
    assert_eq!(original.superseded_by.as_deref(), Some("h2"));
    assert_eq!(
        ledger.get("h2").expect("correction").status,
        ReportStatus::Current
    );
}

#[test]
fn retraction_is_explicit_and_cannot_be_retracted_twice() {
    let mut ledger = ReportLedger::default();
    ledger
        .apply(report("a", "h1", ReportOperation::Publish, None), 10)
        .expect("publish");
    ledger
        .apply(report("b", "h2", ReportOperation::Retract, Some("h1")), 20)
        .expect("retract");
    assert_eq!(
        ledger.get("h1").expect("original").status,
        ReportStatus::Retracted
    );
    assert_eq!(
        ledger.apply(report("c", "h3", ReportOperation::Retract, Some("h1")), 30),
        Err(ReportError::TargetRetracted)
    );
}

#[test]
fn expiry_is_derived_without_deleting_report() {
    let mut ledger = ReportLedger::default();
    let mut expiring = report("a", "h1", ReportOperation::Publish, None);
    expiring.expires_at_ms = Some(100);
    ledger.apply(expiring, 10).expect("publish");
    assert_eq!(
        ledger.get("h1").expect("report").status,
        ReportStatus::Current
    );
    ledger.evaluate_expiry(100);
    assert_eq!(
        ledger.get("h1").expect("report").status,
        ReportStatus::Expired
    );
}

#[test]
fn duplicate_is_idempotent_and_conflicting_reuse_fails() {
    let mut ledger = ReportLedger::default();
    let original = report("a", "h1", ReportOperation::Publish, None);
    assert_eq!(ledger.apply(original.clone(), 10), Ok(true));
    assert_eq!(ledger.apply(original, 10), Ok(false));
    assert_eq!(
        ledger.apply(report("a", "other", ReportOperation::Publish, None), 10),
        Err(ReportError::ArtifactConflict)
    );
}

#[test]
fn missing_target_is_rejected() {
    let mut ledger = ReportLedger::default();
    assert_eq!(
        ledger.apply(
            report("b", "h2", ReportOperation::Correct, Some("missing")),
            20
        ),
        Err(ReportError::MissingTarget)
    );
}
