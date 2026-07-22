use ghostnet_doctrine::{
    EnforcementLevel, ExtensionDisposition, PolicyDecision, PolicyExtensionResult, PolicyOutcome,
    Posture, ReasonCode, compose_policy,
};

fn safety(outcome: PolicyOutcome) -> PolicyDecision {
    PolicyDecision {
        outcome,
        reason: ReasonCode::AuthorizedDirective,
        effective_posture: Posture::Routine,
        enforcement: EnforcementLevel::ClientGuarded,
        selected_issuer: Some("authority".into()),
    }
}

fn extension(outcome: PolicyOutcome) -> PolicyExtensionResult {
    PolicyExtensionResult {
        outcome,
        reason: "mission bundle rule".into(),
        engine_id: "rego-wasm".into(),
        engine_version: "pinned-test".into(),
        bundle_id: "mission-alpha".into(),
        bundle_hash: format!("sha256:{}", "a".repeat(64)),
        claimed_enforcement: None,
    }
}

#[test]
fn extension_can_restrict_safety_permit() {
    let composed = compose_policy(
        safety(PolicyOutcome::Permit),
        Some(extension(PolicyOutcome::Deny)),
    );
    assert_eq!(composed.decision.outcome, PolicyOutcome::Deny);
    assert_eq!(composed.decision.reason, ReasonCode::ExtensionRestricted);
    assert!(matches!(
        composed.extension,
        ExtensionDisposition::Applied { .. }
    ));
}

#[test]
fn extension_cannot_relax_safety_deny_or_indeterminate() {
    for safety_outcome in [PolicyOutcome::Deny, PolicyOutcome::Indeterminate] {
        let composed = compose_policy(
            safety(safety_outcome),
            Some(extension(PolicyOutcome::Permit)),
        );
        assert_eq!(composed.decision.outcome, safety_outcome);
        assert_eq!(composed.decision.reason, ReasonCode::AuthorizedDirective);
    }
}

#[test]
fn missing_extension_preserves_safety_result() {
    let composed = compose_policy(safety(PolicyOutcome::RequireConfirmation), None);
    assert_eq!(
        composed.decision.outcome,
        PolicyOutcome::RequireConfirmation
    );
    assert_eq!(composed.extension, ExtensionDisposition::NotEvaluated);
}

#[test]
fn extension_cannot_inflate_enforcement_evidence() {
    let mut dynamic = extension(PolicyOutcome::Permit);
    dynamic.claimed_enforcement = Some(EnforcementLevel::DaemonEnforced);
    let composed = compose_policy(safety(PolicyOutcome::Permit), Some(dynamic));
    assert_eq!(composed.decision.outcome, PolicyOutcome::Deny);
    assert_eq!(composed.decision.reason, ReasonCode::InvalidExtensionClaim);
    assert_eq!(
        composed.decision.enforcement,
        EnforcementLevel::ClientGuarded
    );
    assert!(matches!(
        composed.extension,
        ExtensionDisposition::RejectedClaim { .. }
    ));
}

#[test]
fn confirmation_cannot_be_relaxed_to_permit() {
    let composed = compose_policy(
        safety(PolicyOutcome::RequireConfirmation),
        Some(extension(PolicyOutcome::Permit)),
    );
    assert_eq!(
        composed.decision.outcome,
        PolicyOutcome::RequireConfirmation
    );
}
