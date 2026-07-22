use ghostnet_doctrine::{
    ActorAuthorization, BearerFacts, Confidentiality, DisclosureClass, EmissionRequest,
    EnforcementLevel, PolicyOutcome, Posture, PostureDirective, ReasonCode, evaluate_emission,
    select_directive,
};

fn directive(issuer: &str, posture: Posture, priority: u16, issued_at_ms: u64) -> PostureDirective {
    PostureDirective {
        posture,
        issued_at_ms,
        expires_at_ms: None,
        priority,
        issuer: issuer.into(),
        authorization: ActorAuthorization::Authorized,
        requires_confirmation: false,
        confirmed: true,
    }
}

fn bearer(enforcement: EnforcementLevel) -> BearerFacts {
    BearerFacts {
        can_transmit: true,
        confidentiality: Confidentiality::EndToEnd,
        enforcement,
    }
}

fn request() -> EmissionRequest {
    EmissionRequest {
        disclosure: DisclosureClass::Operational,
        emergency: false,
        operator_confirmed: false,
    }
}

#[test]
fn higher_priority_beats_newer_and_ties_are_deterministic() {
    let directives = [
        directive("zulu", Posture::Elevated, 10, 200),
        directive("bravo", Posture::Restricted, 20, 100),
        directive("alpha", Posture::Silent, 20, 100),
    ];
    let selected = select_directive(&directives, 300)
        .expect("authorization known")
        .expect("selected");
    assert_eq!(selected.issuer, "alpha");
    assert_eq!(selected.posture, Posture::Silent);
}

#[test]
fn expired_unauthorized_and_future_directives_do_not_apply() {
    let mut expired = directive("expired", Posture::Silent, 100, 10);
    expired.expires_at_ms = Some(20);
    let mut unauthorized = directive("unauthorized", Posture::Silent, 100, 10);
    unauthorized.authorization = ActorAuthorization::Unauthorized;
    let future = directive("future", Posture::Silent, 100, 1000);

    assert_eq!(
        select_directive(&[expired, unauthorized, future], 100),
        Ok(None)
    );
}

#[test]
fn unknown_authorization_is_indeterminate_when_no_authorized_override_exists() {
    let mut unknown = directive("unknown", Posture::Silent, 10, 10);
    unknown.authorization = ActorAuthorization::Unknown;
    let decision = evaluate_emission(
        &[unknown],
        bearer(EnforcementLevel::ClientGuarded),
        request(),
        20,
    );
    assert_eq!(decision.outcome, PolicyOutcome::Indeterminate);
    assert_eq!(decision.reason, ReasonCode::AuthorizationUnknown);
}

#[test]
fn directive_confirmation_is_required_before_other_evaluation() {
    let mut restricted = directive("authority", Posture::Restricted, 10, 10);
    restricted.requires_confirmation = true;
    restricted.confirmed = false;
    let decision = evaluate_emission(
        &[restricted],
        bearer(EnforcementLevel::DaemonEnforced),
        request(),
        20,
    );
    assert_eq!(decision.outcome, PolicyOutcome::RequireConfirmation);
    assert_eq!(decision.reason, ReasonCode::ConfirmationRequired);
}

#[test]
fn confidential_report_is_denied_without_end_to_end_confidentiality() {
    let mut facts = bearer(EnforcementLevel::ClientGuarded);
    facts.confidentiality = Confidentiality::Link;
    let mut emission = request();
    emission.disclosure = DisclosureClass::Confidential;
    let decision = evaluate_emission(&[], facts, emission, 20);
    assert_eq!(decision.outcome, PolicyOutcome::Deny);
    assert_eq!(decision.reason, ReasonCode::ConfidentialityUnavailable);
}

#[test]
fn restricted_emergency_requires_then_accepts_operator_confirmation() {
    let restricted = directive("authority", Posture::Restricted, 10, 10);
    let mut emission = request();
    emission.emergency = true;
    let first = evaluate_emission(
        std::slice::from_ref(&restricted),
        bearer(EnforcementLevel::ClientGuarded),
        emission,
        20,
    );
    assert_eq!(first.outcome, PolicyOutcome::RequireConfirmation);

    emission.operator_confirmed = true;
    let confirmed = evaluate_emission(
        &[restricted],
        bearer(EnforcementLevel::ClientGuarded),
        emission,
        20,
    );
    assert_eq!(confirmed.outcome, PolicyOutcome::Permit);
    assert_eq!(confirmed.reason, ReasonCode::EmergencyConfirmed);
}

#[test]
fn silent_posture_denies_even_emergency_and_reports_actual_enforcement() {
    let silent = directive("authority", Posture::Silent, 10, 10);
    let mut emission = request();
    emission.emergency = true;
    emission.operator_confirmed = true;
    let decision = evaluate_emission(
        &[silent],
        bearer(EnforcementLevel::ClientGuarded),
        emission,
        20,
    );
    assert_eq!(decision.outcome, PolicyOutcome::Deny);
    assert_eq!(decision.reason, ReasonCode::SilentPosture);
    assert_eq!(decision.enforcement, EnforcementLevel::ClientGuarded);
}

#[test]
fn hardware_receive_only_is_not_inferred_from_client_posture() {
    let silent = directive("authority", Posture::Silent, 10, 10);
    let decision = evaluate_emission(&[silent], bearer(EnforcementLevel::Advisory), request(), 20);
    assert_eq!(decision.enforcement, EnforcementLevel::Advisory);
    assert_ne!(decision.enforcement, EnforcementLevel::HardwareReceiveOnly);
}
