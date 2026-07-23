# Policy Engine Selection

## Status

**Decided.** Styrene's unified policy substrate will use Cedar as its primary embedded authorization engine. Regorus provides optional Rego v1 compatibility. WebAssembly is reserved for bounded, exceptional helper logic at unusual integration boundaries; it is not the primary policy language or a path around Cedar, Styrene authorization, or compiled safety invariants.

## Decision

The policy stack is:

```text
compiled Rust invariants
        ↓
Styrene authorization and policy composition
        ├── Cedar (normative primary engine)
        ├── Regorus (optional Rego compatibility)
        └── bounded WASM helper (exceptional boundary adapter)
        ↓
typed decision + obligations + provenance
```

GhostNet Operating Modalities are signed, immutable configuration bundles consumed by a Styrene instance. They are not separate programs and do not host an independent policy authority.

## Cedar: normative primary engine

Cedar is selected because its principal/action/resource/context model matches Styrene authorization and GhostNet OM decisions, its official implementation is an embeddable Rust crate, and its schemas support validation of the authorization model before activation.

Cedar owns runtime evaluation of bounded authorization questions such as:

- whether an identity may request an OM activation in a scope;
- whether an OM role may request a GhostNet workflow transition;
- whether an audience may receive a domain-defined projection;
- whether contextual confirmation or organizational restrictions apply.

Cedar does not own:

- artifact canonicalization or signatures;
- incident and report state-machine semantics;
- transport, messaging, tunnels, routing, or receipts;
- projection construction;
- private key operations;
- arbitrary workflow execution.

OM authors normally use a higher-level typed manifest. The OM compiler validates inheritance, parameter bounds, relaxation rules, workflows, and disclosure flow, then emits Cedar policy, Cedar schemas, typed obligations, and canonical effective configuration.

## Regorus: compatibility engine

Regorus is the supported in-process compatibility path for organizations with existing Rego v1 policy. It is optional and never required to activate a Cedar-native OM.

Compatibility policy must:

- use a versioned allowlist of supported Rego features and built-ins;
- consume the same canonical Styrene policy input contract;
- return the same typed engine-result contract;
- carry source, engine, bundle, and policy hashes in provenance;
- fail closed on unsupported built-ins, undefined decisions, or evaluation errors;
- pass differential compatibility vectors against the declared upstream OPA/Rego version before bundle signing.

A Rego result composes monotonically with compiled invariants and Cedar. It may restrict a permit or require confirmation; it may not relax a deny, resolve indeterminate input to permit, bypass Styrene authorization, or inflate enforcement evidence.

## WebAssembly: bounded exceptional helper

WASM is the rapid-response helper for strange boundary conditions, legacy integration, and short-lived transformation or predicate needs where Cedar and the typed OM vocabulary are insufficient.

It is not a generic escape hatch. A helper must declare:

- a stable helper ID and version;
- input and output schema IDs;
- module SHA-256 hash and publisher signature;
- requested host capabilities;
- fuel/instruction, memory, output, and wall-time limits;
- determinism and replay expectations;
- expiry or review date;
- the OM and scopes permitted to invoke it.

The initial host capability set is empty except for deterministic input/output. No filesystem, network, environment, clock, randomness, process spawning, private keys, or direct Styrene messaging/tunnel access is available. Required time and capability facts arrive through explicit input.

WASM helpers may:

- normalize a legacy external representation into a typed candidate artifact;
- derive a bounded predicate from explicit input;
- perform a reviewed compatibility translation;
- further restrict an operation.

WASM helpers may not:

- issue Styrene operations;
- make an authoritative identity or authorization decision;
- generate or access signing keys;
- relax compiled, Cedar, or Rego restrictions;
- mutate an active OM or activation epoch;
- persist hidden state;
- claim stronger enforcement evidence.

Failure, timeout, trap, schema mismatch, unknown output, or expired helper authority produces an indeterminate or deny result according to the invoking invariant. It never defaults to permit.

## Composition

The common restriction order is:

```text
Permit < RequireConfirmation < Indeterminate < Deny
```

The effective result is the maximum restriction produced by:

1. compiled Rust invariants;
2. Styrene substrate authorization;
3. Cedar policy;
4. optional Regorus compatibility policy;
5. optional bounded WASM helper;
6. unsatisfied typed obligations.

Every participating layer is recorded. A layer that did not run is distinguishable from a layer that returned permit.

## Typed obligations

Policy engines do not return arbitrary executable instructions. They select from a Styrene-owned obligation vocabulary, for example:

- require confirmation from an authorized role;
- narrow an audience selector;
- require a confidentiality class;
- apply a maximum expiry;
- record a named audit class;
- pin an action to an OM activation epoch;
- select an already-defined GhostNet projection profile.

The execution path either proves every obligation satisfied or refuses the action.

## Crate direction

The intended Styrene workspace decomposition is:

```text
styrene-policy
  request/decision types, obligations, composition,
  signed bundles, trust, activation epochs, audit provenance

styrene-policy-cedar
  cedar-policy integration, schema validation, evaluation,
  determining-policy diagnostics

styrene-policy-rego
  Regorus integration and OPA compatibility vectors

styrene-policy-wasm
  sandboxed helper ABI, capability denial, resource limits,
  schema validation and signed-module provenance
```

GhostNet supplies OM manifests, domain schemas, compiled doctrine predicates, and OM-specific scenarios. These policy crates remain independent of GhostNet.

## Versioning and upgrades

Every activated effective configuration records:

- policy request schema version;
- Cedar language and crate versions;
- Regorus and declared OPA/Rego compatibility versions when used;
- WASM runtime/ABI and module hashes when used;
- compiled invariant version;
- OM bundle and deployment-binding hashes;
- activation epoch.

Engine upgrades require the existing golden decision suite plus OM scenario vectors to produce equivalent decisions, obligations, and determining-policy provenance unless an explicit migration declares the difference.

## Rejected alternatives

### OPA daemon as primary

Rejected because a mandatory Go sidecar would violate the single-instance, offline-capable Styrene operator model.

### Rego/Regorus as primary

Rejected because Rego's general document-query model is broader and harder to constrain than the authorization-focused Cedar model. Regorus remains valuable for compatibility.

### Rego compiled to WASM as standard runtime

Rejected as the default because it adds a separate OPA compilation pipeline and runtime ABI without improving Cedar-native authorization. It remains possible inside the compatibility path if future evidence justifies it.

### Raw WASM as policy language

Rejected because WASM supplies isolation, not policy semantics, analyzability, or safe authoring. It is deliberately limited to exceptional helpers.

### Custom runtime policy DSL

Rejected because Styrene would inherit responsibility for a parser, type system, evaluator, diagnostics, formal semantics, and security lifecycle. The GhostNet OM manifest is domain-specific, but it compiles to established policy engines.
