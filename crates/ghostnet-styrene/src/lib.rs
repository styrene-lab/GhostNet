//! `GhostNet`-owned public integration boundary for Styrene Mesh.
//!
//! Production implementations consume only supported public SDK or RPC
//! contracts. This crate must never import `styrened` internals.

/// Reports whether a negotiated substrate capability can be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityState {
    Supported,
    Unsupported,
    Disabled,
    Unauthorized,
}

#[cfg(test)]
mod tests {
    use super::CapabilityState;

    #[test]
    fn unavailable_states_are_not_conflated() {
        assert_ne!(CapabilityState::Unsupported, CapabilityState::Disabled);
        assert_ne!(CapabilityState::Disabled, CapabilityState::Unauthorized);
    }
}
