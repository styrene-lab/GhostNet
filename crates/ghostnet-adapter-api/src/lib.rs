//! Versioned capability vocabulary for supervised external bearer adapters.

/// Adapter API version implemented by this crate.
pub const ADAPTER_API_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::ADAPTER_API_VERSION;

    #[test]
    fn adapter_api_is_explicitly_versioned() {
        assert_eq!(ADAPTER_API_VERSION, 1);
    }
}
