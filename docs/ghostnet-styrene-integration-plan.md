# GhostNet Operating Modality Integration Plan

> **Workspace authority:** This plan implements [`../WORKSPACE-CHARTER.md`](../WORKSPACE-CHARTER.md). The superseded independent-client plan is retained as `ghostnet-styrene-integration-plan.legacy.md` for archaeology only and is not an implementation authority.

## Goal

Integrate GhostNet as signed Operating Modality bundles and deterministic domain contributions inside Styrene's unified command, policy, messaging, tunnel, persistence, and receipt systems.

## Phase 0 — retain reusable semantics

- keep canonical artifact encoding, hashes, schemas, golden vectors, incident/net/report doctrine, UTC windows, and projection tests;
- remove application-owned Styrene communication facades and fake publish/poll clients;
- classify remaining local-store and adapter code as transitional until the generic OM contracts determine ownership;
- keep the architecture guard rejecting private keys, daemon internals, duplicate communications state, and parallel policy/egress facades.

## Phase 1 — generic Styrene OM substrate

Implement upstream, because these capabilities are useful beyond GhostNet:

1. signed OM manifest and bundle format;
2. trust, installation, compatibility, activation, rollback, and epoch persistence;
3. deterministic doctrine/helper ABI with explicit inputs and resource bounds;
4. unified Cedar policy schema, evaluator, typed obligations, provenance, and explanation;
5. optional constrained Regorus compatibility and bounded helper ABI;
6. semantic command/view registration shared by supported surfaces;
7. exact-byte sealed-operation gate covering chat, tunnels, attachments, automation, delayed release, and adapters;
8. activation and queue-release reauthorization;
9. append-only audit evidence with redaction and retention controls.

## Phase 2 — GhostNet OM bundle

- define GhostNet family/profile manifests;
- package doctrine version, schemas, Cedar policies, obligations, projections, commands, views, and scenarios;
- implement example profiles such as `NYC_EMS1_FIRE`, `NYC_EMS1_MED`, `NYC_EMS1_PD`, and `NYC_EMS1_CIV` without changing identity;
- bind deployment-specific roles and resources during activation;
- present active-mode context in ordinary Styrene chat and tunnel surfaces without creating a second inbox or send path.

## Phase 3 — hostile and compatibility testing

- corrupted, expired, untrusted, incompatible, and rollback bundles;
- policy timeout, trap, indeterminate authorization, stale membership, and epoch races;
- exact-byte mutation, retargeting, attachment replacement, queue-release posture change, and duplicate execution;
- projection noninterference and omission-metadata checks;
- cross-engine scenario vectors for Cedar and any Regorus compatibility profile;
- mode switching while messages and tunnels are active;
- crash recovery and rollback to the last known-good activation epoch.

## Merge gates

No implementation is complete until:

- one Styrene identity and communication state survive mode switches;
- every output path crosses one final exact-byte policy gate;
- no GhostNet-owned publish, sign, poll, queue, receipt, or tunnel path exists;
- unsupported required capabilities prevent activation;
- `Deny`, `Indeterminate`, engine failure, and stale facts cannot enqueue;
- the active bundle, engine, policy, fact, operation, and epoch hashes are auditable;
- doctrine and policy scenarios pass under constrained deployment targets.
