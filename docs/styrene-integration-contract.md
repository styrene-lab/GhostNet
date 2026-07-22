# Styrene Integration Contract for GhostNet

## Status

**Required public contract and mapping specification.** This document defines how GhostNet consumes Styrene without importing daemon internals or adding GhostNet-owned services to `styrened`.

## 1. Contract posture

GhostNet targets a **capability-negotiated public client interface**. The adapter may use a Rust SDK, generated client, local IPC, or authenticated remote RPC, but the semantics below remain stable.

The application-facing port is owned by GhostNet:

```rust
trait StyrenePort {
    async fn capabilities(&self) -> Result<CapabilitySnapshot, StyreneError>;
    async fn local_identity(&self) -> Result<IdentitySummary, StyreneError>;
    async fn sign(&self, request: SignRequest) -> Result<DetachedSignature, StyreneError>;
    async fn verify(&self, request: VerifyRequest) -> Result<VerificationResult, StyreneError>;

    async fn create_topic(&self, request: CreateTopic) -> Result<TopicSummary, StyreneError>;
    async fn list_topics(&self, request: ListTopics) -> Result<TopicPage, StyreneError>;
    async fn publish(&self, request: PublishArtifact) -> Result<PublishReceipt, StyreneError>;
    async fn poll(&self, request: PollArtifacts) -> Result<ArtifactPage, StyreneError>;

    async fn upload_attachment(
        &self,
        request: UploadAttachment,
    ) -> Result<AttachmentSummary, StyreneError>;
    async fn fetch_attachment(&self, request: FetchAttachment) -> Result<AttachmentData, StyreneError>;

    async fn create_marker(&self, request: CreateMarker) -> Result<MarkerSummary, StyreneError>;
    async fn update_marker(&self, request: UpdateMarker) -> Result<MarkerSummary, StyreneError>;
    async fn delete_marker(&self, request: DeleteMarker) -> Result<(), StyreneError>;

    async fn apply_tx_policy(
        &self,
        request: ApplyTransmissionPolicy,
    ) -> Result<AppliedTransmissionPolicy, StyreneError>;
    async fn tx_policy_status(&self) -> Result<TransmissionPolicyStatus, StyreneError>;
}
```

The real adapter MAY omit methods unsupported by the connected Styrene version, but capability negotiation must expose their absence. GhostNet policy then denies or degrades affected operations.

## 2. Existing Styrene capabilities to reuse

The current Styrene codebase already exposes or models:

| Capability | Verified implementation anchor | GhostNet usage |
|---|---|---|
| Topic create/list/publish/poll | `styrene-lxmf/src/sdk/domain.rs:5-38`; daemon handlers in `styrened/src/rpc/daemon/sdk_topics.rs` | Net, incident, report, request, and bulletin streams |
| Telemetry append/query | `styrene-lxmf/src/sdk/domain.rs:62-83` | Readiness/equipment observations |
| Attachment upload/download/list/delete | `styrene-lxmf/src/sdk/domain.rs:90-195` | Report evidence and bulletin payloads |
| Marker create/list/update/delete | `styrene-lxmf/src/sdk/domain.rs:197-250` | Geospatial report projections |
| Paper message envelopes | `styrene-lxmf/src/sdk/domain.rs:379-419` | Hand-carry/QR export precedent |
| Store-and-forward policy | `styrene-lxmf/src/sdk/types/config.rs` | Intermittent delivery without a GhostNet queue in `styrened` |
| Identity signing/verification internally | `styrene-rns/src/identity.rs:335-340`, `:409-418` | Basis for detached signing proposal |
| Topic state persistence | `styrened/src/storage/messages.rs:83-84`, tests `:912-927` | Avoid duplicate GhostNet daemon tables |
| RBAC and peer blocking | `styrened/src/services/policy.rs:1-35` | Substrate authorization remains authoritative |
| Serial/KISS transport | `styrene-rns/src/transport/iface/serial.rs` | Native RNode-class transport where configured |
| Page serving | `styrened/src/services/pages.rs` | Optional incident landing-page projection |

These are integration anchors, not permission to call internal modules. The public SDK/RPC methods that own these capabilities are the only production boundary.

## 3. Required upstream additions

Two generic capabilities appear necessary. They should be proposed independently to Styrene proper.

### 3.1 Detached identity signing

#### Requirement

Allow an authorized client to request a signature over bounded caller-supplied bytes using the daemon-managed local identity, without exporting private keys.

#### Suggested request

```rust
SignRequest {
    purpose: String,           // e.g. "ghostnet-artifact-v1"
    payload: Vec<u8>,          // bounded, max proposed 64 KiB
    expected_identity: Option<IdentityRef>,
    correlation_id: String,
}
```

#### Suggested response

```rust
DetachedSignature {
    identity: IdentityRef,
    algorithm: String,
    signature: Vec<u8>,
    signed_payload_hash: HashRef,
}
```

#### Security constraints

- daemon key never leaves the identity service;
- permission is distinct from message sending and remote commands;
- payload is size-bounded;
- audit records contain purpose, caller, hash, and correlation ID, not raw sensitive payload by default;
- purpose is domain-separated by the caller and may be allowlisted by policy;
- expected identity prevents accidental signing under a changed profile;
- verification is available without elevated permission;
- IPC and remote RPC use the same command/projection registration path.

#### Why generic

Detached signing supports manifests, configuration attestations, offline envelopes, audit exports, and other applications beyond GhostNet.

### 3.2 Generic transmission-policy enforcement

#### Requirement

Allow an authorized local operator to apply and inspect a generic outbound policy that covers every daemon-originated transmission class within a declared enforcement boundary.

#### Minimum transmission classes

- application payload;
- acknowledgement/proof;
- discovery/announce;
- heartbeat;
- relay/forward;
- propagation synchronization;
- path request/response;
- adapter or interface control frame where applicable.

#### Suggested request

```rust
ApplyTransmissionPolicy {
    policy_id: String,
    mode: TxPolicyMode,             // normal, restricted, receive_only
    allowed_interfaces: Vec<String>,
    allowed_classes: Vec<TxClass>,
    byte_budget: Option<ByteBudget>,
    expires_at_ms: Option<u64>,
    require_operator_confirmation: bool,
    correlation_id: String,
}
```

#### Suggested result

```rust
AppliedTransmissionPolicy {
    policy_id: String,
    applied_at_ms: u64,
    enforcement_scope: Vec<String>,
    blocked_classes: Vec<TxClass>,
    interfaces_covered: Vec<String>,
    exceptions: Vec<EnforcementException>,
    effective_level: EnforcementLevel,
}
```

#### Security constraints

- `receive_only` fails if any active daemon path is outside enforcement scope;
- policy is checked at one shared outbound command/gate, not separately in each UI;
- automatic responses are covered;
- policy survives client disconnect according to explicit expiry semantics;
- leaving receive-only requires authorized operator confirmation;
- audit events are emitted for apply, reject, bypass, expiry, and attempted blocked sends;
- hardware incapable of transmit can be reported distinctly but not inferred from configuration alone.

#### Why generic

This supports maintenance windows, metered links, travel/airplane modes, power conservation, incident response, and regulated deployments beyond GhostNet.

## 4. Capabilities handshake

On connection, GhostNet requests:

```rust
CapabilitySnapshot {
    server_version: String,
    contract_version: u32,
    identity: CapabilityState,
    detached_signing: CapabilityState,
    topics: CapabilityState,
    attachments: CapabilityState,
    markers: CapabilityState,
    telemetry: CapabilityState,
    transmission_policy: Option<TransmissionPolicyCapability>,
    limits: ContractLimits,
    generated_at_ms: u64,
}
```

`CapabilityState` is `supported`, `unsupported`, `disabled`, or `unauthorized`; these states are not interchangeable.

Relevant limits include:

- maximum publication body;
- maximum attachment size;
- topic page size;
- polling cursor retention behavior;
- signing payload limit;
- supported signature/hash algorithms;
- policy enforcement classes;
- active transport/interface identifiers.

The snapshot is hashed into every consequential GhostNet policy audit record.

## 5. Topic mapping

### 5.1 Path construction

Path components use lowercase Crockford IDs and fixed ASCII prefixes. User-controlled names never enter topic paths.

```text
ghostnet/net/{net_id}/control
ghostnet/net/{net_id}/checkins
ghostnet/net/{net_id}/reports
ghostnet/net/{net_id}/requests
ghostnet/net/{net_id}/bulletins
ghostnet/incident/{incident_id}/control
ghostnet/incident/{incident_id}/reports
ghostnet/incident/{incident_id}/requests
```

### 5.2 Publication payload

```rust
GhostNetPublication {
    content_type: "application/vnd.ghostnet.artifact+json;version=1",
    artifact_id: ArtifactId,
    artifact_hash: HashRef,
    schema: SchemaId,
    signed_envelope: Vec<u8>,
    projection_of: Option<ArtifactRef>,
    correlation_id: String,
}
```

If the SDK topic payload is untyped bytes, this object is encoded as JCS JSON. Large details remain in the signed artifact only while larger evidence is placed in attachments.

### 5.3 Idempotency

`artifact_id + artifact_hash` is the application idempotency key. On publication timeout:

1. query/poll for the key;
2. if found with matching hash, treat as accepted;
3. if found with a different hash, report conflict;
4. only then retry with the same correlation ID.

GhostNet never generates a new artifact ID merely because a request timed out.

### 5.4 Polling and cursors

GhostNet stores the last durably projected cursor per topic. Processing order:

1. receive page;
2. validate and verify each artifact;
3. append valid/invalid evidence to the local projection log;
4. update materialized views;
5. atomically commit the new cursor.

If Styrene reports a cursor retention gap, GhostNet marks affected materialized views `history_incomplete`, runs bounded resynchronization when available, and returns `indeterminate` for policy requiring complete history.

## 6. Identity and signatures

### 6.1 Key custody

The production GhostNet adapter SHALL NOT accept or return private key bytes. Signing is an async port call.

### 6.2 Signing flow

1. Kernel validates an unsigned artifact body.
2. Kernel emits canonical JCS body bytes and domain-separated signature input.
3. Application calls `StyrenePort::sign` with purpose and exact bytes.
4. Adapter confirms returned identity equals the intended author/issuer.
5. Application assembles `SignedArtifact`.
6. Adapter or integration test calls `verify` before publication.
7. Consumer verifies again on receipt.

### 6.3 Identity changes

If the local active identity changes between draft and signing, signing fails with `IdentityChanged`. The operator must explicitly re-author the draft. GhostNet does not silently rewrite author identity.

## 7. Authorization

GhostNet performs operational-role checks before requesting a Styrene action. Styrene independently enforces RBAC and peer policy.

Denial precedence:

1. local GhostNet validation/policy denial avoids unnecessary substrate calls;
2. Styrene denial is final even if GhostNet allowed;
3. a successful Styrene call does not retroactively make an invalid GhostNet artifact valid.

Errors preserve both layers:

```rust
ApplicationError::Denied {
    ghostnet_reason: Option<DecisionReason>,
    styrene_reason: Option<StyreneDenial>,
    correlation_id: String,
}
```

## 8. Attachment flow

1. Validate attachment type and size against GhostNet net/disclosure policy.
2. Upload through Styrene.
3. Receive attachment ID/checksum/size/media type.
4. Include that immutable reference in the canonical report.
5. Sign the report.
6. Publish report.

If upload succeeds but publication does not, the application records an unreferenced-upload cleanup candidate. It does not mutate the signed report to point at a replacement silently.

Consumers decide whether to fetch based on policy, bearer cost, size, and media type. Downloaded material is treated as untrusted data and rendered inertly.

## 9. Marker and telemetry projections

Marker/telemetry writes are optional views after authoritative artifact publication.

A marker request includes:

- stable projection ID derived from report ID;
- report ID/hash reference in metadata;
- disclosure-reduced coordinates;
- expiry matching or preceding report expiry;
- no scriptable or executable content.

Projection failure does not invalidate report publication. The UI shows the projection lag/error.

Telemetry append follows the same principle. Telemetry is not the authoritative operational report and cannot overwrite it.

## 10. Status normalization

GhostNet normalizes Styrene outcomes into:

```text
requested
accepted_by_daemon
queued_for_propagation
sent_to_transport
protocol_acknowledged
expired
failed
```

Not every transport supplies every state. Missing states remain `unknown`; they are not inferred. `protocol_acknowledged` means substrate-level acknowledgement, not human reading or factual confirmation.

External adapters may additionally report `accepted_by_bearer`, `heard`, or `operator_confirmed`. These remain separate custody evidence.

## 11. Local store contract

GhostNet local storage may contain:

```text
net_installations
window_preferences
drafts
pending_commands
subscription_cursors
projection_events
materialized_incidents
materialized_reports
policy_audit
adapter_configuration
exercise_evidence
```

It may cache signed envelopes required to rebuild current views, but cache retention is bounded and records the Styrene source event ID. It SHALL NOT implement an alternate propagation queue.

A `rebuild --from-styrene` operation drops rebuildable views while preserving drafts, configuration, cursors as appropriate, and audit evidence.

## 12. Error mapping

The adapter maps substrate errors without losing retry semantics:

| Styrene condition | GhostNet adapter error | Retry behavior |
|---|---|---|
| Invalid request/schema | `ContractViolation` | Never retry unchanged |
| Unauthorized/blocked | `SubstrateDenied` | Operator/policy change required |
| Unsupported method | `CapabilityUnavailable` | Degrade or refuse |
| Timeout before known acceptance | `OutcomeUnknown` | Query idempotency key first |
| Temporary disconnected | `Unavailable` | Bounded backoff |
| Queue full | `Backpressure` | Respect retry-after |
| Cursor expired | `RetentionGap` | Resynchronize/mark incomplete |
| Conflicting artifact ID | `ArtifactConflict` | Never overwrite |
| Policy enforcement exception | `EnforcementIncomplete` | Do not claim daemon-enforced posture |

Logs redact payloads and secrets. Correlation IDs are safe opaque random IDs.

## 13. Fake and conformance suite

`FakeStyrenePort` is an in-memory deterministic implementation for doctrine/application tests. It supports injected:

- capabilities;
- authorization denials;
- timeouts before/after acceptance;
- duplicate deliveries;
- reordered pages;
- retention gaps;
- attachment failures;
- policy enforcement exceptions.

A shared conformance suite runs against both the fake and real adapter. Required cases include:

- publish then poll exact signed bytes;
- timeout followed by idempotency lookup;
- duplicate delivery;
- altered artifact rejection;
- unsupported detached signing;
- identity change during signing;
- marker projection failure after successful report publish;
- receive-only request with incomplete enforcement;
- cursor retention gap;
- blocked signer and blocked publisher distinctions.

The fake cannot satisfy release criteria for actual transmission-policy enforcement; real daemon integration tests are mandatory.

## 14. Upstream contribution gates

A Styrene patch is allowed only if all are true:

1. the capability is generic and named without GhostNet concepts;
2. an existing Styrene service clearly owns it;
3. TUI, CLI, ACP, and remote surfaces can share the same command/projection source;
4. authorization and audit behavior are specified;
5. tests cover negative and failure paths;
6. no duplicate persistence repository is added;
7. GhostNet works against released Styrene versions by capability negotiation or a documented minimum version.

Detached signing belongs to identity. Transmission policy belongs to the transport/policy command plane. GhostNet artifact schemas and net lifecycle do not belong in Styrene.

## 15. Compatibility policy

GhostNet declares:

- minimum Styrene contract version;
- required capabilities for each feature;
- optional capabilities and degradation behavior;
- supported GhostNet artifact schemas.

A compatibility matrix is generated in CI. Connecting to an older daemon is allowed for read-only or reduced features when safe. Missing policy enforcement never silently degrades from `daemon_enforced` to `client_guarded`; it requires an explicit operator-visible mode change.

## 16. Acceptance tests

The boundary is production-ready when:

- GhostNet uses no path dependency on Styrene source in release builds;
- the real adapter passes the same conformance suite as the fake;
- golden canonical bytes are signed and verified through daemon-managed identity;
- artifact publication is idempotent under unknown-outcome timeouts;
- local views rebuild from public retained events;
- marker and attachment projections preserve report references;
- authorization denials are distinguishable from unsupported capabilities;
- receive-only status reports exact achieved enforcement scope;
- no adapter call returns private identity material;
- no GhostNet-specific migration or service exists in `styrened`.
