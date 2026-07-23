# Layered Policy Architecture

## Status

**Decided and amended.** GhostNet is an Operating Modality within a Styrene instance. Styrene owns the unified policy substrate. GhostNet contributes compiled doctrine invariants, typed OM manifests, domain predicates, scenarios, commands, and views; it does not host an independent communications-policy authority.

The selected engine stack is defined in [`policy-engine-selection.md`](policy-engine-selection.md).

## Decision

Evaluation composes these ordered authorities:

1. **Compiled invariants.** Deterministic Rust safety and domain semantics shipped with Styrene and its installed modes.
2. **Styrene substrate authorization.** Identity, messaging, tunnel, resource, and peer authority.
3. **Cedar policy.** The normative embedded engine for organizational and OM authorization.
4. **Compatibility and exceptional restrictions.** Optional Regorus/Rego compatibility and bounded WASM helpers.
5. **Operating context.** Active OM epoch, deployment bindings, runtime capability facts, and explicit operator confirmation.

The effective result uses monotonic restriction:

```text
Permit < RequireConfirmation < Indeterminate < Deny
```

No later layer can relax an earlier restriction. Indeterminate input blocks consequential action.

## Compiled invariants

Compiled invariants are not dynamically replaceable. They SHALL enforce that:

- a Styrene authorization denial cannot be bypassed;
- unknown authorization cannot become permission;
- silent posture cannot be relaxed by an emergency exception;
- confidential artifacts cannot use an execution path without required confidentiality;
- expired policy cannot authorize an operation;
- absent negotiated capabilities cannot be assumed;
- client-guarded behavior cannot be labeled daemon-enforced;
- hardware receive-only cannot be inferred from software configuration;
- malformed, unknown-required, contradictory, timed-out, or trapped extension results fail closed;
- domain state machines, canonicalization, hashes, and signature inputs retain compiled semantics;
- every consequential decision emits stable reason codes and auditable provenance.

## Operating Modality rule

A GhostNet OM bundle and its deployment bindings may:

- further restrict an operation permitted by higher layers;
- require typed confirmation;
- select among alternatives already proven safe;
- bind Styrene identities to OM-specific roles;
- supply Cedar policies and typed obligations;
- include reviewed Rego compatibility policy;
- invoke explicitly authorized bounded WASM helpers;
- return indeterminate when required facts are missing.

They may not:

- convert deny or indeterminate into permit;
- increase an enforcement-evidence level;
- override Styrene identity, messaging, tunnel, or peer authorization;
- claim unsupported substrate capabilities;
- introduce or access private key material;
- redefine GhostNet artifact or workflow semantics;
- depend on ambient network, filesystem, wall-clock, randomness, environment, or mutable hidden state during deterministic evaluation.

Time, capabilities, actor authorization, active epoch, and deployment configuration are explicit versioned inputs.

## Runtime engine requirements

Every policy engine or helper must be offline-capable, resource-bounded, version-pinned, deterministic for identical inputs, and driven by immutable signed bundles. Outputs use Styrene-owned typed decisions and obligations rather than arbitrary executable instructions.

Evaluation provenance records at least:

- compiled-invariant version;
- policy input schema version and hash;
- OM and policy bundle IDs, versions, and hashes;
- engine IDs and versions;
- capability snapshot hash;
- each layer's outcome and determining rules;
- obligations and satisfaction evidence;
- stable reason codes;
- activation epoch;
- evaluation timestamp supplied through the host clock contract.

## Ownership

Styrene owns:

- the generic policy request, decision, and obligation contracts;
- Cedar, Regorus, and WASM engine adapters;
- policy-set trust, activation epochs, and audit provenance;
- identity and substrate authorization;
- messaging, tunnels, routing, persistence, and receipts;
- enforcement of satisfied obligations before execution.

GhostNet mode owns:

- operational artifact and workflow semantics;
- OM source manifests and family profiles;
- OM-specific Cedar schemas, policies, and scenarios;
- compiled doctrine predicates;
- projection definitions and domain views;
- guided OM authoring and effective-rule explanations.

The dependency direction remains Styrene core → generic policy substrate, while GhostNet mode consumes the public extension contracts. GhostNet does not wrap Styrene behind a separate operator-facing program.
