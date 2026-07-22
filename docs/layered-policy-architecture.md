# Layered Policy Architecture

## Status

**Decided.** GhostNet proceeds with the compiled Rust safety kernel as Layer 0. The product leaves typed integration points for future signed declarative policy engines and operator configuration, but neither may weaken Layer 0 or bypass Styrene authorization.

## Decision

Policy evaluation has three ordered layers:

1. **Layer 0 — compiled safety kernel.** Deterministic Rust invariants shipped with GhostNet.
2. **Layer 1 — signed declarative policy.** A future bounded engine such as embedded Rego/WASM, Cedar, or a constrained GhostNet policy language.
3. **Layer 2 — operator and mission configuration.** Nets, roles, disclosure profiles, posture requests, confirmations, and deployment parameters.

The effective decision is the intersection of substrate authorization, Layer 0, Layer 1, current capability facts, Layer 2, and required operator confirmation. A deny at any layer wins. Indeterminate input blocks consequential action.

## Layer 0 invariants

Layer 0 is not dynamically replaceable. It SHALL enforce that:

- a Styrene authorization denial cannot be bypassed;
- unknown authorization cannot become permission;
- silent posture cannot be relaxed by an emergency exception;
- confidential artifacts cannot use a bearer without required confidentiality;
- expired policy cannot authorize an operation;
- absent negotiated capabilities cannot be assumed;
- client-guarded behavior cannot be labeled daemon-enforced;
- hardware receive-only cannot be inferred from software configuration;
- malformed, unknown-required, or contradictory extension results fail closed;
- every consequential decision emits stable reason codes and auditable provenance.

## Extension rule

Layer 1 and Layer 2 may:

- further restrict a Layer 0 permit;
- require confirmation;
- select among alternatives already proven safe by Layer 0;
- return indeterminate when required facts are missing.

They may not:

- convert Layer 0 deny or indeterminate into permit;
- increase an enforcement-evidence level;
- override Styrene RBAC or peer policy;
- claim unsupported substrate capabilities;
- introduce private key material;
- depend on network, filesystem, wall-clock, randomness, or mutable ambient state during deterministic evaluation.

Time, capabilities, actor authorization, and deployment configuration are explicit versioned inputs.

## Future engine requirements

A Layer 1 implementation must be offline-capable, resource-bounded, version-pinned, deterministic for identical inputs, and driven by immutable signed policy bundles. Its decision output uses the GhostNet `PolicyExtensionResult` contract rather than arbitrary text.

Evaluation provenance records at least:

- safety-kernel version;
- policy input schema version and hash;
- bundle ID, version, and hash when present;
- engine ID and version;
- capability snapshot hash;
- Layer 0 and extension outcomes;
- stable reason codes;
- evaluation timestamp supplied through the `Clock` port.

OPA/Rego, Cedar, and a small GhostNet DSL remain implementation candidates. No engine is selected by this decision.

## Ownership

GhostNet owns operational policy bundles, evaluation inputs, extension adapters, configuration, confirmations, and audit records. Styrene remains authoritative for substrate authorization and actual outbound enforcement. The public dependency direction in [`../WORKSPACE-CHARTER.md`](../WORKSPACE-CHARTER.md) remains unchanged.
