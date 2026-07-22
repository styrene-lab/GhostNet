# GhostNet Doctrine Kernel

## Status

**Normative domain design.** This document defines the pure model and policy engine owned by GhostNet. It intentionally excludes Styrene daemon internals, persistence implementation details, and bearer-specific modem behavior.

## 1. Purpose

The doctrine kernel converts prepared communications doctrine into deterministic, testable decisions. It answers:

- Is this net definition structurally and semantically valid?
- What communication window is active or next?
- Is an incident transition valid?
- Is a report valid, expired, corrected, or retracted?
- What may be disclosed to a given audience and bearer?
- Is a proposed emission permitted by GhostNet policy?
- What projection should be sent over a constrained bearer?
- What operator confirmation or substrate enforcement is required?

It does not send, store, route, authorize at the substrate layer, or hold private keys.

## 2. Determinism contract

For the same:

- input artifact bytes;
- policy revision;
- capability snapshot;
- explicit evaluation time;
- local operator decision;

all conforming implementations MUST produce the same:

- validation result;
- policy decision code;
- canonical artifact bytes;
- projection plan;
- derived incident/net state.

The kernel MUST NOT call the system clock, environment, network, random generator, filesystem, or secret store. Those values enter through explicit parameters.

## 3. IDs and time

### 3.1 Artifact IDs

Artifact IDs are 128-bit opaque values encoded as 26-character Crockford Base32 strings. UUID/ULID generation is an application responsibility. The kernel validates syntax and treats IDs as opaque.

An artifact's identity is the tuple:

```text
(schema_id, artifact_id, body_hash)
```

Reusing an `artifact_id` with different canonical body bytes is a conflict, not an update.

### 3.2 Identity references

A signer is represented as an opaque `IdentityRef` owned by Styrene. The kernel does not assume the key algorithm or expose public-key bytes unless the Styrene contract defines them.

### 3.3 Time

Times are unsigned Unix epoch milliseconds. Evaluation functions accept `now_ms`. Wall-clock skew policy is explicit in each net definition.

Unknown or unsynchronized time is represented explicitly. A client MUST NOT silently use local wall time when policy requires synchronized time.

## 4. Artifact envelope

Every signed GhostNet artifact uses an envelope:

```rust
SignedArtifact {
    schema: SchemaId,
    body: CanonicalBody,
    body_hash: Hash32,
    signer: IdentityRef,
    signature: DetachedSignature,
    signed_at_ms: u64,
}
```

`body_hash` is computed over canonical body bytes, not over the envelope. Signature input is domain-separated:

```text
"ghostnet-artifact-v1\0" || schema_utf8 || "\0" || canonical_body_bytes
```

The detached signature covers the exact byte sequence above. `signer`, `algorithm`, and any Styrene verification metadata live outside the canonical body but are bound by the signature operation's identity context.

### 4.1 Canonical encoding

The canonical encoding is **JCS JSON** as specified by RFC 8785, restricted as follows:

- UTF-8 only;
- object keys are schema-defined strings;
- no floating-point values;
- no `null` where omission is semantically equivalent;
- integers must be within the interoperable signed 53-bit range unless encoded as decimal strings by schema;
- arrays preserve order and schemas define whether order is semantic;
- all extension fields live in a namespaced `extensions` object;
- unknown required extension namespaces make an artifact uninterpretable;
- unknown optional extensions are retained when forwarding.

A schema change that alters canonical bytes requires a new schema ID. Field reordering in source code MUST NOT alter canonical bytes.

Golden vectors MUST include canonical bytes, body hash, signature input, signer identity reference, detached signature, and expected verification result.

### 4.2 Hashing

The v1 body hash algorithm is SHA-256. Hash agility is represented by:

```text
sha256:<64 lowercase hexadecimal characters>
```

The algorithm prefix is mandatory anywhere a hash crosses a contract boundary.

## 5. Net definition

```rust
NetDefinition {
    id: NetId,
    revision: u64,
    previous_revision_hash: Option<HashRef>,
    name: String,
    description: String,
    effective_at_ms: u64,
    expires_at_ms: Option<u64>,
    issuer: IdentityRef,
    audience: AudiencePolicy,
    roles: Vec<RoleBinding>,
    windows: Vec<NetWindow>,
    report_policy: ReportPolicy,
    disclosure_profiles: Vec<DisclosureProfile>,
    posture_profiles: Vec<PostureProfile>,
    bearer_profiles: Vec<BearerProfile>,
    extensions: Extensions,
}
```

Validation requirements:

- IDs and names are non-empty and bounded;
- revision zero is invalid;
- revision one has no predecessor; later revisions require one;
- effective time precedes expiry;
- role bindings refer to known roles;
- window and profile IDs are unique;
- every allowed report kind has at least one authorized role;
- every referenced disclosure, posture, and bearer profile exists;
- no bearer profile claims capabilities absent from its declared capability set;
- regional policy is a reference to configured policy data, not embedded legal advice.

### 5.1 Revision resolution

A revision is applicable only if:

1. its signature verifies;
2. the issuer is authorized by the previous applicable revision or bootstrap policy;
3. predecessor hash matches;
4. its effective-time constraints pass;
5. no earlier artifact with the same ID has a different body hash.

Competing valid children of one revision create a fork. The kernel returns `NetRevisionFork` and does not silently choose by arrival time. An authorized merge/supersession artifact is required.

## 6. Communication windows

```rust
NetWindow {
    id: WindowId,
    purpose: WindowPurpose,
    schedule: WindowSchedule,
    duration_ms: u64,
    checkin_lead_ms: u64,
    allowed_bearer_ids: Vec<BearerProfileId>,
    default_posture_id: PostureProfileId,
    coordinator_hint: Option<IdentityRef>,
    max_clock_skew_ms: u64,
}
```

`WindowSchedule` is either:

- one-shot UTC start;
- bounded recurring UTC schedule using a restricted recurrence grammar;
- externally triggered, with a signed activation artifact.

The v1 recurrence grammar supports interval, UTC time-of-day, weekdays, start, and optional end. It does not support local civil time or daylight-saving interpretation. UI may render local time, but canonical schedules remain UTC.

Kernel operations:

```rust
window_status(window, now_ms, clock_state) -> WindowStatus
next_occurrence(window, after_ms) -> Result<Occurrence>
validate_checkin_window(window, report_time_ms, now_ms) -> Decision
```

If clock uncertainty exceeds `max_clock_skew_ms`, time-sensitive transmission decisions return `IndeterminateClock`, not `Allow`.

## 7. Incident lifecycle

```text
planned ──activate──> active ──monitor──> monitoring ──close──> closed
    └────cancel─────> cancelled       └──reactivate──> active
active ──close──────> closed
active ──cancel─────> cancelled
monitoring ─cancel──> cancelled
```

Closed and cancelled are terminal for an incident ID. Reopening requires a new incident with `previous_incident_id`.

Each transition is an immutable artifact:

```rust
IncidentTransition {
    id: ArtifactId,
    incident_id: IncidentId,
    from: Option<IncidentStatus>,
    to: IncidentStatus,
    reason: String,
    effective_at_ms: u64,
    participating_net_ids: Vec<NetId>,
    area: Option<GeoArea>,
    previous_transition_hash: Option<HashRef>,
}
```

The first transition has `from = None` and `to = planned` or `active`. Derived state requires a valid hash chain and an operationally authorized signer. Duplicate transitions are idempotent. A divergent child creates `IncidentTransitionFork`.

## 8. Operational reports

```rust
OperationalReportBody {
    schema_version: u16,
    id: ReportId,
    kind: ReportKind,
    net_id: NetId,
    incident_id: Option<IncidentId>,
    author: IdentityRef,
    created_at_ms: u64,
    observed_at_ms: Option<u64>,
    area: Option<GeoArea>,
    summary: String,
    details: Option<String>,
    confidence: Confidence,
    priority: Priority,
    expires_at_ms: u64,
    references: Vec<ArtifactRef>,
    attachments: Vec<AttachmentRef>,
    supersedes: Vec<ReportRef>,
    source_chain: Vec<SourceAssertion>,
    extensions: Extensions,
}
```

Report kinds:

- `check_in`;
- `situation`;
- `alert`;
- `request`;
- `offer`;
- `observation`;
- `bulletin`;
- `correction`;
- `retraction`.

Validation:

- summary contains 1–280 UTF-8 bytes;
- details contain at most 16 KiB in the canonical artifact;
- creation is not before observation unless observation is unknown;
- expiry is after creation and within net policy bounds;
- correction/retraction has at least one superseded report;
- ordinary reports have no superseded reports unless policy permits consolidation;
- attachment references include algorithm-qualified checksum, size, and media type;
- area precision does not exceed the artifact's source disclosure class;
- source-chain depth and serialized size are bounded;
- author field must match the identity used for detached signing.

### 8.1 Corrections and retractions

Reports are immutable. A correction or retraction creates a signed new report. Derived views retain the original and mark it superseded. A retraction means “the authorized signer withdraws this report,” not “the bytes disappear.”

A correction by a different signer does not supersede unless net policy gives that role authority. Otherwise it is a conflicting observation referencing the original.

### 8.2 Confidence and corroboration

Confidence values are `unknown`, `low`, `medium`, `high`, and `confirmed`. They are author claims. The kernel may calculate corroboration metadata from independent source identities but MUST NOT convert that into a factual truth score.

## 9. Disclosure policy

A single report may have multiple projections. The canonical report remains unchanged.

```rust
DisclosureProfile {
    id: DisclosureProfileId,
    audience: AudienceSelector,
    max_area_precision: AreaPrecision,
    include_details: bool,
    include_attachment_metadata: bool,
    allowed_attachment_media: Vec<MediaPattern>,
    expose_source_chain: SourceExposure,
    expose_author: AuthorExposure,
    max_serialized_bytes: u64,
}
```

Projection returns:

```rust
ProjectionPlan {
    source_artifact: ArtifactRef,
    projection_schema: SchemaId,
    audience: AudienceSelector,
    body: CanonicalProjectionBody,
    omitted_fields: Vec<OmissionReason>,
    requires_projection_signature: bool,
}
```

A projection never claims to be the source artifact. It references the source artifact ID/hash and identifies the projecting gateway or client. If fields are omitted, omission metadata is explicit.

## 10. Communications posture

Posture is a policy input, not a transport implementation.

```rust
PostureProfile {
    id: PostureProfileId,
    level: PostureLevel,
    allowed_bearers: Vec<BearerProfileId>,
    allowed_intents: Vec<EmissionIntent>,
    tx_budget: Option<TxBudget>,
    quiet_intervals: Vec<UtcInterval>,
    confirmation: ConfirmationPolicy,
    required_enforcement: EnforcementRequirement,
}
```

Levels:

- `routine`;
- `elevated`;
- `restricted`;
- `silent_rx`.

Emission intents:

- `user_report`;
- `check_in`;
- `discovery`;
- `heartbeat`;
- `acknowledgement`;
- `relay`;
- `propagation_sync`;
- `path_request`;
- `adapter_control`.

`silent_rx` permits no emission intent, has zero TX budget, and allows no transmit-capable bearer. Leaving it requires explicit operator confirmation and successful substrate policy application when machine enforcement is requested.

## 11. Policy decision API

```rust
PolicyInput {
    now_ms: u64,
    clock_state: ClockState,
    identity: IdentityRef,
    roles: RoleSnapshot,
    net_revision: ValidatedNetDefinition,
    incident_state: Option<DerivedIncidentState>,
    posture: PostureProfile,
    bearer: BearerCapability,
    substrate: StyreneCapabilitySnapshot,
    proposed_action: ProposedAction,
}

PolicyDecision {
    outcome: DecisionOutcome,
    reasons: Vec<DecisionReason>,
    required_confirmations: Vec<Confirmation>,
    required_substrate_controls: Vec<TransmissionControl>,
    projection: Option<ProjectionPlan>,
    audit_fields: AuditFields,
}
```

Outcomes:

- `allow`;
- `deny`;
- `require_confirmation`;
- `indeterminate`.

Decision reason codes are stable machine-readable enums. Human descriptions are localized by clients.

Mandatory deny reasons include:

- artifact invalid or signature unverified;
- caller lacks operational role;
- Styrene capability unavailable;
- Styrene authorization denied;
- artifact expired;
- outside valid window where policy requires one;
- posture forbids intent;
- bearer not allowed;
- TX budget exhausted;
- quiet interval active;
- projection exceeds bearer constraints;
- required disclosure policy cannot be satisfied.

Mandatory indeterminate reasons include:

- clock quality insufficient;
- net revision fork;
- incident transition fork;
- substrate enforcement state unknown;
- retention gap prevents required state derivation.

## 12. Enforcement levels

The kernel distinguishes:

```rust
EnforcementLevel {
    Advisory,
    ClientGuarded,
    DaemonEnforced,
    HardwareReceiveOnly,
}
```

- `Advisory`: UI warning only.
- `ClientGuarded`: GhostNet refuses to request a transmission, but other applications or daemon activity may transmit.
- `DaemonEnforced`: Styrene confirms a generic policy blocks all prohibited outbound classes within its enforcement boundary.
- `HardwareReceiveOnly`: transmit-capable hardware is absent or physically disabled, supported by local evidence.

The UI and audit log MUST display the achieved level. `ClientGuarded` may never be labeled “zero RF” or “machine-enforced silent.”

## 13. Bearer capability and degradation

```rust
BearerCapability {
    id: BearerId,
    rx: bool,
    tx: bool,
    broadcast: bool,
    addressed: bool,
    acknowledgements: AckModel,
    store_forward: bool,
    max_payload_bytes: u64,
    estimated_bps: Option<u64>,
    half_duplex: bool,
    operator_tuned: bool,
    accurate_clock_required: bool,
    confidentiality: ConfidentialityClass,
    legal_profile_ref: Option<String>,
}
```

Projection degradation order:

1. complete report with attachment references;
2. complete text without attachments;
3. summary, priority, time, area class, and report reference;
4. fixed compact alert/check-in code with report reference;
5. deny with `NoSafeProjection`.

The kernel never truncates canonical bytes arbitrarily. Every degraded form has a schema and references its source.

## 14. Audit model

Every consequential command creates a local audit record:

```rust
DecisionAuditRecord {
    id: AuditId,
    evaluated_at_ms: u64,
    action_fingerprint: HashRef,
    policy_revision: HashRef,
    capability_snapshot_hash: HashRef,
    outcome: DecisionOutcome,
    reasons: Vec<DecisionReason>,
    operator_confirmation: Option<ConfirmationEvidence>,
    requested_enforcement: EnforcementRequirement,
    achieved_enforcement: EnforcementLevel,
    styrene_correlation_id: Option<String>,
}
```

Audit records contain references and hashes, not secret keys or complete sensitive payloads. Logs are append-only at the application layer and may be exported as signed exercise evidence.

## 15. Error taxonomy

Errors are separated into:

- `ValidationError` — malformed or semantically invalid artifact;
- `VerificationError` — signature/hash/identity failure;
- `AuthorizationError` — valid identity lacks required operation authority;
- `PolicyDenial` — valid request prohibited under current doctrine;
- `CapabilityError` — required Styrene or bearer capability absent;
- `ConflictError` — artifact ID collision or event-chain fork;
- `StateIncomplete` — retention gap, unknown predecessor, or clock uncertainty;
- `ProjectionError` — safe bounded representation cannot be produced.

Transport and persistence errors do not enter the kernel taxonomy; adapters map them at the application boundary.

## 16. Test obligations

### 16.1 Unit and property tests

- every valid incident transition;
- every invalid transition;
- report length and time boundaries;
- canonical bytes independent of source field order;
- altered body fails detached verification;
- correction and retraction chain semantics;
- fork detection;
- UTC recurrence around week/year boundaries;
- disclosure never increases geographic precision;
- projection size never exceeds bearer limit;
- silent posture denies every emission intent;
- leaving silent posture requires confirmation;
- policy decisions are deterministic;
- unknown capabilities never default to allow.

### 16.2 Golden vectors

Golden vectors are language-neutral and committed under `tests/golden-vectors/`. Each includes positive and negative verification cases. A Styrene detached-signing integration test signs the exact golden bytes and verifies through the public SDK.

### 16.3 Model-based tests

Generate event sequences for net revision and incident transition graphs. Assert that:

- linear authorized history has one derived state;
- duplicates are idempotent;
- divergent children produce conflict;
- terminal incidents never reactivate under the same ID;
- invalid signatures never alter derived state.

## 17. Security invariants

1. The doctrine crate contains no private-key type.
2. The application cannot sign bytes other than explicit domain-separated canonical artifact bytes without a separate generic capability.
3. Every mutable operational change is represented by an immutable signed artifact.
4. Arrival order does not grant authority.
5. Topic membership does not grant operational role.
6. A receipt is not factual confirmation.
7. A gateway projection cannot replace original authorship.
8. Unknown policy/capability state fails closed for transmissions.
9. Client-side silence is never represented as daemon- or hardware-enforced silence.
10. Retraction preserves historical evidence.

## 18. V1 implementation boundary

V1 includes:

- net definitions and UTC windows;
- incident transition chains;
- signed reports, corrections, and retractions;
- disclosure projections;
- posture policy and decisions;
- public Styrene topic/message projection;
- golden vectors and a fake Styrene port.

V1 excludes:

- autonomous gateway election;
- probabilistic truth scoring;
- native radio control;
- civil-time recurrence;
- transparent tunnels;
- automatic legal determination;
- mutable daemon-side GhostNet state.
