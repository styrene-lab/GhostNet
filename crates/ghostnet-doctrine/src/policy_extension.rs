//! Composition boundary between compiled safety invariants and extensions.

use crate::{EnforcementLevel, PolicyDecision, PolicyOutcome, ReasonCode};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RestrictionLevel {
    Permit,
    RequireConfirmation,
    Indeterminate,
    Deny,
}

impl From<PolicyOutcome> for RestrictionLevel {
    fn from(value: PolicyOutcome) -> Self {
        match value {
            PolicyOutcome::Permit => Self::Permit,
            PolicyOutcome::RequireConfirmation => Self::RequireConfirmation,
            PolicyOutcome::Indeterminate => Self::Indeterminate,
            PolicyOutcome::Deny => Self::Deny,
        }
    }
}

impl From<RestrictionLevel> for PolicyOutcome {
    fn from(value: RestrictionLevel) -> Self {
        match value {
            RestrictionLevel::Permit => Self::Permit,
            RestrictionLevel::RequireConfirmation => Self::RequireConfirmation,
            RestrictionLevel::Indeterminate => Self::Indeterminate,
            RestrictionLevel::Deny => Self::Deny,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyExtensionResult {
    pub outcome: PolicyOutcome,
    pub reason: String,
    pub engine_id: String,
    pub engine_version: String,
    pub bundle_id: String,
    pub bundle_hash: String,
    pub claimed_enforcement: Option<EnforcementLevel>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExtensionDisposition {
    NotEvaluated,
    Applied {
        engine_id: String,
        engine_version: String,
        bundle_id: String,
        bundle_hash: String,
        reason: String,
    },
    RejectedClaim {
        engine_id: String,
        reason: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComposedPolicyDecision {
    pub decision: PolicyDecision,
    pub safety_outcome: PolicyOutcome,
    pub extension: ExtensionDisposition,
}

/// Composes an optional policy extension without permitting relaxation of the
/// compiled safety result or inflation of enforcement evidence.
#[must_use]
pub fn compose_policy(
    safety: PolicyDecision,
    extension: Option<PolicyExtensionResult>,
) -> ComposedPolicyDecision {
    let safety_outcome = safety.outcome;
    let Some(extension) = extension else {
        return ComposedPolicyDecision {
            decision: safety,
            safety_outcome,
            extension: ExtensionDisposition::NotEvaluated,
        };
    };

    if extension
        .claimed_enforcement
        .is_some_and(|claimed| claimed > safety.enforcement)
    {
        return ComposedPolicyDecision {
            decision: PolicyDecision {
                outcome: PolicyOutcome::Deny,
                reason: ReasonCode::InvalidExtensionClaim,
                ..safety
            },
            safety_outcome,
            extension: ExtensionDisposition::RejectedClaim {
                engine_id: extension.engine_id,
                reason: "extension attempted to increase enforcement evidence".into(),
            },
        };
    }

    let effective =
        RestrictionLevel::from(safety.outcome).max(RestrictionLevel::from(extension.outcome));
    ComposedPolicyDecision {
        decision: PolicyDecision {
            outcome: effective.into(),
            reason: if effective > RestrictionLevel::from(safety.outcome) {
                ReasonCode::ExtensionRestricted
            } else {
                safety.reason
            },
            ..safety
        },
        safety_outcome,
        extension: ExtensionDisposition::Applied {
            engine_id: extension.engine_id,
            engine_version: extension.engine_version,
            bundle_id: extension.bundle_id,
            bundle_hash: extension.bundle_hash,
            reason: extension.reason,
        },
    }
}
