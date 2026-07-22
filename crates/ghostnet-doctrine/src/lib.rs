//! Deterministic `GhostNet` doctrine and policy types.
//!
//! This crate must remain independent of transport, persistence, platform APIs,
//! and private key material.

/// Identifies the initial Phase 0 workspace contract.
pub const CONTRACT_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::CONTRACT_VERSION;

    #[test]
    fn contract_version_starts_at_one() {
        assert_eq!(CONTRACT_VERSION, 1);
    }
}
