# GhostNet–Styrene Implementation Plan

> **Workspace authority:** Every task in this plan is constrained by [`../WORKSPACE-CHARTER.md`](../WORKSPACE-CHARTER.md). A task that would move GhostNet operational ownership into Styrene proper must stop for an explicit charter amendment.

## Status

**Implementation plan constrained by the policy/client-overlay architecture.** This plan supersedes any earlier implication that GhostNet domain models, SQLite repositories, or RPC services should be added directly to `styrened`.

Normative design inputs:

- [`ghostnet-policy-client-overlay.md`](ghostnet-policy-client-overlay.md)
- [`ghostnet-doctrine-kernel.md`](ghostnet-doctrine-kernel.md)
- [`styrene-integration-contract.md`](styrene-integration-contract.md)
- [`integration-friction-register.md`](integration-friction-register.md)
- `references/GhostNet-v1.5.txt`

## 1. Strategic objective

Implement the useful GhostNet doctrine—prepared nets, scheduled windows, incident activation, concise signed reports, multiple paths, receive-only participation, store-and-forward, disclosure discipline, and graceful degradation—as an independent GhostNet product using Styrene's public communications substrate.

GhostNet is not a second mesh daemon. Styrene proper remains responsible for generic secure communications. GhostNet adds operational semantics and workflows.

## 2. Workstream boundaries

### GhostNet repository

Owns:

- doctrine and policy kernel;
- canonical artifact schemas;
- client orchestration;
- local drafts/cursors/views/audit;
- public Styrene adapter;
- operator surfaces;
- external radio-application sidecars;
- simulation and exercise tooling.

### Styrene proper

May receive narrowly scoped generic proposals for:

- detached identity signing;
- generic outbound transmission-policy enforcement;
- clarified topic retention/idempotency contract if required.

It does not receive GhostNet nets, incidents, reports, windows, check-ins, adapters, tables, or UI workflows.

## 3. Phase 0 — boundary reset and contract fixtures

### Goal

Make ownership and interoperability executable before product behavior.

### Tasks

1. Mark the exploratory `feat/ghostnet-netops` Styrene branch as non-mergeable research.
2. Create a GhostNet Rust workspace and initial crates:
   - `ghostnet-doctrine`;
   - `ghostnet-styrene`;
   - `ghostnet-store`;
   - `ghostnet-adapter-api`;
   - `ghostnet-cli`.
3. Add architecture tests that reject production imports from `styrened`, `styrene-rns` internal identity types, `rusqlite` in doctrine, and private-key types.
4. Commit JSON Schemas for:
   - net definition v1;
   - incident transition v1;
   - operational report v1;
   - detached signed envelope v1;
   - projection v1;
   - adapter capability v1.
5. Implement the RFC 8785 restricted canonical encoder and SHA-256 body references.
6. Add language-neutral golden vectors with positive and negative signature cases.
7. Define `StyrenePort`, `Clock`, `LocalStorePort`, and `AdapterSupervisorPort`.
8. Implement `FakeStyrenePort` with injected duplicates, reorder, timeout, denials, and retention gaps.
9. Produce two Styrene upstream proposals, without implementation coupling:
   - detached identity signing;
   - generic transmission-policy enforcement.
10. Add CI checks for Markdown, JSON Schema, Rust formatting, clippy, tests, and dependency boundaries.

### Exit criteria

- GhostNet builds without a Styrene checkout.
- Doctrine has no platform or secret dependencies.
- Golden canonical bytes are stable.
- Fake client conformance suite passes.
- No GhostNet change is pending merge in Styrene proper.

## 4. Phase 1 — doctrine kernel

### Goal

Implement deterministic prepared-net and incident semantics independent of transport.

### Tasks

#### 4.1 Net definitions

- IDs, revisions, predecessor hashes, effective/expiry times;
- role bindings;
- window/profile references;
- validation limits;
- fork detection and explicit conflict state;
- signed bootstrap and revision authorization policy.

#### 4.2 Communication windows

- one-shot UTC windows;
- restricted UTC recurrence grammar;
- next/current occurrence calculation;
- check-in lead periods;
- explicit clock-quality and skew handling;
- property tests across week/year boundaries.

#### 4.3 Incidents

- immutable transition artifacts;
- planned/active/monitoring/closed/cancelled lifecycle;
- terminal-state behavior;
- predecessor hash chain;
- duplicate idempotency;
- divergent transition detection;
- new-incident linkage for follow-on events.

#### 4.4 Reports

- report kinds, confidence, priorities, TTL;
- area references and disclosure class;
- attachment references;
- source assertions;
- correction/retraction semantics;
- validation and resource limits;
- canonical signature input generation.

#### 4.5 Disclosure and degradation

- audience profiles;
- geographic precision reduction;
- details/attachment/source-chain omission;
- complete, text-only, summary, compact-code projections;
- explicit omission metadata;
- projection size proofs.

#### 4.6 Posture and decisions

- emission intents;
- routine/elevated/restricted/silent profiles;
- confirmation requirements;
- policy outcome/reason codes;
- enforcement-level semantics;
- deterministic decision audit fields.

### Test scenarios

- prepared weekly readiness net;
- ad-hoc incident activation;
- receive-only observer;
- expired alert;
- corrected location report;
- conflicting net revisions;
- divergent incident transitions;
- confidential report on public bearer;
- clock uncertainty during a window;
- projection onto a 160-byte text bearer.

### Exit criteria

- all normative invariants in `ghostnet-doctrine-kernel.md` have tests;
- model-based event-chain tests pass;
- fuzz targets enforce bounded allocation and no panics;
- canonical golden vectors do not depend on Rust struct field order.

## 5. Phase 2 — client application and local views

### Goal

Create usable workflows against `FakeStyrenePort` while preserving substrate ownership.

### Tasks

1. Implement command handlers:
   - install/remove net;
   - list/show net revisions;
   - calculate next window;
   - open/monitor/reactivate/close/cancel incident;
   - draft/sign/publish report;
   - correct/retract report;
   - check in;
   - set desired posture;
   - inspect policy decision/audit.
2. Implement application-local storage for:
   - net installation preferences;
   - drafts;
   - pending confirmations;
   - subscription cursors;
   - projection event log;
   - rebuildable materialized views;
   - policy audit records;
   - adapter configuration.
3. Ensure no tables duplicate raw Styrene message bodies, native receipts, propagation queue, identity secrets, RBAC roster, or interface/path state.
4. Implement atomic cursor-and-view commits.
5. Add full rebuild from fake retained events.
6. Implement status labels that distinguish requested, accepted, queued, sent, protocol-acknowledged, operator-confirmed, expired, failed, and unknown.
7. Build initial CLI commands and JSON output for automation.
8. Add an exercise runner that consumes scripted capability/outage schedules.

### Exit criteria

- a two-client fake exercise installs a net, activates an incident, publishes reports, corrects one, partitions, reconnects, deduplicates, and converges;
- retention gaps visibly produce incomplete/indeterminate state;
- publication unknown-outcome path does not duplicate artifacts;
- local views rebuild from the fake public event history;
- private-key types remain absent.

## 6. Phase 3 — real public Styrene adapter

### Goal

Replace the fake with a capability-negotiated adapter over supported Styrene SDK/RPC.

### Tasks

1. Implement connection and capability handshake.
2. Map fixed GhostNet topic paths to topic create/list/publish/poll APIs.
3. Map identity lookup to opaque `IdentityRef`.
4. Implement detached signing only when the public capability exists.
5. Implement publish idempotency and correlation IDs.
6. Implement polling cursors and retention-gap handling.
7. Map attachment upload/download and immutable references.
8. Map marker projections and optional telemetry observations.
9. Normalize receipts without overstating delivery.
10. Run the same conformance suite against fake and real adapter.
11. Test local and authenticated remote authorization contexts.
12. Publish a supported Styrene version/capability matrix.

### Degradation rules

- no detached signing: drafts and read-only verification may work; signed publication is disabled;
- no topics: core net synchronization is unavailable; do not fall back silently to chat;
- no markers: reports work, map view is disabled;
- no attachments: text-only reports remain available;
- no transmission policy: maximum silence claim is `client_guarded`;
- insufficient retention: derived state is explicitly incomplete.

### Exit criteria

- real adapter passes conformance suite;
- daemon-managed identity signs and verifies golden bytes;
- no path dependency on the Styrene source repository in release builds;
- no daemon-private imports or database access;
- report and control artifacts round-trip byte-for-byte.

## 7. Phase 4 — generic transmission enforcement

### Goal

Support truthful daemon-enforced receive-only and restricted operation when Styrene provides the generic capability.

### Styrene upstream scope

A separate Styrene change implements:

- generic transmission classes;
- one shared outbound enforcement gate;
- policy apply/status commands;
- operator confirmation to relax receive-only;
- explicit exceptions and enforcement scope;
- audit events;
- end-to-end tests for automatic and application traffic.

The patch contains no GhostNet artifact or workflow types.

### GhostNet tasks

1. Map doctrine posture requirements to generic Styrene policy requests.
2. Compare requested and achieved enforcement.
3. Refuse `daemon_enforced` label if exceptions exist.
4. Refresh capability/policy state before every consequential send.
5. Record applied policy ID and correlation in audit.
6. Require explicit confirmation when leaving silent posture.
7. Provide client-guarded fallback only with prominent labeling.

### Test matrix

- application payload;
- ACK/proof;
- announce/discovery;
- heartbeat;
- relay/forward;
- propagation sync;
- path request/response;
- policy expiration;
- daemon restart;
- second client attempting transmission;
- interface added after policy application;
- hardware receive-only evidence.

### Exit criteria

- blocked classes emit zero frames in an instrumented transport test;
- every active path is covered or reported as an exception;
- UI/API report achieved enforcement accurately;
- relaxing receive-only is impossible without required authority and confirmation.

## 8. Phase 5 — operator surfaces

### Goal

Deliver the doctrine as understandable operational workflows.

### CLI

```text
ghostnet net install|list|show|remove
ghostnet window next|status|preflight
ghostnet incident open|status|monitor|reactivate|close|cancel
ghostnet report draft|sign|publish|list|show|correct|retract
ghostnet check-in
ghostnet posture show|set|verify
ghostnet sync status|rebuild
ghostnet audit list|export
ghostnet capabilities
```

### UI views

- next window and countdown;
- clock quality;
- installed net revision/fork state;
- incident lifecycle and participating nets;
- priority reports with author, observation time, receipt time, confidence, and provenance;
- draft/pending/queued status;
- current desired and achieved posture;
- incomplete-history warning;
- adapter health and custody evidence;
- attachment fetch cost/size warning.

### Accessibility and constrained devices

- keyboard-only operation;
- text-first rendering;
- no color-only severity distinction;
- compact 80×24 layout;
- bounded pagination;
- no hidden automatic transmissions triggered by opening a view.

### Exit criteria

- every transmission has a visible policy decision and confirmation path;
- no UI wording promotes protocol ACK to human confirmation;
- receive-only enforcement level is always visible;
- all commands provide stable machine-readable output.

## 9. Phase 6 — offline bulletins and operational picture

### Goal

Turn signed reports into durable, low-bandwidth information products.

### Tasks

1. Generate a signed incident summary manifest from selected authoritative artifacts.
2. Render a static incident landing page without executable content.
3. Publish bulletin manifests before optional content chunks.
4. Reference Kiwix/Qdrant/local knowledge as reference sources distinct from live reports.
5. Project geospatial reports to markers with disclosure-reduced precision.
6. Add optional Cursor-on-Target import/export as an external adapter, not a doctrine primitive.
7. Produce printable/QR hand-carry envelopes referencing signed source artifacts.
8. Support checkpoints with event-range and root-hash integrity.

### Exit criteria

- disconnected client can verify incident summary provenance;
- reference content is visually and structurally distinct from live operational evidence;
- marker changes do not alter source reports;
- checkpoint omissions/tampering are detected.

## 10. Phase 7 — adapter framework

### Goal

Bridge non-native bearers without embedding modem/link-specific behavior in Styrene or doctrine.

### Sidecar protocol

Every adapter declares:

- name/version/protocol version;
- receive/transmit capability;
- addressed/broadcast semantics;
- ACK model;
- store-forward capability;
- payload and throughput limits;
- half-duplex and operator-tuning requirements;
- clock requirements;
- confidentiality class;
- configured legal-profile reference;
- health and achieved mode.

Security controls:

- process argument arrays, no shell interpolation;
- bounded message/frame sizes;
- scoped filesystem/socket access;
- no Styrene private keys;
- no direct daemon database access;
- adapter-specific credentials only;
- deadlines, bounded queues, restart limits, and redacted logs;
- inbound text/content treated as untrusted.

### Adapter order

1. **Spool/QR/file hand-carry** — deterministic and safe generic baseline.
2. **Meshtastic receive-only**, then supervised transmit.
3. **JS8Call receive-only**, then supervised compact report transmit.
4. **Winlink P2P** message/file exchange.
5. **FLDigi/RTTY** blind bulletin projection.
6. **ALE** only after hardware-in-loop safety and policy validation.

### Cross-bearer rules

- preserve source artifact ID/hash/signature;
- create explicit gateway projection/attestation;
- never replace author identity;
- deduplicate by source reference;
- bound relay trace;
- enforce TTL and net policy;
- schedule by priority, age, cost, and fit;
- maintain bearer custody separately from Styrene receipts.

### Exit criteria

- a report enters one sidecar, traverses public Styrene topics, exits another, and does not loop;
- crashing/restarting an adapter does not corrupt native state;
- receive-only mode is tested before transmit enablement;
- each adapter's ACK wording matches actual evidence.

## 11. Phase 8 — constrained-link scheduling and exercises

### Goal

Validate useful behavior under real outage and bandwidth conditions.

### Simulator

Configurable:

- MTU/max text length;
- bytes per hour;
- latency/jitter;
- loss/duplication/reorder;
- outage windows;
- half-duplex contention;
- gateway count;
- ACK model;
- clock skew;
- retention depth;
- power/airtime cost.

### Scheduling

Prioritize using:

- emergency/priority/routine class;
- expiry urgency;
- payload fit;
- available confidentiality;
- estimated airtime/energy;
- existing custody evidence;
- quiet windows and TX budget;
- summary-before-detail policy.

Do not use FIFO alone.

### Exercises

1. weekly readiness check-in;
2. natural-disaster incident activation;
3. cross-region partition and delayed bridge;
4. competing gateways;
5. malicious duplicate/replay;
6. false custody claim;
7. clock-skewed participant;
8. receive-only observer;
9. attachment available only after reconnection;
10. net revision fork during partition.

### Exit criteria

- bounded duplicate traffic;
- priority reports beat routine traffic;
- expired traffic does not consume constrained bearer budget;
- all nodes converge or show explicit fork/incomplete state;
- exercise output produces auditable, signed evidence bundles.

## 12. Upstream Styrene proposal plan

### Proposal A — detached identity signing

Expected files are determined by Styrene ownership at proposal time, likely public identity trait/types, daemon facade/RPC handler, identity service, authorization registry, and tests. No GhostNet dependency or schema is added.

Acceptance scenarios:

- authorized caller signs bounded bytes;
- private key never leaves daemon;
- expected-identity mismatch rejects;
- oversize payload rejects;
- unauthorized caller rejects;
- purpose/hash/correlation are audited;
- verification succeeds through public API;
- local and remote command surfaces share semantics.

### Proposal B — generic transmission policy

Expected files are determined by Styrene transport/policy ownership. No GhostNet dependency or posture enum is added; Styrene defines generic classes/modes.

Acceptance scenarios:

- receive-only covers every active outbound class;
- uncovered interface/path rejects or reports exception;
- blocked attempt creates audit event;
- policy survives/discontinues across restart according to contract;
- expiry restores only declared prior/default state;
- relaxing requires authorization/confirmation;
- second client cannot bypass;
- status reports exact effective scope.

### Proposal C — topic contract clarification, only if needed

Before proposing code, determine and document:

- publication idempotency support;
- cursor ordering;
- retention gap signaling;
- payload limits;
- event immutability;
- query-by-correlation or artifact key.

Prefer contract documentation/tests over new APIs when existing behavior suffices.

## 13. Definition of done

GhostNet v1 is done when:

- doctrine is independently buildable and fully covered by normative tests;
- a supported Styrene daemon can sign, publish, retain, and return exact artifacts through public APIs;
- net and incident views rebuild from public events within declared retention;
- report corrections/retractions and forks behave deterministically;
- priority and disclosure policy produce bounded safe projections;
- no GhostNet code or state is required inside `styrened`;
- no private key crosses into GhostNet;
- receive-only claims state their actual enforcement level;
- at least the spool adapter and one receive-only live radio adapter pass conformance;
- an end-to-end exercise survives a partition without duplicate storms or silent state corruption;
- all legal/frequency/operator assumptions are deployment configuration, not universal defaults.

## 14. Immediate next increment

Implement **Phase 0 in the GhostNet repository**:

1. create workspace/crate skeleton;
2. define doctrine IDs/envelopes and canonical encoder;
3. commit report/net/incident JSON Schemas;
4. add golden vectors;
5. define `StyrenePort` and `FakeStyrenePort`;
6. add dependency-boundary tests;
7. draft detached-signing and transmission-policy upstream proposals.

Do not continue implementing daemon persistence or GhostNet RPC services on the exploratory Styrene branch.
