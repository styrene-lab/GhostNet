//! Versioned capability vocabulary and supervision port for external bearers.

use std::collections::BTreeMap;
use std::sync::Mutex;

use async_trait::async_trait;
use thiserror::Error;

/// Adapter API version implemented by this crate.
pub const ADAPTER_API_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcknowledgementModel {
    None,
    Transport,
    Custody,
    Operator,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterCapability {
    pub adapter_id: String,
    pub api_version: u32,
    pub bearer: String,
    pub maximum_payload_bytes: usize,
    pub acknowledgement: AcknowledgementModel,
    pub can_transmit: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum AdapterError {
    #[error("adapter is unavailable: {0}")]
    Unavailable(String),
    #[error("adapter request is invalid: {0}")]
    InvalidRequest(String),
}

/// Discovers and supervises versioned external bearer sidecars.
#[async_trait]
pub trait AdapterSupervisorPort: Send + Sync {
    async fn capabilities(&self) -> Result<Vec<AdapterCapability>, AdapterError>;
    async fn capability(&self, adapter_id: &str) -> Result<AdapterCapability, AdapterError>;
}

/// Deterministic adapter registry for application tests.
#[derive(Debug, Default)]
pub struct FakeAdapterSupervisor {
    adapters: Mutex<BTreeMap<String, AdapterCapability>>,
}

impl FakeAdapterSupervisor {
    /// Adds or replaces one fake adapter capability.
    ///
    /// # Panics
    ///
    /// Panics if an earlier fake operation poisoned the internal test lock.
    pub fn upsert(&self, capability: AdapterCapability) {
        self.adapters
            .lock()
            .expect("fake adapter lock poisoned")
            .insert(capability.adapter_id.clone(), capability);
    }
}

#[async_trait]
impl AdapterSupervisorPort for FakeAdapterSupervisor {
    async fn capabilities(&self) -> Result<Vec<AdapterCapability>, AdapterError> {
        Ok(self
            .adapters
            .lock()
            .expect("fake adapter lock poisoned")
            .values()
            .cloned()
            .collect())
    }

    async fn capability(&self, adapter_id: &str) -> Result<AdapterCapability, AdapterError> {
        self.adapters
            .lock()
            .expect("fake adapter lock poisoned")
            .get(adapter_id)
            .cloned()
            .ok_or_else(|| AdapterError::Unavailable(adapter_id.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ADAPTER_API_VERSION, AcknowledgementModel, AdapterCapability, AdapterSupervisorPort,
        FakeAdapterSupervisor,
    };

    #[test]
    fn adapter_api_is_explicitly_versioned() {
        assert_eq!(ADAPTER_API_VERSION, 1);
    }

    #[tokio::test]
    async fn fake_supervisor_returns_stable_adapter_order() {
        let fake = FakeAdapterSupervisor::default();
        for adapter_id in ["zulu", "alpha"] {
            fake.upsert(AdapterCapability {
                adapter_id: adapter_id.into(),
                api_version: ADAPTER_API_VERSION,
                bearer: "test".into(),
                maximum_payload_bytes: 160,
                acknowledgement: AcknowledgementModel::None,
                can_transmit: false,
            });
        }

        let capabilities = fake.capabilities().await.expect("capabilities");
        assert_eq!(capabilities[0].adapter_id, "alpha");
        assert_eq!(capabilities[1].adapter_id, "zulu");
    }
}
