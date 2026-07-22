//! Deterministic `GhostNet` doctrine and policy types.
//!
//! This crate must remain independent of transport, persistence, platform APIs,
//! and private key material.

mod canonical;
mod clock;
mod incident;
mod net;
mod policy;
mod report;
mod window;

pub use canonical::{CanonicalError, body_hash, canonicalize, signature_input};
pub use clock::{Clock, ClockQuality, ClockReading, FakeClock};
pub use incident::{
    ApplyTransition, IncidentChain, IncidentError, IncidentState, IncidentTransition,
};
pub use net::{ApplyRevision, NetError, NetRevision, NetRevisionChain};
pub use policy::{
    ActorAuthorization, BearerFacts, Confidentiality, DisclosureClass, EmissionRequest,
    EnforcementLevel, PolicyDecision, PolicyOutcome, Posture, PostureDirective, ReasonCode,
    evaluate_emission, select_directive,
};
pub use report::{
    ReportArtifact, ReportError, ReportLedger, ReportOperation, ReportStatus, ReportView,
};
pub use time::{Time as UtcTime, Weekday};
pub use window::{
    NetWindow, OneShotWindow, WeeklyWindow, WindowError, WindowEvaluation, WindowOccurrence,
    utc_date,
};

/// Identifies the initial Phase 0 workspace contract.
pub const CONTRACT_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::{
        CONTRACT_VERSION, CanonicalError, Clock, ClockQuality, FakeClock, body_hash, canonicalize,
        signature_input,
    };

    const SCHEMA: &str = "https://ghostnet.styrene.io/schema/operational-report-v1";

    #[test]
    fn contract_version_starts_at_one() {
        assert_eq!(CONTRACT_VERSION, 1);
    }

    #[test]
    fn canonicalizes_golden_vector() {
        let input: Value = serde_json::from_str(include_str!(
            "../../../test-vectors/canonical-json/operational-report-v1.input.json"
        ))
        .expect("golden input must be JSON");
        let expected = include_bytes!(
            "../../../test-vectors/canonical-json/operational-report-v1.canonical.json"
        );
        let expected = expected.strip_suffix(b"\n").unwrap_or(expected);

        let canonical = canonicalize(&input).expect("golden input must canonicalize");
        assert_eq!(canonical, expected);

        let metadata: Value = serde_json::from_str(include_str!(
            "../../../test-vectors/canonical-json/operational-report-v1.vector.json"
        ))
        .expect("vector metadata must be JSON");
        assert_eq!(body_hash(&canonical), metadata["body_hash"]);
        assert_eq!(
            hex(&signature_input(SCHEMA, &canonical).expect("schema must be valid")),
            metadata["signature_input_hex"]
        );
    }

    #[test]
    fn source_member_order_does_not_change_bytes() {
        let left = json!({"z": 1, "a": {"y": 2, "x": 3}});
        let right = json!({"a": {"x": 3, "y": 2}, "z": 1});
        assert_eq!(canonicalize(&left), canonicalize(&right));
    }

    #[test]
    fn orders_object_names_by_utf16_code_units() {
        let value = json!({"\u{e000}": 1, "\u{10000}": 2});
        assert_eq!(
            canonicalize(&value).expect("integers are valid"),
            "{\"𐀀\":2,\"\":1}".as_bytes()
        );
    }

    #[test]
    fn rejects_values_outside_restricted_profile() {
        assert_eq!(canonicalize(&Value::Null), Err(CanonicalError::Null));
        assert_eq!(canonicalize(&json!(1.5)), Err(CanonicalError::Float));
        assert_eq!(
            canonicalize(&json!(9_007_199_254_740_992_u64)),
            Err(CanonicalError::IntegerRange)
        );
        assert_eq!(
            canonicalize(&json!(-9_007_199_254_740_992_i64)),
            Err(CanonicalError::IntegerRange)
        );
    }

    #[test]
    fn rejects_ambiguous_schema_identifier() {
        assert_eq!(
            signature_input("", b"{}"),
            Err(CanonicalError::InvalidSchema)
        );
        assert_eq!(
            signature_input("schema\0other", b"{}"),
            Err(CanonicalError::InvalidSchema)
        );
    }

    #[test]
    fn fake_clock_advances_deterministically() {
        let clock = FakeClock::new(
            1_000,
            ClockQuality::Uncertain {
                maximum_skew_ms: 25,
            },
        );
        clock.advance(250);
        assert_eq!(clock.now().unix_ms, 1_250);
        assert_eq!(
            clock.now().quality,
            ClockQuality::Uncertain {
                maximum_skew_ms: 25
            }
        );
    }

    fn hex(bytes: &[u8]) -> String {
        use std::fmt::Write as _;

        bytes.iter().fold(String::new(), |mut output, byte| {
            write!(output, "{byte:02x}").expect("writing to a String cannot fail");
            output
        })
    }
}
