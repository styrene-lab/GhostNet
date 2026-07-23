# GhostNet Workspace Charter

## Status

**Canonical and binding for this workspace.** This charter defines the ownership boundary between GhostNet Operating Modalities and Styrene Mesh. Proposed code, schemas, documentation, and upstream changes must conform to it.

If another workspace document conflicts with this charter, this charter controls until an explicit architecture decision amends it.

## Product definition

**Styrene Mesh is the sovereign communications system.** It owns identity, authorization, unified policy evaluation, messaging, tunnels, transport, routing, queueing, persistence, attachments, receipts, and final outbound enforcement.

**GhostNet is a family of signed Operating Modality (OM) profiles within Styrene.** A GhostNet OM contributes operational doctrine, domain artifact semantics, workflows, projections, posture requests, and operator views. It does not create a second program, identity, inbox, policy authority, or communications path.

A portable Styrene identity remains independent of the active OM. An authorized operator can switch among signed profiles such as `NYC_EMS1_FIRE`, `NYC_EMS1_MED`, `NYC_EMS1_PD`, `NYC_EMS1_CIV`, or Standard without changing identity or installing another client.

The durable distinction is:

> **Styrene communicates and enforces. GhostNet describes an operational modality Styrene can activate and apply.**

## Relationship and dependency direction

GhostNet OMs are signed configuration and policy artifacts consumed through Styrene's public mode-extension contracts. They are not installed executable applications and do not wrap Styrene's SDK/RPC API behind a competing application service.

```text
Styrene operator surfaces
        ↓
Styrene commands, policy, messaging, and tunnels
        ↓
Public Operating Modality extension contracts
        ↓
GhostNet OM manifests, doctrine predicates, artifacts, projections, and views
```

Styrene core must not depend on GhostNet-specific concepts. Generic extension contracts must remain useful to non-GhostNet modes.

## Ownership matrix

| Concern | Styrene owns | GhostNet OM contributes | Prohibited overlap |
|---|---|---|---|
| Identity | Key custody, lifecycle, signing, verification, portable actor identity | Opaque identity references and OM role bindings | GhostNet identity or private-key store |
| Authorization and policy | Unified request/decision/obligation contracts, Cedar/Regorus/WASM adapters, trust, activation epochs, final permit/deny | Signed OM policy sources, schemas, scenarios, domain predicates | Separate GhostNet policy authority or bypass of Styrene denial |
| Messaging and tunnels | Compose/send APIs, addressing, membership resolution, final bytes, encryption, publication, polling | Structured operational actions and artifact semantics | GhostNet publication client, second inbox, tunnel, or topic engine |
| Egress safety | One authoritative exact-byte gate for every queue, tunnel, adapter, export, and transport | Disclosure labels, projection candidates, requested posture, OM constraints | Alternate path around the Styrene egress gate |
| Transport and routing | Bearer facts, path selection, routing, retries, queue release, store-and-forward | Requirements and preferences expressed as typed obligations | GhostNet routing, retry, or bearer execution engine |
| Persistence and receipts | Messages, queues, attachments, delivery state, receipts, activation epochs | Draft semantics, rebuildable domain views, OM-local presentation preferences | Shadow message, receipt, attachment, or propagation stores |
| Domain semantics | Generic extension host and artifact carriage | Nets, windows, incidents, reports, corrections, retractions, priorities | GhostNet lifecycle tables or services in Styrene core |
| Projection | Exact-byte construction pipeline and final authorization | Deterministic projection definitions and omission semantics | GhostNet directly selecting/sending around active Styrene policy |
| Posture | Actual enforcement and truthful evidence level | Requested operational posture and domain reason codes | Client preference represented as daemon/hardware enforcement |
| Operator experience | One Styrene application, identity, chat/tunnel surfaces, policy explanation | Mode badge, operational views, structured composers, guided workflows | Separate GhostNet operator-facing application |
| External adapters | Sandboxing, sealed operation permit, execution, audit | Domain projection support and declared adapter requirements | Adapter access to source history, keys, or unrestricted messaging |

## GhostNet OM SHALL contribute

- signed and versioned OM manifests and family profiles;
- net definitions, communication windows, and immutable revisions;
- incident transitions and derived lifecycle semantics;
- operational report, alert, request, correction, and retraction schemas;
- deterministic canonicalization and domain hash/signature-input specifications;
- compiled doctrine predicates and state machines;
- OM-specific Cedar schemas, policies, obligations, and scenario vectors;
- optional reviewed Regorus compatibility policy and bounded helper declarations;
- disclosure labels, safe projection definitions, and omission metadata;
- requested posture, confirmation semantics, and stable domain reason codes;
- operational commands, views, exercises, and authoring guidance.

## GhostNet OM SHALL NOT own

- a standalone messaging, tunnel, topic, or publication client;
- final authorization or a policy engine separate from Styrene;
- Styrene private keys or raw signing-key types;
- final destination resolution or group-membership truth;
- outbound signing, encryption, queueing, retry, routing, or receipts;
- message, attachment, propagation, or transport persistence;
- direct sockets, tunnels, transports, or unrestricted adapter execution;
- a second identity, contact list, inbox, delivery indicator, or operator application;
- claims that a requested or client-visible posture proves machine or RF enforcement.

## Disclosure-safe egress invariants

Every externally observable byte and metadata field must pass one Styrene-owned gate immediately before atomic insertion into an execution queue. The gate covers chat, tunnels, files, attachments, previews, exports, telemetry, delayed delivery, automation, and external adapters.

A permit binds at least:

- actor identity and authorization snapshot;
- active OM bundle, deployment binding, policy hashes, and activation epoch;
- exact resolved destination and membership version;
- source artifact and projection hashes;
- exact outbound bytes and attachment hashes;
- disclosure labels and satisfied obligations;
- required transport/confidentiality facts;
- expiry or single-use nonce.

Mutation of any bound value invalidates the permit. `Deny`, `Indeterminate`, timeout, stale state, missing facts, policy failure, or unsupported capability all block emission. Delayed release is reauthorized under current facts. No production path may reach a queue, tunnel, adapter, socket, or transport without a valid sealed permit.

## Policy composition

The unified Styrene policy substrate composes:

1. compiled safety and domain invariants;
2. Styrene identity and substrate authorization;
3. Cedar policy as the normative embedded engine;
4. optional Regorus/Rego compatibility restrictions;
5. optional bounded WASM helper results;
6. explicit operating facts and typed obligations.

Restriction is monotonic:

```text
Permit < RequireConfirmation < Indeterminate < Deny
```

No later layer may relax an earlier result or inflate enforcement evidence. Policy evaluation has no ambient network, filesystem, process, environment, wall-clock, randomness, private-key, messaging, or tunnel access.

## Local-state ownership test

Before adding durable state, ask:

> Is this a signed OM/domain source artifact or rebuildable presentation state, or is it communications execution state?

GhostNet may define the former. Styrene owns the latter.

Allowed examples:

- OM source manifests and scenario vectors;
- unsigned local form drafts as a generic Styrene UI concern;
- deterministic domain projections and rebuildable views;
- exercise fixtures;
- OM authoring preferences.

Forbidden examples:

- a GhostNet publication workflow ledger;
- duplicate raw message history;
- native receipt or retry queues;
- copied identity secrets;
- parallel attachment storage;
- destination-membership or routing truth;
- shadow policy activation state.

## Generic Styrene extension rule

A change belongs in Styrene only when it is:

1. useful to non-GhostNet modes;
2. owned by an existing generic Styrene subsystem;
3. exposed through a documented public command/projection or extension contract;
4. shared across applicable Styrene surfaces;
5. authorized, audited, bounded, and negatively tested;
6. non-duplicative of existing state;
7. capability-negotiated and versioned;
8. independently reviewable and releasable.

The immediate generic requirement is the **sealed exact-byte egress permit and OM activation contract**, covering messaging, tunnels, delayed delivery, and adapters through one enforcement path.

## Architecture tests

The workspace guard is `scripts/check-architecture-boundary.sh`. Production work fails review if it:

- describes GhostNet as an independent client, application, messaging layer, or policy authority;
- imports daemon-private modules or reaches into sibling Styrene source trees;
- handles private key material;
- implements direct publication, transport, routing, retry, or receipt ownership;
- creates shadow communications or policy persistence;
- exposes a bypass around Styrene's exact-byte egress gate;
- represents client guarding as daemon- or hardware-enforced silence.

## Decision protocol

Any exception requires an architecture decision that:

1. identifies the exact clause being amended;
2. explains why the generic Styrene extension contract is insufficient;
3. defines ownership, migration, compatibility, rollback, and disclosure risk;
4. supplies a non-GhostNet justification for Styrene-core changes;
5. updates this charter and automated guardrails in the same change.

## Review checklist

Every review must answer:

- Does this contribute OM semantics, or duplicate Styrene communications execution?
- Can the user remain in one Styrene application, identity, inbox, and delivery model?
- Does one Styrene policy substrate make the final decision?
- Is authorization bound to exact bytes, destination, epoch, and current facts?
- Can any path reach a queue, tunnel, adapter, socket, or transport without a sealed permit?
- Are unknown, stale, timed-out, trapped, or unsupported states fail-closed?
- Are classification and projection transformations monotonic and auditable?
- Is the displayed enforcement evidence truthful?
- Is every Styrene-core addition generic and useful without GhostNet?

If an answer is unclear, implementation stops and the boundary is resolved before code proceeds.
