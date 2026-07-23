# GhostNet Operating Modality Architecture

> **Workspace authority:** This architecture implements [`../WORKSPACE-CHARTER.md`](../WORKSPACE-CHARTER.md). The charter controls if another document conflicts.

## Status

**Decided.** GhostNet is a family of signed Operating Modality profiles activated inside Styrene Mesh. It is not an independent application, SDK wrapper, messaging client, or policy authority.

## User model

A day-to-day operator remains in one Styrene application with one portable identity, contact graph, inbox, tunnel model, and delivery state. Activating a profile such as `NYC_EMS1_FIRE` changes visible operational tools, domain workflows, disclosure constraints, posture, and policy context. It does not start another program or create another communications path.

The user-visible rule is:

> If GhostNet introduces a second identity, contact list, inbox, delivery indicator, or send path, the boundary is wrong.

## System context

```text
┌───────────────────────────────────────────────────────────────┐
│ Styrene operator surfaces                                     │
│ chat · tunnels · files · operational views · mode switcher    │
├───────────────────────────────────────────────────────────────┤
│ Styrene command and projection registry                       │
│ one action model shared across supported surfaces             │
├───────────────────────────────────────────────────────────────┤
│ Styrene unified policy and exact-byte egress gate              │
│ identity · Cedar · compatibility restrictions · obligations   │
├───────────────────────────────────────────────────────────────┤
│ Styrene messaging, tunnels, queues, persistence, and receipts │
├───────────────────────────────────────────────────────────────┤
│ Public Operating Modality extension contracts                 │
├───────────────────────────────────────────────────────────────┤
│ GhostNet OM contribution                                      │
│ manifests · doctrine · artifacts · projections · workflows    │
│ policy sources · scenarios · commands · views                 │
└───────────────────────────────────────────────────────────────┘
```

## GhostNet OM contribution

### Manifests and profiles

A signed OM bundle identifies:

- OM family and profile ID;
- compatible Styrene and policy-contract versions;
- issuer and trust requirements;
- validity interval and deployment bindings;
- role vocabulary and identity-binding constraints;
- domain schemas, doctrine version, and policy sources;
- commands, views, confirmations, and projection definitions;
- requested capabilities and fail-closed behavior;
- canonical bundle hash and signature provenance.

Profiles are immutable. Switching profile activates a new epoch; it does not mutate the operator identity.

### Compiled doctrine

The deterministic doctrine library defines:

- prepared-net revisions and communication windows;
- incident state transitions and fork handling;
- report correction, retraction, and expiry;
- canonical artifact bytes and source hashes;
- safe projection forms and explicit omission metadata;
- posture vocabulary and domain reason codes.

These are domain semantics, not a communications execution service.

### Policy sources

GhostNet supplies OM-specific Cedar schemas, policies, obligations, and scenario vectors. Optional Regorus policy and bounded helper declarations are compatibility inputs. Styrene owns engine execution, trust, activation, composition, audit, and final enforcement.

### Commands and views

GhostNet contributes structured actions such as situation report, incident transition, correction, and posture request, plus views that explain active OM, domain state, omissions, and policy reasons. Styrene owns the action dispatch, destination resolution, messaging, tunnel, and receipt lifecycle.

## Exact-byte operation path

```text
operator/domain intent
        ↓
Styrene resolves identity, destination, membership, and active OM epoch
        ↓
GhostNet doctrine constructs a typed candidate artifact/projection
        ↓
Styrene canonicalizes the final envelope and all observable metadata
        ↓
compiled invariants + Styrene authorization + Cedar + restrictions
        ↓
typed obligations are proven satisfied
        ↓
single-use permit binds exact bytes, destination, epoch, and facts
        ↓
atomic sealed-operation enqueue
        ↓
Styrene messaging, tunnel, delayed delivery, or sandboxed adapter
```

No alternative path may reach the final rows.

GhostNet does not call signing, publish, poll, retry, attachment, marker, or transmission-policy methods as an application-owned workflow. It supplies typed domain material to the generic Styrene operation pipeline.

## Disclosure safety

Every output path—including previews, attachments, exports, automation, telemetry, tunnels, delayed release, and adapters—uses the same gate. Policy approves the final exact bytes and metadata rather than an earlier mutable draft.

A projection is a new governed derivative, not a truncation. It binds to the immutable source hash, declares its form and omissions, receives its own classification, and is independently authorized.

Unknown or stale authorization, membership, policy, OM epoch, transport facts, classification, or engine results block execution. Queue release revalidates current constraints.

## Policy engine stack

The selected stack is detailed in [`policy-engine-selection.md`](policy-engine-selection.md):

- compiled Rust invariants;
- Cedar as the normative embedded authorization engine;
- Regorus for constrained Rego compatibility;
- bounded WASM for exceptional pure helpers;
- monotonic deny-overrides composition;
- typed obligations rather than executable engine instructions.

## External bearers

An external adapter receives only a sealed, already-authorized operation with exact bytes, destination handle, constraints, expiry, and single-use permit. It cannot access source history, private keys, unrestricted messaging/tunnels, or choose a broader audience or weaker projection.

## Persistence

Styrene owns messages, attachments, queues, receipts, activation epochs, and policy-decision provenance. GhostNet source artifacts and OM bundles may be stored as signed domain content through generic Styrene facilities. Rebuildable views are not a second source of communications truth.

## Compatibility and evolution

OM bundles declare contract versions and capabilities. Unsupported required semantics prevent activation. Engine or doctrine upgrades replay golden vectors and OM scenarios. Material decision changes require an explicit migration and a new signed bundle or activation epoch.

## Non-goals

GhostNet does not provide:

- a second communications daemon or client;
- an SDK/RPC facade over Styrene;
- a separate policy service;
- direct signing or key custody;
- independent queueing, retry, routing, persistence, or receipts;
- an alternate tunnel or adapter execution path;
- hidden policy implementations per UI surface.
