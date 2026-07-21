# GhostNet doctrine integration plan for Styrene

## Status

**Planning document.** This plan maps useful concepts from the operator-supplied *GhostNet v1.5* reference into Styrene capabilities. It is not approval to transmit, a frequency plan, or a replacement for jurisdiction-specific radio rules.

Source artifacts:

- `GhostNet_Version_1.5.pdf` — original PDF supplied by the operator
- `references/GhostNet-v1.5.txt` — searchable extraction of the supplied PDF
- `references/GhostNet-v1.5.provenance.md` — provenance and licensing note

## Executive conclusion

GhostNet is primarily **communications doctrine**: prepare before a crisis, use multiple paths, schedule contact windows, support receive-only participants, relay concise reports, degrade gracefully, and control emissions according to risk. It is not a protocol that Styrene should reimplement wholesale.

Styrene already supplies much of the secure digital substrate that GhostNet approximates with separate radio applications:

- Reticulum identities, encrypted links, paths, resources, and TCP/UDP/Serial-KISS interfaces
- LXMF messaging and configurable store-and-forward
- topics, telemetry, geospatial markers, contacts/trust, attachments, and paper-message envelopes
- page serving, content-addressed chunk distribution, fleet control, RBAC, and offline hub services
- TUI, CLI, daemon IPC, and hub/edge deployment surfaces

The useful extension is an **incident/net operations layer plus supervised adapters**, not native implementations of JS8Call, Winlink, RTTY, or ALE. Styrene should remain the identity, message, custody, policy, and operator-workflow plane. Existing radio applications and hardware should remain modem/link specialists behind narrow adapters.

## 1. Doctrine to preserve

The following source concepts are worth implementing as product behavior:

1. **Prepare and exercise before disruption.** A net definition, contacts, transport profiles, and test windows must be installable and testable before an incident.
2. **No single mandatory controller.** Nodes should converge through signed state and replicated reports; a coordinator may facilitate a window but must not be a protocol root.
3. **Multiple communications paths.** A report can move over any suitable active transport and can be relayed across transports without changing its identity.
4. **Scheduled and ad-hoc nets.** Recurring check-in windows coexist with incident-triggered “CrisisNet” sessions.
5. **Receive-only participation.** Monitoring, ingest, and local display are first-class roles that cannot accidentally transmit.
6. **Store, carry, and forward.** Intermittent links are normal. Custody, expiry, retries, and duplicate suppression must be explicit.
7. **Concise, useful information exchange.** Structured SITREPs, alerts, requests, observations, and bulletins should outrank chat and diagnostics on constrained links.
8. **Emission discipline.** A STRADCON-like posture controls discovery, heartbeat, acknowledgements, automatic relays, and permitted transports.
9. **Local autonomy and modular trust.** Public-interest nets, private groups, and trusted-team traffic can coexist without sharing one identity or one policy.
10. **Graceful degradation.** Rich content should become summaries, text-only reports, delayed attachments, or manual hand-carry rather than simply fail.

## 2. Concepts not to copy directly

The source includes operational details that should remain reference material rather than software defaults:

- fixed amateur frequencies and schedules
- shared static channel/encryption keys
- assumptions that emergency conditions automatically remove legal obligations
- anonymous or fabricated callsign guidance
- continuous automatic transmissions without operator and regional policy controls
- one global public group as a trust or truth mechanism

Styrene should also avoid pretending that its packet transport can transparently ride every named mode. JS8Call, Winlink P2P, RTTY, and ALE have different semantics, latency, duty cycles, addressing, and operator requirements. Treat them as **bridged bearers**, not interchangeable byte streams, until an adapter proves otherwise.

## 3. Current Styrene capability map

| GhostNet need | Existing Styrene foundation | Gap |
|---|---|---|
| Decentralized identity and private messaging | Reticulum identity/destinations/links; LXMF messaging | Net-scoped roles and signed operational reports |
| Intermittent delivery | LXMF store-and-forward and delivery state | Cross-bearer custody policy, expiry, and operator-visible relay receipts |
| Data bridges | Reticulum routing plus daemon transport abstraction | Supervised application adapters and bridge loop prevention |
| Low-cost LoRa/packet access | Serial/KISS interface for RNode/RP2040/ESP32-class devices | Hardware profiles, regional TX guards, onboarding |
| Receive-only monitoring | Daemon subscriptions, telemetry, TUI | Enforced receive-only node/adapter role and ingest provenance |
| Group information exchange | SDK topics and subscriptions | Durable replicated topic log and constrained-link prioritization |
| Situation reports and map data | Telemetry points and geospatial markers | Signed report schema, confidence, source chain, TTL, supersession |
| Bulletins and offline information | NomadNet-compatible pages, attachments, content distribution, hub Kiwix/Qdrant | Net bundles, manifests, priority replication, operator curation |
| Comms posture | Policy/RBAC primitives | Net-specific emission-control state machine |
| Scheduled nets and crisis activation | No dedicated domain model | Net/window/incident scheduler and check-in workflow |
| Equipment checks | Interface and link stats | Preflight workflow, synthetic receive-only tests, readiness score |
| TAK interoperability | Marker/telemetry domain types | Optional Cursor-on-Target import/export adapter |

Relevant implementation anchors include:

- transport interface modules in `styrene-rns/src/transport/iface/` (TCP, UDP, Serial/KISS)
- `MeshTransport` in `crates/apps/styrened/src/transport/mesh_transport.rs`
- SDK domain types in `crates/libs/styrene-lxmf/src/sdk/domain.rs`
- store-and-forward config in `crates/libs/styrene-lxmf/src/sdk/types/config.rs`
- pages in `crates/apps/styrened/src/services/pages.rs`
- content distribution in `crates/libs/styrene-content/src/distributor.rs`
- I2P fast/degraded path precedent in `crates/libs/styrene-mesh/src/i2p.rs`

## 4. Target architecture

```text
Operator surfaces
  styrene TUI / CLI / future mobile
              |
              v
Net Operations service (styrened)
  - net definitions and windows
  - incidents and check-ins
  - report validation/priorities
  - comms posture and TX interlocks
  - delivery/custody ledger
              |
      +-------+--------------------+
      |                            |
      v                            v
Native Styrene plane          Adapter supervisor
RNS/LXMF/topics/pages         versioned local adapter protocol
content/telemetry             no adapter gets daemon secrets
      |                            |
TCP/UDP/Serial-KISS           JS8Call / Meshtastic / Winlink
and future drivers            FLDigi-RTTY / ALE / import spool
      |                            |
      +----------- bridged bearers+
```

### 4.1 New domain module: `styrene-netops`

Create `crates/libs/styrene-netops` as a pure, transport-neutral library. Keeping policy and data models out of `styrened` allows deterministic tests and reuse by desktop/mobile clients.

Initial types:

```rust
NetDefinition {
    id, name, description, region_policy,
    memberships, windows, report_policy,
    transport_profiles, default_posture,
    extensions
}

NetWindow {
    id, starts_at, duration, recurrence,
    purpose, coordinator_hint, allowed_transports,
    preflight_lead, extensions
}

Incident {
    id, title, severity, status,
    opened_at, area, activation_reason,
    participating_nets, extensions
}

OperationalReport {
    id, kind, author, created_at, observed_at,
    area, summary, details, confidence,
    priority, ttl, references, attachments,
    supersedes, signature
}

CommsPosture {
    level,
    discovery, heartbeat, auto_ack, auto_relay,
    allowed_transports, tx_budget, quiet_hours,
    operator_confirmation
}

CustodyRecord {
    report_id, bearer, peer, state,
    attempted_at, acknowledged_at, retry_after,
    failure_class
}
```

Report kinds for v1: `check_in`, `situation`, `alert`, `request`, `offer`, `observation`, `bulletin`, `correction`, and `retraction`.

Use stable IDs and canonical serialization so the same report retains identity across RNS, LXMF, print/QR, or a radio application. Sign the canonical report; never sign bearer-specific wrappers.

### 4.2 Net topic convention

Map net operations onto existing topic primitives:

```text
net/<net-id>/control       signed net definitions and posture changes
net/<net-id>/checkins      compact presence/check-in events
net/<net-id>/reports       operational reports
net/<net-id>/requests      resource/information requests
net/<net-id>/bulletins     curated longer-lived information
incident/<incident-id>/*   incident-specific projection of the above
```

A topic is a routing and subscription boundary, not proof of truth. Clients must show author identity, trust, observation time, received time, relay chain, confidence, and whether corroboration exists.

### 4.3 Bridge envelope

Every non-native bearer uses a minimal envelope:

```text
version | report_id | fragment | fragment_count | ttl
source_identity | destination/net | priority | created_at
payload_hash | payload/fragment | signature | relay_trace
```

Rules:

- deduplicate on `report_id + payload_hash`
- cap relay trace growth and detect loops
- preserve the original signature across relays
- allow bearer gateways to add, never replace, custody attestations
- expire traffic by TTL and incident state
- redact fields according to destination trust and bearer confidentiality
- fragment only after applying the bearer's size profile
- schedule by priority, age, and airtime cost; do not use FIFO alone

## 5. Delivery phases

### Phase 0 — decisions and test fixtures

**Goal:** remove ambiguity before adding runtime behavior.

1. Write an ADR defining Styrene as the operations plane and external applications as supervised bearers.
2. Define the regulatory policy interface: region, service, license class, permitted modes, encryption constraints, duty-cycle limits, and `rx_only`.
3. Define canonical report serialization and signing.
4. Define incident and posture lifecycle transitions.
5. Build a low-bandwidth simulator with configurable MTU, latency, loss, outage windows, and bytes/hour.
6. Convert representative GhostNet workflows into fixtures without copying frequencies or shared keys:
   - weekly readiness net
   - ad-hoc disaster net
   - receive-only monitor
   - cross-region relay
   - high-risk low-emission posture

**Exit:** reviewed ADRs, schemas, and failing scenario tests.

### Phase 1 — native net operations MVP

**Goal:** deliver GhostNet's useful workflow entirely over existing Styrene/RNS links.

Implementation:

- add `styrene-netops` models, validation, recurrence, priority, and posture logic
- add SQLite persistence and migrations in `styrened`
- add daemon services for net CRUD, window schedule, incident activation, check-in, report publish/list, and custody query
- project reports onto SDK topics; attach large payloads through existing attachment/content APIs
- add IPC request/response/event types in `styrene-ipc`
- add CLI commands:
  - `styrene net list|show|join|leave`
  - `styrene net check-in`
  - `styrene incident open|close|status`
  - `styrene report send|list|show|correct`
  - `styrene posture get|set`
- add a TUI “Operations” view for next window, readiness, incident status, high-priority reports, queued custody, and current posture

**Exit:** two disconnected test nodes exchange, relay, deduplicate, expire, correct, and display signed reports after a simulated outage.

### Phase 2 — emission control and receive-only safety

**Goal:** make operational posture enforceable rather than advisory.

Implementation:

- introduce an outbound policy gate below all service sends and above concrete interfaces
- define posture presets such as `routine`, `elevated`, `restricted`, and `silent_rx`
- enforce per-interface and per-net TX budgets
- disable discovery, heartbeat, auto-ack, propagation, or gateway relay according to posture
- require explicit operator confirmation to leave `silent_rx` or use a legally restricted bearer
- expose an immutable audit event for every automatic transmission decision
- add “preflight mode” before windows: identity, clock, interface, receive path, queue, storage, and power-health checks without requiring on-air transmission

**Exit:** property and integration tests prove `silent_rx` produces zero outbound frames, including acknowledgements and discovery traffic.

### Phase 3 — adapter framework and first bearers

**Goal:** bridge existing radio ecosystems without embedding their full stacks.

Build a versioned local adapter protocol over a Unix socket/named pipe using the project's existing structured IPC conventions. The adapter supervisor owns process lifecycle, capability negotiation, bounded queues, deadlines, and redacted logs. Adapters receive scoped bearer credentials only; they never receive Styrene identity private keys.

Adapter capability declaration:

```text
bearer name/version
rx, tx, broadcast, addressed, ack, store_forward
max_payload, text_only, estimated_bps
half_duplex, operator_tuned, requires_accurate_clock
confidentiality, regional constraints, health
```

Recommended order:

1. **Meshtastic adapter** — useful local LoRa text/telemetry bridge; map channel traffic to an explicitly configured net and preserve source provenance.
2. **JS8Call adapter** — compact check-ins and reports through a locally running application where a documented local API is available. Start receive-only, then require operator-supervised TX.
3. **Import/export spool adapter** — human-carried USB/QR/file bundles for severe outages; also provides a safe generic fallback.
4. **Winlink P2P adapter** — exchange report bundles as messages/attachments through supported local workflows; do not treat Winlink as a transparent packet link.
5. **FLDigi/RTTY adapter** — receive and originate short blind bulletins with aggressive size limits and no delivery assumption.
6. **ALE adapter** — last, because radio control, automatic sounding, legal constraints, and software variance require hardware-in-loop validation.

Each adapter starts as `rx_only`. TX is enabled only after fixtures, simulator tests, and hardware-in-loop tests pass.

**Exit:** a report enters through one adapter, is signed/normalized/deduplicated by Styrene, traverses native RNS, and exits a second adapter without a relay loop.

### Phase 4 — constrained-link routing and custody

**Goal:** make mixed bearers useful under real bandwidth and outage constraints.

Implementation:

- add bearer scoring using availability, payload fit, confidentiality, latency, energy/airtime cost, and legal policy
- add compact binary encoding plus a human-readable text projection
- add summary-first delivery: header/summary before details and attachments
- add fragment selective repeat where the bearer can address peers; use fountain/repeated bulletin strategy only after simulation justifies it
- add gateway leases so multiple bridges do not rebroadcast the same public report simultaneously
- add hop/custody limits, retry jitter, congestion backoff, and quiet-window scheduling
- surface “queued,” “heard,” “custodied,” “delivered,” and “confirmed by operator” as distinct states

**Exit:** simulation demonstrates bounded duplicate traffic and delivery of priority reports under loss, partitions, and competing gateways.

### Phase 5 — bulletins, offline knowledge, and operational picture

**Goal:** turn messages into durable, useful shared information.

Implementation:

- generate a compact incident landing page from current signed reports
- publish curated bulletin bundles through `styrene-content`
- replicate bundle manifests first and chunks by priority/availability
- integrate hub Kiwix/Qdrant search as a local enrichment source, clearly separating reference material from live reports
- add marker projections for geospatial reports and optional CoT import/export
- add report corroboration views without automatic “truth scores”
- support printable/QR report envelopes for hand carry

**Exit:** an edge node with no Internet can obtain the incident summary, inspect provenance, fetch selected bulletin content, and carry a signed report bundle to another partition.

### Phase 6 — field validation and release hardening

**Goal:** validate behavior rather than merely protocol correctness.

Test matrix:

- desktop ↔ desktop over TCP with induced partitions
- desktop ↔ RNode-class device over Serial/KISS
- two gateways attached to the same external bearer
- receive-only node under every posture transition
- clock skew and stale-window handling
- malicious duplicate, replay, oversized fragment, false custody, and forged report attempts
- adapter crash/restart and queue recovery
- database rollback/upgrade
- 24-hour low-rate soak and scheduled-window exercise
- jurisdiction profile refusing an invalid TX configuration

Run a staged exercise: lab simulation, shielded/dummy-load hardware test, receive-only field test, then lawful supervised transmission.

## 6. Security and trust model

1. **Identity is not veracity.** A valid signature proves who signed a report, not that the report is true.
2. **Preserve provenance.** Never collapse author, relay, gateway, and local observer into one source field.
3. **Replay resistance.** Use report IDs, timestamps, TTL, supersession, and bounded replay caches.
4. **Least privilege.** Adapter permissions are bearer-scoped and net-scoped. Remote command capability is separate from report forwarding.
5. **No shared global secret.** Private nets use per-net key material with rotation and membership revocation; public nets assume public observability.
6. **Metadata awareness.** The UI must warn that callsigns, timing, frequency, direction finding, and gateway patterns can expose participants even when payloads are encrypted elsewhere.
7. **Untrusted content handling.** Pages, attachments, and imported text render inertly; no scripts, shell interpolation, or automatic command execution.
8. **Operator authority.** Automatic bridging and transmission are bounded by posture, regional policy, budget, and explicit enablement.
9. **Abuse controls.** Local blocklists, per-identity rate limits, bounded queues, report-size limits, and trust-filtered views are required even for decentralized public nets.
10. **Secrets separation.** RNS identity keys stay in the daemon/identity backend and are never copied into radio sidecars.

## 7. Product decisions and tradeoffs

### Adopt

- scheduled and ad-hoc net workflows
- multiple paths with explicit bridge/custody semantics
- receive-only roles
- structured reports with concise text projections
- posture-driven emission controls
- decentralized operation with optional coordinators
- offline bulletins and knowledge bundles

### Defer

- native DSP/modem implementations
- transparent IP tunneling over low-rate HF modes
- autonomous ALE radio control
- automatic truth/reputation scoring
- broad TAK synchronization over HF

### Reject

- hard-coded frequencies, shared keys, or callsigns
- “emergency means regulations no longer apply” logic
- hidden automatic transmissions
- using a public group name as authorization
- assuming delivery merely because a gateway accepted custody

The main tradeoff is deliberate: adapters add deployment complexity, but keep radio-specific failure modes and legal controls outside the trusted core. A monolithic all-mode daemon would be easier to demo and substantially harder to secure, test, or operate safely.

## 8. Initial backlog

The first shippable increment should stay narrow:

1. `styrene-netops` types and canonical report signing.
2. `styrened` net/incident/report service with SQLite persistence.
3. IPC and CLI support for nets, windows, check-ins, reports, and posture.
4. `silent_rx` enforcement at the outbound transport gate.
5. Lossy-link simulator and end-to-end two-node tests.
6. TUI operations dashboard.
7. Receive-only import/export spool adapter.
8. Meshtastic adapter spike.
9. JS8Call receive-only adapter spike.

Do not start Winlink, RTTY, or ALE transmission work until the native MVP and `silent_rx` invariants are proven.

## 9. Success criteria

The integration is successful when:

- an operator can install a net plan before an incident and see the next readiness window
- a receive-only participant can monitor with a machine-enforced no-transmit guarantee
- an incident can be activated without a central server
- signed reports retain identity, provenance, TTL, and correction history across relays
- priority traffic survives partitions and resumes without broadcast storms
- the same report can cross native RNS and at least two supervised external bearers
- no frequency, key, callsign, or legal assumption is embedded as a universal default
- adapters can fail or be removed without corrupting the native Styrene message plane
- field exercises produce auditable readiness and delivery evidence

## 10. Recommended next decision

Approve **Phase 0 + Phase 1 only** as the first implementation change. That proves the doctrine is useful on Styrene's existing transport before taking on radio-application integration risk. Treat Meshtastic and JS8Call as parallel, receive-only validation spikes after the net/report schema stabilizes.
