# GhostNet Policy/Client Overlay on Styrene

> **Workspace authority:** This architecture implements the binding boundary in [`../WORKSPACE-CHARTER.md`](../WORKSPACE-CHARTER.md). If this document becomes inconsistent with the charter, the charter controls.

## Status

**Proposed architecture and ownership contract.** This document supersedes the exploratory approach of embedding GhostNet domain services and persistence directly in `styrened`.

Companion documents:

- [`ghostnet-doctrine-kernel.md`](ghostnet-doctrine-kernel.md) — executable domain and policy semantics
- [`styrene-integration-contract.md`](styrene-integration-contract.md) — public client boundary and concrete integration points
- [`integration-friction-register.md`](integration-friction-register.md) — risks, gaps, mitigations, and review gates
- [`ghostnet-styrene-integration-plan.md`](ghostnet-styrene-integration-plan.md) — phased delivery plan

## 1. Decision

GhostNet SHALL be implemented as an independent operational policy and client layer over Styrene's public SDK/RPC boundary.

GhostNet SHALL NOT become:

- a second communications daemon;
- a private extension of `styrened::AppContext`;
- an alternate SQLite repository inside `styrened`;
- a new propagation or receipt engine;
- an owner of Reticulum identities or private keys;
- a modem or transport implementation embedded in Styrene;
- a parallel RBAC or peer-blocking authority.

The architectural rule is:

> Styrene provides identity, authorization, secure transport, messaging, topic distribution, persistence, attachments, telemetry, markers, receipts, and generic transmission controls. GhostNet defines prepared-net and incident doctrine, evaluates operational policy, projects typed artifacts onto Styrene capabilities, and presents operator workflows.

## 2. Drivers

### 2.1 Preserve general-purpose substrate ownership

Styrene already has domain and daemon surfaces for topics, telemetry, markers, attachments, messages, propagation, policy, pages, and transport. Embedding GhostNet-specific tables and APIs in `styrened` would create competing ownership of state, lifecycle, and migration behavior.

Verified integration anchors in the current Styrene working repository include:

- topics and publication in `crates/libs/styrene-lxmf/src/sdk/domain.rs:5-38`;
- telemetry queries and points in `crates/libs/styrene-lxmf/src/sdk/domain.rs:62-83`;
- attachments in `crates/libs/styrene-lxmf/src/sdk/domain.rs:90-195`;
- geospatial markers in `crates/libs/styrene-lxmf/src/sdk/domain.rs:197-250`;
- store-and-forward configuration in `crates/libs/styrene-lxmf/src/sdk/types/config.rs`;
- identity signing and verification in `crates/libs/styrene-rns/src/identity.rs:335-340` and `:409-418`;
- centralized daemon persistence in `crates/apps/styrened/src/storage/messages.rs:79-100`;
- SDK-domain snapshot persistence in `crates/apps/styrened/src/storage/messages.rs:83-84` and tests at `:912-927`;
- centralized authorization in `crates/apps/styrened/src/services/policy.rs:1-35`;
- service assembly in `crates/apps/styrened/src/app_context.rs:100-160`.

Line references describe the audited branch at design time and may move. The named types and files are the durable anchors.

### 2.2 Keep doctrine independently testable

The rules for incidents, windows, reports, corrections, disclosure, degradation, and emissions do not require a live daemon. They should be deterministic library behavior with tests and golden vectors.

### 2.3 Keep private keys in Styrene

GhostNet needs signed artifacts, but it does not need direct key custody. Directly accepting `ed25519_dalek::SigningKey` would duplicate identity behavior and create an unsafe secret boundary.

### 2.4 Support more than one client surface

The same doctrine should support a CLI, desktop UI, constrained-device UI, mobile client, and automated exercise harness. It therefore cannot be hidden inside one daemon facade or TUI.

### 2.5 Permit independent release cadence

GhostNet artifact schemas and workflows can evolve without forcing every Styrene node to adopt GhostNet-specific daemon migrations. Compatibility is negotiated at the public artifact and client-contract boundaries.

## 3. System context

```text
┌──────────────────────────────────────────────────────────────────┐
│ GhostNet operator surfaces                                       │
│ CLI · desktop · constrained UI · exercise harness                │
├──────────────────────────────────────────────────────────────────┤
│ GhostNet application orchestration                               │
│ commands · subscriptions · materialized views · confirmations    │
├──────────────────────────────────────────────────────────────────┤
│ GhostNet doctrine kernel                                         │
│ nets · windows · incidents · reports · posture · policy          │
│ canonical bytes · projection · degradation · audit decisions     │
├──────────────────────────────────────────────────────────────────┤
│ GhostNet integration ports                                       │
│ StyrenePort · LocalStorePort · Clock · AdapterSupervisor          │
├──────────────────────────────────────────────────────────────────┤
│ Public Styrene SDK / RPC                                          │
│ identity · RBAC · topics · messages · telemetry · markers         │
│ attachments · receipts · propagation · generic TX policy          │
├──────────────────────────────────────────────────────────────────┤
│ Styrene daemon and transports                                    │
│ storage · RNS/LXMF · TCP · UDP · Serial/KISS · I2P               │
└──────────────────────────────────────────────────────────────────┘
                         │
                         ├── GhostNet JS8Call sidecar
                         ├── GhostNet Meshtastic sidecar
                         ├── GhostNet Winlink sidecar
                         └── GhostNet file/QR hand-carry adapter
```

Dependency direction SHALL point inward toward the doctrine kernel and downward only through declared ports. No GhostNet crate may import `styrened` internals.

## 4. Component responsibilities

### 4.1 `ghostnet-doctrine`

Owns pure domain and policy behavior:

- `NetDefinition` and revisions;
- `NetWindow` and recurrence calculations;
- `Incident` lifecycle;
- operational report body and validation;
- canonical report encoding;
- detached signature envelope shape;
- correction and retraction chains;
- priorities, confidence, TTL, and expiry;
- disclosure profiles;
- emission-intent postures;
- bearer-independent projection plans;
- policy decisions and reason codes.

It SHALL NOT depend on Styrene, SQLite, a UI toolkit, modem APIs, or secret-key libraries.

### 4.2 `ghostnet-styrene`

Implements the application-owned `StyrenePort` using public Styrene SDK/RPC calls. It owns:

- capability discovery and version negotiation;
- topic-path mapping;
- artifact publication and polling;
- identity references and detached signing requests;
- attachment upload/download calls;
- marker and telemetry projections;
- receipt/status normalization;
- generic transmission-policy requests;
- mapping Styrene errors into stable application errors.

It SHALL NOT call `MessagesStore`, `MessagingService`, `MeshTransport`, `PolicyService`, or `AppContext` directly.

### 4.3 `ghostnet-store`

Owns application-local state:

- installed net plans and preferences;
- drafts and pending confirmations;
- subscription cursors;
- adapter configuration;
- policy decision/audit records;
- rebuildable materialized views;
- local exercise evidence.

It SHALL NOT duplicate raw Styrene messages, attachment bodies, propagation queues, delivery receipts, identities, RBAC rosters, interface state, or path tables.

The ownership test is:

> If data answers “what did GhostNet decide, draft, configure, or display?”, GhostNet may own it. If it answers “what did the communications substrate authorize, send, receive, retain, route, or acknowledge?”, Styrene owns it.

### 4.4 `ghostnet-adapter-api`

Defines a sidecar protocol for non-native bearers. It owns:

- capability declarations;
- bounded inbound observations;
- outbound projection requests;
- operator-tuning requirements;
- health and custody evidence;
- process version negotiation.

Adapters receive signed artifacts or permitted projections, never Styrene private keys. Styrene remains unaware of GhostNet-specific radio applications unless a capability later proves sufficiently generic for upstream adoption.

### 4.5 Operator surfaces

Operator surfaces own interaction, not policy. They render policy decisions, ask for confirmations, display enforcement levels, and submit commands. They SHALL NOT duplicate authorization or emission logic in UI conditionals.

## 5. Repository shape

```text
GhostNet/
├── Cargo.toml
├── crates/
│   ├── ghostnet-doctrine/
│   │   ├── src/net.rs
│   │   ├── src/window.rs
│   │   ├── src/incident.rs
│   │   ├── src/report.rs
│   │   ├── src/signature.rs
│   │   ├── src/posture.rs
│   │   ├── src/policy.rs
│   │   └── src/projection.rs
│   ├── ghostnet-styrene/
│   │   ├── src/client.rs
│   │   ├── src/capabilities.rs
│   │   ├── src/topics.rs
│   │   ├── src/signing.rs
│   │   └── src/subscriptions.rs
│   ├── ghostnet-store/
│   ├── ghostnet-adapter-api/
│   └── ghostnet-cli/
├── adapters/
│   ├── spool/
│   ├── meshtastic/
│   └── js8call/
├── schemas/
│   ├── report-v1.schema.json
│   ├── net-definition-v1.schema.json
│   └── adapter-protocol-v1.schema.json
└── tests/
    ├── golden-vectors/
    ├── fake-styrene/
    └── constrained-link/
```

`ghostnet-doctrine` is dependency-free with respect to platform concerns. `ghostnet-styrene` depends on doctrine and the public Styrene client contract. UI crates depend on orchestration interfaces rather than daemon internals.

## 6. Source-of-truth model

### 6.1 Immutable operational artifacts

Net revisions, incident transitions, reports, corrections, and retractions are immutable artifacts. Current state is derived from a valid authorized event sequence.

The artifact body and signature are authoritative. A GhostNet local materialized view is a cache and can be rebuilt.

### 6.2 Styrene as transport record

Styrene is authoritative for:

- accepted publication IDs;
- topic event ordering/cursors as defined by its contract;
- retained payloads;
- attachment IDs and checksums;
- protocol receipts and propagation state;
- identity and authorization decisions;
- applied generic transmission controls.

GhostNet may interpret these events but SHALL preserve the distinction between substrate evidence and operational conclusions.

### 6.3 No mutable daemon row as GhostNet truth

A dedicated `netops_incidents` or `netops_reports` table inside `styrened` would make local daemon state the de facto domain authority and couple every artifact change to daemon migrations. This design rejects that model.

## 7. Artifact projection onto Styrene

Suggested topic paths:

```text
ghostnet/net/<net-id>/control
ghostnet/net/<net-id>/checkins
ghostnet/net/<net-id>/reports
ghostnet/net/<net-id>/requests
ghostnet/net/<net-id>/bulletins

ghostnet/incident/<incident-id>/control
ghostnet/incident/<incident-id>/reports
ghostnet/incident/<incident-id>/requests
```

Topic paths are conventions, not authorization. Every consumer validates:

- artifact type and schema;
- artifact ID and body hash;
- detached signature and signer identity;
- operational authorization for the artifact kind;
- expiry and supersession;
- disclosure projection metadata.

### 7.1 Net definitions

Each revision is a signed artifact on the net control topic. It includes a monotonic revision, prior revision hash, effective time, and policy body. Clients choose the latest valid authorized revision, not merely the latest arrival.

### 7.2 Incident state

State is formed from signed transition events such as `incident_opened`, `incident_entered_monitoring`, `incident_reactivated`, `incident_closed`, and `incident_cancelled`. Invalid transitions remain visible as rejected evidence and do not alter the derived state.

### 7.3 Reports

The report ID is used as the correlation/idempotency key. The same signed report body retains its identity across native Styrene topics, external bearer projections, print, QR, or hand carry.

### 7.4 Markers

A Styrene marker is a view projection, not the signed source report. It references the report ID and body hash and applies the permitted geographic precision. Moving or deleting the marker does not rewrite the report.

### 7.5 Attachments

Attachments are uploaded to Styrene before signing the report. The report body signs the returned attachment ID, checksum, size, and media type. GhostNet controls whether an attachment reference is disclosed or fetched on a given bearer.

### 7.6 Telemetry

Telemetry may represent readiness and equipment observations, but it is not silently promoted to a human operational report. Any promotion is an explicit derived artifact with provenance.

## 8. Trust and authorization composition

Identity, authorization, and truth are separate:

1. A valid signature proves control of the signing identity at signing time.
2. Styrene RBAC determines whether the caller may invoke substrate capabilities.
3. GhostNet operational roles determine whether a signer is authorized to issue a specific net or incident artifact.
4. Neither proves that an observation is factually true.

GhostNet roles may include:

- `observer` — receives and drafts but cannot publish net-control events;
- `participant` — publishes permitted reports and check-ins;
- `coordinator` — publishes authorized window and incident transitions;
- `gateway_operator` — emits bounded custody attestations and bearer projections;
- `net_administrator` — revises membership and policy.

Styrene blocklists and RBAC always remain effective. GhostNet SHALL NOT override a substrate denial.

## 9. Deployment modes

### 9.1 Same-host client

GhostNet connects to a local Styrene daemon over its supported IPC transport. This offers low latency but no permission to import daemon internals.

### 9.2 Remote client

GhostNet uses an authenticated public Styrene endpoint. Capability negotiation and remote authorization are mandatory. Local filesystem assumptions are forbidden.

### 9.3 Constrained client

A reduced UI uses the same doctrine and public contract with bounded page sizes and compact projections. It may omit authoring features but not policy validation.

### 9.4 Receive-only monitor

The client subscribes and maintains materialized views. If Styrene cannot report enforcement of all required transmission controls, the UI labels the state `client_guarded`, not `machine_enforced`.

### 9.5 Disconnected exercise mode

An in-memory or recorded `StyrenePort` fake drives doctrine and UI tests without a daemon. Fake behavior follows the same conformance suite and MUST NOT be used as evidence of actual transport enforcement.

## 10. Failure and recovery behavior

- A lost Styrene connection leaves drafts intact and marks substrate state unknown.
- A publication timeout is not retried blindly; the client first queries by artifact/correlation ID.
- Subscription cursors are committed only after durable local projection.
- A retention gap triggers bounded resynchronization or a visible incomplete-history state.
- Unsupported schema versions are retained as opaque evidence but not interpreted.
- Conflicting authorized net revisions require an explicit deterministic tie-break and visible conflict state.
- Local materialized-view corruption is handled by rebuilding from retained events and configuration.
- Adapter failure cannot corrupt the native Styrene event stream.

## 11. Explicit non-goals

The overlay does not:

- make all Styrene transports legally suitable for every jurisdiction;
- claim factual truth from signatures;
- guarantee end-to-end confidentiality on public or bridged bearers;
- make a local application Boolean equivalent to hardware receive-only;
- hide metadata leakage from callsigns, timing, location, or gateway behavior;
- provide transparent IP tunneling over constrained radio modes;
- grant external adapters access to daemon identity secrets.

## 12. Migration from the exploratory branch

The exploratory branch introduced `styrene-netops` and a `NetOpsStore` in `styrened`. Its findings remain useful as prototypes for validation and schema tests, but its ownership placement is superseded.

Migration rules:

1. Move domain types and tests to `ghostnet-doctrine` in this repository.
2. Replace direct signing-key APIs with canonical-byte and detached-signature APIs.
3. Do not merge `NetOpsStore` into Styrene proper.
4. Re-express persistence tests as GhostNet materialized-view tests or public Styrene integration tests.
5. Propose only generic missing capabilities to Styrene.
6. Keep each upstream proposal independently reviewable and useful beyond GhostNet.

## 13. Acceptance criteria

The architecture boundary is upheld when:

- GhostNet can build and test doctrine without the Styrene source tree;
- GhostNet connects through only public Styrene contracts;
- no GhostNet crate handles a Styrene private key;
- no GhostNet table or service is required inside `styrened`;
- retained Styrene events can rebuild GhostNet materialized views within declared retention limits;
- receive-only enforcement is labeled according to actual substrate capability;
- external bearer adapters can be removed without affecting native Styrene operation;
- generic upstream additions contain no GhostNet-specific artifact or workflow names.
