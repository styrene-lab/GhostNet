# GhostNet

> **Internal workspace boundary:** GhostNet is an operational mode within [Styrene Mesh](https://styrene.io). Styrene owns identity, authorization, policy evaluation, messaging, tunnels, transport, routing, persistence, and receipts. GhostNet contributes signed operating-modality profiles, doctrine, workflows, projections, and operator views.
>
> The binding ownership rules for this repository are in **[WORKSPACE-CHARTER.md](WORKSPACE-CHARTER.md)**. Architecture and implementation work must conform to that charter.

<img src="GhostNet_logov1.PNG" align="right" height="50%" width="50%">

## Product direction

GhostNet turns Styrene's resilient communications capabilities into prepared operational modalities: scheduled nets, incident activation, signed situation reports, alerts and requests, disclosure profiles, constrained-link projections, receive-only postures, and operator exercises. Operators activate signed OM profiles inside the same Styrene instance and retain the same portable identity.

GhostNet does **not** implement or wrap a competing MANET, messaging system, tunnel layer, routing stack, communications daemon, identity store, policy substrate, propagation queue, or private-key system. Those substrate concerns remain with Styrene. GhostNet defines domain artifacts and workflows that request Styrene capabilities through public extension contracts.

Start here:

- [Workspace Charter](WORKSPACE-CHARTER.md) — canonical product boundary and ownership matrix
- [GhostNet Operating Modality Architecture](docs/ghostnet-policy-client-overlay.md) — architecture
- [GhostNet Doctrine Kernel](docs/ghostnet-doctrine-kernel.md) — operational semantics
- [Styrene Integration Contract](docs/styrene-integration-contract.md) — public client boundary
- [Layered Policy Architecture](docs/layered-policy-architecture.md) — compiled invariants and unified Styrene policy ownership
- [Policy Engine Selection](docs/policy-engine-selection.md) — Cedar primary, Regorus compatibility, bounded WASM helpers
- [Integration Friction Register](docs/integration-friction-register.md) — known risks and gates
- [Implementation Plan](docs/ghostnet-styrene-integration-plan.md) — phased delivery

## Workspace architecture guardrail

GhostNet is a signed Operating Modality within Styrene; Styrene remains the sole communications and policy-enforcement system. Run this check before committing changes to GhostNet code, manifests, or architecture documents:

```bash
scripts/check-architecture-boundary.sh
```

The check rejects daemon-internal imports, sibling Styrene source-tree dependencies, private-key handling in GhostNet production code, duplicated transport/substrate persistence ownership, and missing links to the binding [Workspace Charter](WORKSPACE-CHARTER.md).

This check is intentionally conservative and complements architectural review; it does not prove that every semantic overlap is absent.

## Source-project description

GhostNet is the overarching term for a collection of communications networks set up to allow users
around the world to exchange information without relying on pre-established infrastructure. Far from
being just an emergency plan, GhostNet is intended to ease the transition of radio technology into everyday
life. Though radio networks cannot truly replace the internet, we hope that we can replace a substantial
portion of a person’s daily information requirements, and promote a culture of off-grid information sharing.

S2 Underground © 2025 by S2A1 is licensed under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/).
