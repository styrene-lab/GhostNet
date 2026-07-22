//! Public contract types for the `GhostNet`–Styrene boundary.

use std::collections::BTreeMap;

/// Reports whether a negotiated substrate capability can be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityState {
    Supported,
    Unsupported,
    Disabled,
    Unauthorized,
}

/// Capabilities and limits observed for the current authorization context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilitySnapshot {
    pub contract_version: u32,
    pub generated_at_ms: u64,
    pub identity: CapabilityState,
    pub detached_signing: CapabilityState,
    pub topics: CapabilityState,
    pub maximum_publication_bytes: usize,
}

/// An opaque Styrene-owned identity reference.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IdentityRef(pub String);

/// A topic path built from fixed `GhostNet` prefixes and validated identifiers.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TopicPath(pub String);

/// Stable application identity used to make publication retries idempotent.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ArtifactKey {
    pub artifact_id: String,
    pub body_hash: String,
}

/// Request to publish canonical artifact bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishRequest {
    pub topic: TopicPath,
    pub artifact: ArtifactKey,
    pub body: Vec<u8>,
    pub correlation_id: String,
}

/// Evidence returned after the substrate accepts a publication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishReceipt {
    pub event_id: String,
    pub artifact: ArtifactKey,
    pub accepted_at_ms: u64,
    pub duplicate: bool,
}

/// A retained public event. The cursor remains opaque to `GhostNet`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactEvent {
    pub event_id: String,
    pub artifact: ArtifactKey,
    pub body: Vec<u8>,
}

/// Request for retained events after an optional opaque cursor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PollRequest {
    pub topic: TopicPath,
    pub after: Option<String>,
    pub limit: usize,
}

/// A page of retained events and the cursor for its final event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactPage {
    pub events: Vec<ArtifactEvent>,
    pub next_cursor: Option<String>,
    pub retention_gap: bool,
}

/// Request for daemon-managed detached signing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignRequest {
    pub purpose: String,
    pub payload: Vec<u8>,
    pub expected_identity: Option<IdentityRef>,
    pub correlation_id: String,
}

/// Detached signature metadata; no private key material crosses this boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetachedSignature {
    pub identity: IdentityRef,
    pub algorithm: String,
    pub signature: Vec<u8>,
    pub signed_payload_hash: String,
}

/// Deterministic in-memory topic state used by the fake implementation.
pub(crate) type TopicEvents = BTreeMap<TopicPath, Vec<ArtifactEvent>>;
