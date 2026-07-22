# GhostNet Workspace Charter

## Status

**Canonical and binding for this workspace.** This charter defines what GhostNet is, what Styrene Mesh is, and which project owns each responsibility. Proposed code, schemas, documentation, and upstream contributions must conform to it.

If another workspace document conflicts with this charter, this charter controls until an explicit architecture decision amends it.

## Product definition

**GhostNet is a local-first operational coordination system for degraded communications environments.** It turns available communications paths into prepared, repeatable operations through scheduled nets, incident activation, signed reports, priorities, disclosure policy, constrained-link projections, emission discipline, and operator workflows.

**Styrene Mesh is the general-purpose sovereign communications substrate.** It provides identities, authorization, secure transport, routing, messaging, topic distribution, persistence, attachments, telemetry, markers, receipts, interfaces, and store-and-forward behavior without requiring Internet or cloud infrastructure.

The durable distinction is:

> **Styrene makes sovereign communication possible. GhostNet defines how a group uses that capability coherently before, during, and after disruption.**

Or, stated as an engineering boundary:

> Styrene owns what the communications substrate authorizes, sends, receives, retains, routes, and acknowledges. GhostNet owns what the operational application defines, drafts, decides, projects, schedules, and displays.

## Relationship

GhostNet is an independent client and policy layer over Styrene's public SDK/RPC contracts. Styrene is a preferred substrate, not a source tree into which GhostNet is embedded.

GhostNet may also supervise external bearer adapters such as Meshtastic, JS8Call, Winlink, RTTY, ALE, and file/QR hand carry. Those adapters project GhostNet artifacts; they do not turn GhostNet into a replacement transport stack.

The dependency direction is one-way:

```text
GhostNet operator surfaces
        ↓
GhostNet orchestration and doctrine
        ↓
GhostNet-owned public integration ports
        ↓
Styrene public SDK / RPC contracts
        ↓
Styrene daemon, persistence, RNS/LXMF, and interfaces
```

Styrene proper must not depend on GhostNet.

## Ownership matrix

| Concern | Styrene owns | GhostNet owns | Prohibited overlap |
|---|---|---|---|
| Cryptographic identity | Key custody, identity lifecycle, signing implementation, verification | Opaque identity references and requests to sign canonical artifact bytes | GhostNet private-key storage or direct signing-key APIs |
| Authorization | Substrate RBAC, peer blocking, caller permissions | Operational roles for issuing net/incident artifacts | GhostNet bypassing a Styrene denial or replacing RBAC |
| Transport and routing | RNS/LXMF, paths, links, TCP/UDP/Serial-KISS/I2P interfaces | Bearer selection policy and safe projection plans | GhostNet routing tables, packet forwarding, or native transport forks |
| Messaging and topics | Publication, polling, retention, protocol envelopes, generic events | Topic naming conventions and signed GhostNet payload schemas | GhostNet-specific topic implementation inside `styrened` |
| Persistence | Native messages, topics, attachments, receipts, queues, transport state | Drafts, preferences, cursors, policy audit, adapter config, rebuildable views | GhostNet daemon tables duplicating substrate state |
| Store-and-forward | Native queueing, retries, propagation, protocol receipts | Priority/TTL policy and interpretation of custody evidence | A second native propagation or receipt engine |
| Attachments | Upload, storage, IDs, checksums, download | Disclosure/fetch policy and signed attachment references | Duplicate attachment repository |
| Telemetry and markers | Generic telemetry and geospatial record services | Operational projections that reference signed source artifacts | Treating mutable projections as GhostNet source truth |
| Incidents and nets | No GhostNet-specific domain ownership | Net definitions, windows, incidents, reports, corrections, retractions | GhostNet lifecycle types or migrations in Styrene proper |
| Emission control | Generic daemon transmission classes and enforcement gate | Desired posture, decision policy, confirmations, truthful enforcement label | Client-only controls presented as daemon- or hardware-enforced |
| Radio applications | Generic interfaces where appropriate | Supervised, bearer-specific sidecars | Embedding JS8Call/Winlink/ALE workflows in Styrene core |
| Operator experience | Generic Styrene network/product surfaces | GhostNet readiness, incident, report, posture, and exercise workflows | Hidden per-surface policy implementations |

## GhostNet SHALL own

- net definitions and immutable revisions;
- scheduled and externally activated communication windows;
- incident transition artifacts and derived lifecycle;
- operational reports, alerts, requests, offers, bulletins, corrections, and retractions;
- canonical artifact schemas and signature-input construction;
- operational role evaluation layered over substrate authorization;
- disclosure profiles and constrained-bearer degradation;
- communications posture intent and decision reason codes;
- operator confirmations and decision audit records;
- local drafts, installation preferences, subscription cursors, and rebuildable materialized views;
- exercise fixtures and constrained-link simulations;
- supervised non-native bearer adapters.

## GhostNet SHALL NOT own

- Reticulum/LXMF protocol implementation;
- Styrene private keys or raw signing-key types;
- daemon-internal service assembly;
- Styrene message/topic/attachment databases;
- native propagation queues or receipt state machines;
- transport path discovery or routing;
- generic RBAC or peer blocking;
- a private fork of Styrene IPC;
- modem/DSP implementations inside the doctrine kernel;
- universal frequency, callsign, encryption, or legal defaults;
- claims that a client-side no-send preference guarantees zero RF emission.

## Upstream contribution rule

A change may be proposed to Styrene proper only when every condition below holds:

1. **Generic:** it is useful outside GhostNet and contains no GhostNet artifact or workflow concepts.
2. **Owned:** an existing Styrene subsystem clearly owns the behavior.
3. **Public:** GhostNet will consume it through a documented public SDK/RPC contract.
4. **Unified:** CLI, TUI, ACP, local IPC, and remote RPC can share the same command/projection source where applicable.
5. **Authorized:** permissions, audit behavior, limits, and negative cases are specified.
6. **Non-duplicative:** it does not introduce a second repository for existing substrate state.
7. **Negotiable:** clients discover support and limits through capability negotiation.
8. **Independently reviewable:** the upstream patch can be reviewed and released without requiring GhostNet.

Current plausible generic upstream proposals are limited to:

- bounded detached signing with daemon-managed identity;
- generic outbound transmission-policy enforcement;
- topic idempotency/retention contract clarification if existing behavior is insufficient.

## Local-state ownership test

Before adding a GhostNet table or durable record, ask:

> Does this record describe an application decision/draft/configuration/view, or does it describe substrate communications state?

GhostNet may persist the former. Styrene owns the latter.

Allowed examples:

- unsigned report draft;
- desired posture and operator confirmation;
- installed net preference;
- topic cursor;
- projection/materialized-view cache;
- policy decision audit;
- adapter configuration;
- exercise result.

Forbidden examples:

- duplicate raw Styrene message archive;
- native delivery receipt ledger;
- propagation retry queue;
- copied identity secret;
- parallel attachment blob store;
- interface/path table;
- shadow RBAC membership database.

## Enforcement-level vocabulary

GhostNet must distinguish:

- **Advisory:** a warning or recommendation only.
- **Client-guarded:** GhostNet refuses to request prohibited transmission, but other daemon activity may transmit.
- **Daemon-enforced:** Styrene confirms all declared transmission classes and interfaces are covered by a generic enforcement policy.
- **Hardware receive-only:** transmitting hardware is absent or physically disabled and supporting evidence is available.

No weaker level may be described using language belonging to a stronger level.

## Architecture tests

The workspace guard script is `scripts/check-architecture-boundary.sh`. It must run in CI once code exists and currently validates repository contents directly.

Production GhostNet source must fail review if it:

- imports from `styrened` or daemon-private modules;
- uses direct Styrene source-tree path dependencies;
- introduces private-key/signing-key types outside test fixtures;
- adds a crate or service named `styrene-netops`;
- adds GhostNet-specific database tables to Styrene;
- implements transport/routing internals in doctrine code;
- labels client-only suppression as machine-enforced receive-only.

Test fixtures may use deterministic test keys, but production APIs remain detached-signature based.

## Decision protocol

Any intentional exception requires an architecture decision that:

1. identifies the exact charter clause being amended;
2. states why a public client-layer implementation is insufficient;
3. defines ownership, migration, compatibility, and rollback;
4. includes a non-GhostNet justification for any Styrene upstream change;
5. updates this charter and the automated boundary checks in the same change.

Silently violating the charter is not an acceptable prototype shortcut. Spikes that cross the boundary must live on clearly labeled non-mergeable research branches.

## Current branch disposition

The Styrene exploratory branch `feat/ghostnet-netops` is research only. Its `styrene-netops` crate and `styrened::NetOpsStore` demonstrate useful validation and persistence ideas but violate the final ownership model and must not be merged into Styrene proper.

Useful tests or algorithms may be reimplemented in this GhostNet workspace under the doctrine/client architecture. They must not be transplanted with direct key custody or daemon-specific persistence.

## Review checklist

Every implementation review must answer:

- Is this GhostNet operational semantics or Styrene substrate behavior?
- Is the dependency through a public contract?
- Does GhostNet handle any private key material?
- Is local state a draft/configuration/audit/view rather than substrate truth?
- Does Styrene already own this queue, receipt, message, attachment, marker, telemetry, identity, or authorization record?
- What happens under duplicate delivery, reordering, timeout, partition, fork, clock uncertainty, and retention loss?
- What exact enforcement level is achieved and displayed?
- Is a proposed upstream change generic and useful without GhostNet?
- Can GhostNet build and test without a Styrene source checkout?

If any answer is unclear, implementation stops at the boundary and the ambiguity is resolved in design first.
