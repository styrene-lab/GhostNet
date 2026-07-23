# Styrene Operating Modality Extension Contract

## Status

**Required generic contract.** This document defines what Styrene must expose so GhostNet and other signed Operating Modalities can contribute domain semantics without becoming separate communications applications.

## Contract posture

Styrene owns identity, authorization, unified policy evaluation, messaging, tunnels, destination resolution, final-byte construction, signing, encryption, queues, transport, persistence, and receipts.

An OM extension supplies declarative content and pure domain functions. It never receives private keys, direct transport handles, unrestricted message history, or an application-owned `publish`/`sign`/`tunnel` API.

## Generic contract families

### 1. Bundle installation and activation

```rust
trait OperatingModalityRegistry {
    async fn inspect_bundle(&self, bytes: &[u8]) -> Result<BundleInspection, OmError>;
    async fn install_bundle(&self, bytes: &[u8]) -> Result<InstalledBundle, OmError>;
    async fn activate(&self, request: ActivateOm) -> Result<ActivationEpoch, OmError>;
    async fn deactivate(&self, request: DeactivateOm) -> Result<ActivationEpoch, OmError>;
    async fn effective_configuration(&self) -> Result<EffectiveOm, OmError>;
}
```

Installation validates canonical bytes, publisher signature, trust, schemas, engine compatibility, required capabilities, and scenario vectors. Activation binds deployment configuration and actor-role assignments into an immutable epoch.

### 2. Domain contribution

```rust
trait OperatingModality {
    fn descriptor(&self) -> &OmDescriptor;
    fn schemas(&self) -> &[DomainSchema];
    fn commands(&self) -> &[CommandDescriptor];
    fn views(&self) -> &[ViewDescriptor];
    fn policy_sources(&self) -> &PolicySources;

    fn validate_intent(&self, intent: &DomainIntent) -> Result<ValidatedIntent, DomainError>;
    fn derive_projection(
        &self,
        source: &CanonicalArtifact,
        request: &ProjectionRequest,
    ) -> Result<ProjectionCandidate, DomainError>;
    fn apply_event(
        &self,
        state: &DomainState,
        event: &CanonicalArtifact,
    ) -> Result<DomainState, DomainError>;
}
```

These functions are deterministic over explicit inputs. They do not send, sign, fetch, persist, or consult ambient state.

### 3. Unified policy

Styrene maps validated domain intent into its generic policy request:

```rust
struct PolicyRequest {
    actor: ActorSnapshot,
    action: Action,
    resource: Resource,
    context: PolicyContext,
    active_epoch: EpochRef,
    exact_operation_hash: Option<HashRef>,
}
```

The result is typed:

```rust
enum Outcome {
    Permit,
    RequireConfirmation,
    Indeterminate,
    Deny,
}

struct PolicyDecision {
    outcome: Outcome,
    reason_codes: Vec<ReasonCode>,
    obligations: Vec<Obligation>,
    provenance: DecisionProvenance,
}
```

Cedar is normative. Optional Regorus and bounded helper results compose monotonically. Engines do not return arbitrary executable instructions.

### 4. Sealed egress

All output-producing surfaces submit an intent to one internal Styrene operation pipeline. No OM receives a direct egress method.

```rust
struct SealedOperation {
    operation_hash: HashRef,
    actor: IdentityRef,
    destination: ResolvedDestination,
    membership_version: Option<HashRef>,
    active_epoch: EpochRef,
    policy_snapshot: HashRef,
    exact_bytes: Vec<u8>,
    attachment_hashes: Vec<HashRef>,
    disclosure: DisclosureSet,
    transport_requirements: TransportRequirements,
    permit: SingleUsePermit,
}
```

The permit binds every field. Any mutation invalidates it. Authorization and atomic queue insertion form one logical transaction.

### 5. Commands and projections

OM commands register through the same semantic command registry used by applicable Styrene surfaces. OM views are projections over Styrene-owned communication state plus deterministic domain state. There are no hidden CLI-, TUI-, or desktop-only authorization paths.

## Required invariants

- Only explicit `Permit` can produce a sealed operation.
- `Deny`, `Indeterminate`, timeout, trap, stale facts, or missing capabilities block.
- Destination and membership resolve before final authorization.
- Final observable bytes and metadata are hashed before permit issuance.
- Editing, retargeting, attachment replacement, epoch change, or policy change invalidates confirmation and permit.
- Delayed queue release reauthorizes current posture, membership, transport, and expiry requirements.
- Classification cannot be silently lowered.
- Projections are complete schema-defined derivatives with source hash and omission metadata.
- Every tunnel, chat, attachment, preview, export, telemetry path, automation, and adapter crosses the same gate.
- Audit evidence avoids duplicating sensitive plaintext unless separately authorized.

## Capability negotiation

Styrene exposes support and limits for:

- OM bundle and manifest versions;
- doctrine ABI and schema dialects;
- Cedar, Regorus, and helper ABI versions;
- command/view extension contracts;
- projection and obligation vocabulary;
- exact-byte sealing and queue-release reauthorization;
- adapter sandbox capabilities;
- achieved enforcement evidence.

A required unsupported capability prevents activation. Optional features may degrade only according to signed bundle rules.

## GhostNet mapping

GhostNet contributes:

- net, incident, report, correction, and retraction schemas;
- deterministic doctrine and projection functions;
- OM-specific commands and views;
- Cedar schemas, policies, obligations, and scenario vectors;
- requested posture and disclosure semantics.

Styrene performs:

- actor and role binding;
- destination and membership resolution;
- policy evaluation and explanation;
- exact envelope construction;
- signing and encryption;
- queueing and release;
- messaging, tunnels, routing, adapters, persistence, and receipts.

## Testing contract

A shared conformance suite must prove:

- invalid or untrusted bundles cannot install;
- unsupported required features cannot activate;
- switching OM preserves identity and creates a new epoch;
- policy failures never permit;
- exact-byte mutation, retargeting, and membership races invalidate permits;
- no direct egress path exists outside sealed enqueue;
- delayed release reauthorizes;
- adapters cannot alter bytes or audience;
- duplicate execution is idempotent;
- audit provenance identifies every policy layer and bundle hash.

Fakes demonstrate contract semantics only. They are not evidence of actual transport or hardware enforcement.
