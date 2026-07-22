//! `GhostNet`-owned public integration boundary for Styrene Mesh.
//!
//! Production implementations consume only supported public SDK or RPC
//! contracts. This crate must never import `styrened` internals.

mod contract;
mod fake;

use async_trait::async_trait;
use thiserror::Error;

pub use contract::{
    ArtifactEvent, ArtifactKey, ArtifactPage, CapabilitySnapshot, CapabilityState,
    DetachedSignature, IdentityRef, PollRequest, PublishReceipt, PublishRequest, SignRequest,
    TopicPath,
};
pub use fake::{FakeStyrenePort, Fault};

/// Stable failures exposed to application policy and audit.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum StyreneError {
    #[error("operation timed out with an unknown outcome")]
    Timeout,
    #[error("operation was denied: {0}")]
    Denied(String),
    #[error("capability is unavailable: {0}")]
    CapabilityUnavailable(&'static str),
    #[error("request is invalid: {0}")]
    InvalidRequest(String),
}

/// The narrow subset of Styrene behavior required by current `GhostNet` use cases.
#[async_trait]
pub trait StyrenePort: Send + Sync {
    async fn capabilities(&self) -> Result<CapabilitySnapshot, StyreneError>;
    async fn local_identity(&self) -> Result<IdentityRef, StyreneError>;
    async fn sign(&self, request: SignRequest) -> Result<DetachedSignature, StyreneError>;
    async fn publish(&self, request: PublishRequest) -> Result<PublishReceipt, StyreneError>;
    async fn poll(&self, request: PollRequest) -> Result<ArtifactPage, StyreneError>;
}
