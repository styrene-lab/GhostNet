//! Deterministic emission posture and disclosure-policy evaluation.

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Posture {
    Routine,
    Elevated,
    Restricted,
    Silent,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EnforcementLevel {
    Advisory,
    ClientGuarded,
    DaemonEnforced,
    HardwareReceiveOnly,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DisclosureClass {
    Public,
    Operational,
    Confidential,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Confidentiality {
    None,
    Link,
    EndToEnd,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActorAuthorization {
    Authorized,
    Unauthorized,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostureDirective {
    pub posture: Posture,
    pub issued_at_ms: u64,
    pub expires_at_ms: Option<u64>,
    pub priority: u16,
    pub issuer: String,
    pub authorization: ActorAuthorization,
    pub requires_confirmation: bool,
    pub confirmed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BearerFacts {
    pub can_transmit: bool,
    pub confidentiality: Confidentiality,
    pub enforcement: EnforcementLevel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EmissionRequest {
    pub disclosure: DisclosureClass,
    pub emergency: bool,
    pub operator_confirmed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyOutcome {
    Permit,
    Deny,
    RequireConfirmation,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReasonCode {
    AuthorizedDirective,
    NoApplicableDirective,
    AuthorizationUnknown,
    ConfirmationRequired,
    SilentPosture,
    RestrictedPosture,
    BearerCannotTransmit,
    ConfidentialityUnavailable,
    EmergencyConfirmed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyDecision {
    pub outcome: PolicyOutcome,
    pub reason: ReasonCode,
    pub effective_posture: Posture,
    pub enforcement: EnforcementLevel,
    pub selected_issuer: Option<String>,
}

/// Selects the applicable directive using deterministic precedence.
///
/// Unauthorized, expired, and not-yet-issued directives are ignored. Among
/// authorized directives, higher priority wins, then newer issue time, then
/// lexicographically smaller issuer as a deterministic tie-breaker.
///
/// # Errors
///
/// Returns [`ReasonCode::AuthorizationUnknown`] when a fresh directive has
/// unknown authorization and no authorized directive can determine posture.
pub fn select_directive(
    directives: &[PostureDirective],
    now_ms: u64,
) -> Result<Option<&PostureDirective>, ReasonCode> {
    let mut saw_unknown = false;
    let selected = directives
        .iter()
        .filter(|directive| {
            let fresh = directive.issued_at_ms <= now_ms
                && directive.expires_at_ms.is_none_or(|expiry| now_ms < expiry);
            if fresh && directive.authorization == ActorAuthorization::Unknown {
                saw_unknown = true;
            }
            fresh && directive.authorization == ActorAuthorization::Authorized
        })
        .max_by(|left, right| {
            left.priority
                .cmp(&right.priority)
                .then(left.issued_at_ms.cmp(&right.issued_at_ms))
                .then_with(|| right.issuer.cmp(&left.issuer))
        });
    if selected.is_none() && saw_unknown {
        Err(ReasonCode::AuthorizationUnknown)
    } else {
        Ok(selected)
    }
}

/// Evaluates whether one emission request may use a bearer.
#[must_use]
pub fn evaluate_emission(
    directives: &[PostureDirective],
    bearer: BearerFacts,
    request: EmissionRequest,
    now_ms: u64,
) -> PolicyDecision {
    let directive = match select_directive(directives, now_ms) {
        Ok(directive) => directive,
        Err(reason) => {
            return decision(
                PolicyOutcome::Indeterminate,
                reason,
                Posture::Restricted,
                bearer.enforcement,
                None,
            );
        }
    };
    let posture = directive.map_or(Posture::Routine, |value| value.posture);
    let issuer = directive.map(|value| value.issuer.clone());

    if let Some(value) = directive
        && value.requires_confirmation
        && !value.confirmed
    {
        return decision(
            PolicyOutcome::RequireConfirmation,
            ReasonCode::ConfirmationRequired,
            posture,
            bearer.enforcement,
            issuer,
        );
    }
    if !bearer.can_transmit {
        return decision(
            PolicyOutcome::Deny,
            ReasonCode::BearerCannotTransmit,
            posture,
            bearer.enforcement,
            issuer,
        );
    }
    if request.disclosure == DisclosureClass::Confidential
        && bearer.confidentiality != Confidentiality::EndToEnd
    {
        return decision(
            PolicyOutcome::Deny,
            ReasonCode::ConfidentialityUnavailable,
            posture,
            bearer.enforcement,
            issuer,
        );
    }
    if posture == Posture::Silent {
        return decision(
            PolicyOutcome::Deny,
            ReasonCode::SilentPosture,
            posture,
            bearer.enforcement,
            issuer,
        );
    }
    if posture == Posture::Restricted {
        if request.emergency && request.operator_confirmed {
            return decision(
                PolicyOutcome::Permit,
                ReasonCode::EmergencyConfirmed,
                posture,
                bearer.enforcement,
                issuer,
            );
        }
        return decision(
            if request.emergency {
                PolicyOutcome::RequireConfirmation
            } else {
                PolicyOutcome::Deny
            },
            if request.emergency {
                ReasonCode::ConfirmationRequired
            } else {
                ReasonCode::RestrictedPosture
            },
            posture,
            bearer.enforcement,
            issuer,
        );
    }

    decision(
        PolicyOutcome::Permit,
        directive.map_or(ReasonCode::NoApplicableDirective, |_| {
            ReasonCode::AuthorizedDirective
        }),
        posture,
        bearer.enforcement,
        issuer,
    )
}

fn decision(
    outcome: PolicyOutcome,
    reason: ReasonCode,
    effective_posture: Posture,
    enforcement: EnforcementLevel,
    selected_issuer: Option<String>,
) -> PolicyDecision {
    PolicyDecision {
        outcome,
        reason,
        effective_posture,
        enforcement,
        selected_issuer,
    }
}
