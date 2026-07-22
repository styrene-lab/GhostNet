//! `GhostNet` operator command surface.
//!
//! Command handlers will depend on application ports rather than substrate
//! implementation details.

/// Human-readable product name used by command projections.
pub const PRODUCT_NAME: &str = "GhostNet";

#[cfg(test)]
mod tests {
    use super::PRODUCT_NAME;

    #[test]
    fn product_name_is_stable() {
        assert_eq!(PRODUCT_NAME, "GhostNet");
    }
}
