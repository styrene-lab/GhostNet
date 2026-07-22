# GhostNet–Styrene Integration Friction Register

## Status

Active architecture risk register. Entries are reviewed at each delivery phase and before any upstream Styrene proposal.

Severity:

- **Critical** — could violate safety/security guarantees or force architectural rework;
- **High** — blocks a core workflow or causes ambiguous ownership;
- **Medium** — has a bounded workaround with product cost;
- **Low** — manageable implementation or documentation friction.

## Register

| ID | Friction point | Severity | Evidence/trigger | Architectural response | Verification gate | Owner |
|---|---|---:|---|---|---|---|
| F-01 | Domain overlap with Styrene topics, telemetry, markers, attachments, and storage | Critical | Existing SDK types and daemon persistence already own these substrate records | GhostNet is a client/policy overlay; no `NetOpsStore` or GhostNet service in `styrened` | Dependency scan and architecture test reject imports from `styrened` internals | GhostNet architecture |
| F-02 | Signing API prototype handles `ed25519_dalek::SigningKey` directly | Critical | Exploratory `styrene-netops` API required private key material | Doctrine emits canonical bytes; public Styrene detached-signing API keeps keys daemon-side | Production crates contain no `SigningKey`/private-key type; golden integration vectors pass | GhostNet + Styrene identity owner |
| F-03 | Public detached-signing capability may not exist | High | Internal identity has `sign`, but public IPC contract has identity lookup/set rather than generic detached sign | Submit a narrow generic identity-signing proposal; capability-negotiate; allow unsigned drafts but forbid publication requiring signature | Real adapter signs/verifies golden bytes without key export | Styrene identity owner |
| F-04 | Full receive-only cannot be guaranteed by a GhostNet client | Critical | Client refusal does not stop daemon discovery, acknowledgements, propagation, path traffic, or other clients | Define enforcement levels; upstream generic transmission-policy gate; hardware receive-only remains distinct | Integration test observes zero outbound frames for every declared class and reports uncovered paths | Styrene transport/policy owner |
| F-05 | Topic event model may not guarantee durable append-only history | High | Current SDK topic API has publish/poll but exact ordering, retention, and cursor semantics require contract verification | Treat topic store as public transport record only within advertised guarantees; surface retention gaps; preserve local signed evidence where needed | Conformance tests inject reorder, duplicate, and retention gap | GhostNet integration |
| F-06 | Competing authorized net or incident revisions can fork | High | Decentralized publication and partitions permit divergent children | Hash-chained immutable revisions; never choose latest arrival; explicit conflict/merge artifact | Model-based fork tests | GhostNet doctrine |
| F-07 | Public topic path is not authorization | Critical | Anyone with topic publication capability may target a conventional path | Verify signatures and GhostNet operational role for every artifact; Styrene RBAC remains additional gate | Unauthorized signed publisher cannot change derived state | GhostNet doctrine |
| F-08 | Identity signature proves signer, not truth | High | Operational observations can be mistaken or malicious | Preserve confidence/source chain; corroboration metadata only; no automatic truth score | UI and schema review; conflicting observations retained | GhostNet product |
| F-09 | Signature identity representation may not match raw public key assumptions | High | Styrene identity combines key material and exposes destination/identity hashes | Use opaque `IdentityRef`; verification through public Styrene contract; do not serialize guessed key bytes as identity | Cross-version identity integration tests | GhostNet integration + Styrene identity owner |
| F-10 | Canonical JSON drift across languages | High | Default `serde_json` field order is deterministic only under implementation assumptions, not a cross-language contract | RFC 8785 JCS subset, no floats, schema IDs, golden vectors | Rust and one independent implementation produce identical bytes/hash | GhostNet doctrine |
| F-11 | Application ID and topic event ID may be different | Medium | Substrate may assign its own publication/event identifier | Correlation uses `artifact_id + body_hash`; store both IDs | Timeout/idempotency conformance test | GhostNet integration |
| F-12 | Unknown publication outcome can create duplicate sends | High | Client timeout may occur after daemon acceptance | Query by idempotency key before retry; duplicates are idempotent | Fault injection before/after acceptance | GhostNet integration |
| F-13 | Attachment upload precedes signed report and may orphan content | Medium | Signed report must contain returned immutable attachment metadata | Track cleanup candidates; never silently replace attachment reference | Upload-success/publish-failure test | GhostNet application |
| F-14 | Marker and telemetry records can be mistaken for source truth | Medium | They are mutable/rendering-oriented domain records | Treat as views referencing signed source artifact/hash | Delete/update marker leaves report history unchanged | GhostNet integration |
| F-15 | Store-and-forward queue ownership could be duplicated | Critical | Temptation to add GhostNet custody queue in daemon or app | Styrene owns native queues/receipts; GhostNet stores only command/audit/cursor state; adapter custody remains separate evidence | Schema review rejects duplicate raw message/receipt tables | GhostNet architecture |
| F-16 | External bearer acknowledgements are not semantically uniform | High | JS8Call, Meshtastic, Winlink, RTTY, and ALE differ | Adapter capability declares ack model; normalize evidence without claiming universal delivery | Adapter tests map every state and unknown correctly | Adapter owners |
| F-17 | Bridge loops and duplicate gateways | High | Same artifact can re-enter through multiple bearers/gateways | Stable artifact ID/hash, bounded relay trace, gateway custody evidence, TTL, optional lease protocol later | Multi-gateway simulation bounds duplicates | GhostNet adapter framework |
| F-18 | Bearer confidentiality differs from Styrene confidentiality | Critical | Public RF and third-party applications may expose plaintext/metadata | Disclosure projection and confidentiality capability checks; deny if required confidentiality unavailable | Policy test for confidential report over public bearer | GhostNet doctrine |
| F-19 | Callsign, location, timing, and gateway metadata remain exposed | High | Payload encryption cannot remove RF metadata | Explicit metadata warning, precision reduction, quiet windows, no promise of anonymity | Threat-model review and UI acceptance test | GhostNet security/product |
| F-20 | Radio legality cannot be inferred globally | Critical | Service, mode, power, encryption, and automation rules vary | Policy uses configured regional profile references; no built-in claim that emergency suspends law; operator confirmation where required | No executable universal frequency/legal defaults | GhostNet product/security |
| F-21 | Automatic ALE/radio control has hardware and safety variance | High | Radios and local control software differ; sounding can transmit unexpectedly | Defer; start receive-only; hardware-in-loop and shielded tests before TX | Explicit release gate and adapter capability audit | Adapter owner |
| F-22 | Existing Styrene CLI/TUI command surfaces may diverge | Medium | Adding feature-specific hidden RPC paths would violate shared command/projection architecture | GhostNet owns its UI; generic Styrene additions use shared command registry/projection source | Surface parity tests for any upstream capability | Styrene integration owner |
| F-23 | Local and remote IPC may have different authorization context | High | Same API invoked by local socket vs authenticated remote caller | Capability snapshot includes authorization; no local-trust shortcut in GhostNet | Same denial suite against local and remote adapters | Styrene IPC owner |
| F-24 | Client capability snapshot may become stale | Medium | Interface/policy/identity can change after connection | Snapshot has timestamp/hash; refresh before consequential actions; expected identity and policy IDs on commands | Identity/policy change race tests | GhostNet integration |
| F-25 | Clock uncertainty affects windows, TTL, and signatures | High | Disconnected nodes may drift | Explicit clock state and max skew; transmission decision becomes indeterminate | Skew boundary/property tests | GhostNet doctrine |
| F-26 | Topic retention may be insufficient to rebuild all views | High | Retention policy can evict older control/report events | Net bundles/checkpoints may be published as signed manifests; clients mark incomplete history; define minimum retention for deployments | Rebuild test after retention truncation | GhostNet deployment |
| F-27 | Snapshot/checkpoint can conceal omitted history | High | Compact checkpoint may be accepted without prior events | Checkpoint signs included range/root hash and issuer authority; UI distinguishes checkpoint-derived state | Tampered/partial checkpoint tests | GhostNet doctrine |
| F-28 | Schema extension can break canonical validation | Medium | Unknown required extension semantics | Namespaced required/optional extensions; unknown required means uninterpretable, not invalidly interpreted | Extension compatibility vectors | GhostNet doctrine |
| F-29 | Oversized or malicious imported content can exhaust resources | High | Public/bridged nets are untrusted input | Strict size/depth/count limits before allocation; inert rendering; bounded queues | Fuzz and resource-limit tests | GhostNet security |
| F-30 | Source chain can leak identities or grow unbounded | High | Relay provenance is useful but sensitive | Disclosure controls, bounded chain, hashed/redacted assertions, no implicit full-chain forwarding | Projection privacy and size tests | GhostNet doctrine |
| F-31 | Blocking a peer after receiving signed history | Medium | Local block policy and historical audit conflict | Block future presentation/traffic per policy while retaining cryptographic evidence in restricted audit view | Block/unblock history test | GhostNet product + Styrene policy owner |
| F-32 | Net membership and Styrene RBAC can drift | High | Two authorization layers evolve independently | Effective permission is intersection; reconciliation view shows mismatches; no automatic privilege escalation | Matrix tests for role/RBAC combinations | GhostNet integration |
| F-33 | Adapter process compromise could forge gateway metadata | High | Sidecars parse untrusted application output and may request transmissions | Process sandboxing, least privilege, signed source artifacts, gateway attestations distinct from authorship, bounded protocol | Adversarial adapter tests | Adapter supervisor |
| F-34 | Adapter API version skew | Medium | Sidecars and client may upgrade independently | Version/capability handshake and strict field limits; refuse incompatible required features | Cross-version conformance matrix | Adapter framework |
| F-35 | Application-local audit records are mutable | Medium | SQLite/file owner can edit local evidence | Hash-chain records and optionally export a detached signed exercise bundle; do not claim tamper-proof local storage | Audit tamper-detection test | GhostNet application |
| F-36 | Receipt wording can overstate delivery | High | Protocol ACK is not human receipt or factual validation | Maintain distinct normalized states; UI vocabulary reviewed | Product tests forbid “read/confirmed” from protocol ACK | GhostNet product |
| F-37 | `StyrenePort` could become a shadow of the whole Styrene SDK | Medium | Overbroad abstraction increases maintenance and obscures semantics | Include only use-case-required methods; preserve Styrene IDs/errors; split ports if cohesion falls | API review each phase | GhostNet architecture |
| F-38 | Generic Styrene changes could still be driven by hidden GhostNet assumptions | High | Names can be generic while semantics remain special-purpose | Require independent non-GhostNet use case and upstream ownership review | ADR for each upstream proposal | Both architecture owners |
| F-39 | Exploratory branch may be accidentally merged | High | Branch contains `styrene-netops` and `styrened::NetOpsStore` prototypes | Mark branch superseded; do not merge; preserve only as research or selectively transplant tests | Merge-base/status check before PR creation | Operator/repository owner |
| F-40 | Exact SDK contract may change while GhostNet develops | Medium | Styrene proper is active work | Pin minimum contract version, capability negotiate, CI against supported release matrix | Compatibility CI | GhostNet integration |

## Detailed friction notes

### A. Persistence and event semantics

The largest overlap risk is not code duplication alone; it is **competing truth models**. Styrene already persists SDK-domain snapshots and message/attachment state. A dedicated GhostNet daemon repository would require atomic coordination with topic publication and authorization, creating dual-write failure modes:

- GhostNet row commits but topic publication fails;
- publication succeeds but GhostNet row rolls back;
- replays update one side but not the other;
- migration/version skew causes conflicting derived state.

The chosen model removes the dual write. Immutable signed artifacts are published through Styrene. GhostNet's local view is rebuilt from those artifacts and clearly records retention limitations.

### B. Detached signing

The exploratory implementation proved canonical bytes can be signed and validated, but accepting a `SigningKey` is the wrong production API. The key boundary must be daemon-owned. The principal friction is permission granularity: unrestricted arbitrary signing can be abused as a signing oracle.

Mitigations:

- bounded payload;
- explicit purpose/domain separation;
- caller authorization;
- expected identity;
- hash-based audit;
- no private-key export;
- optional purpose allowlist;
- rate limiting.

The design does not require Styrene to understand GhostNet schemas.

### C. Receive-only claims

“Silent” has four different meanings:

1. GhostNet does not request outbound application messages.
2. Styrene blocks application payloads but may send control traffic.
3. Styrene blocks every covered outbound frame.
4. Hardware cannot transmit.

Conflating them is dangerous. Every status and audit record therefore carries an enforcement level and exceptions. Until the generic Styrene gate exists and has complete test coverage, the maximum software claim is `client_guarded`.

### D. Decentralized state

Topics distribute artifacts but do not remove distributed-systems problems. Partitions permit:

- duplicate events;
- reordered events;
- concurrent net revisions;
- missing predecessors;
- retention gaps;
- clocks that disagree.

The doctrine kernel addresses these with immutable IDs/hashes, predecessor chains, fork states, explicit time quality, idempotency, and `indeterminate` decisions. It does not choose whichever event arrived last.

### E. External bearer integration

Existing radio applications are not byte-equivalent interfaces. Treating each as a supervised bearer prevents assumptions from leaking into Styrene. The adapter protocol must carry capability facts and evidence rather than claim a uniform transport abstraction.

Examples:

- JS8Call may expose text messages and application-level callbacks but operator tuning and HF timing constrain automation.
- Meshtastic offers channels, nodes, and acknowledgements with payload limits unlike LXMF.
- Winlink P2P is message/file exchange, not a transparent stream.
- RTTY blind transmission supplies no delivery guarantee.
- ALE can tune and sound automatically and therefore needs especially strict operator and hardware controls.

## Review checklist

Before implementing a GhostNet feature, answer:

1. Does Styrene already own the underlying identity, message, topic, receipt, attachment, marker, telemetry, policy, queue, or transport state?
2. Can the feature be implemented through a public capability?
3. Is local GhostNet state a draft/configuration/audit/view rather than substrate truth?
4. Does the doctrine rule remain testable without Styrene?
5. Does any operation require private-key access? If yes, redesign around detached signing.
6. What exact enforcement level can be claimed?
7. What happens on duplicate, reorder, timeout, partition, fork, clock skew, and retention gap?
8. Is the upstream change generic and independently useful?
9. Are radio/legal assumptions configuration rather than defaults?
10. Is every delivery/confirmation label evidence-accurate?

## Phase gates

### Gate 0 — architecture

- F-01, F-02, F-15, and F-39 resolved by repository boundaries.
- No production code merged to Styrene proper.
- Doctrine and Styrene contracts reviewed.

### Gate 1 — doctrine/client MVP

- F-06, F-07, F-10, F-12, F-24, and F-25 covered by tests.
- Fake `StyrenePort` conformance suite passes.
- Materialized view rebuild works.

### Gate 2 — real Styrene integration

- Detached signing available or publication remains disabled.
- F-03, F-05, F-09, F-11, F-23, and F-26 verified against a real supported daemon.
- No key export and no daemon-private imports.

### Gate 3 — posture enforcement

- F-04 closed for the exact claimed enforcement boundary.
- Blocked automatic emissions tested end to end.
- UI distinguishes client, daemon, and hardware enforcement.

### Gate 4 — external bearer transmit

- F-16 through F-21 and F-33/F-34 have adapter-specific evidence.
- Receive-only field validation precedes supervised TX.
- Regional/operator configuration reviewed.

## Current disposition

The architecture is safe to proceed **in the GhostNet repository** with:

- doctrine types and canonical encoding;
- golden vectors;
- application-owned ports;
- fake Styrene conformance tests;
- topic-path and projection logic;
- local drafts/cursors/materialized views.

It is not yet safe to claim daemon-enforced receive-only or to publish signed GhostNet artifacts without a public detached-signing capability. The exploratory Styrene branch should not be merged.
