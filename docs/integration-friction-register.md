# GhostNet Operating Modality Integration Friction Register

## Active risks

| ID | Risk | Impact | Mitigation | Gate |
|---|---|---|---|---|
| F-01 | OM becomes a second application or inbox | Split identity and communication truth | One Styrene surface and command registry; reject direct egress facades | Architecture review |
| F-02 | Policy approves intent but final bytes change | Disclosure bypass | Hash final bytes/metadata and bind a single-use permit | Mutation tests |
| F-03 | Cedar/Regorus/helper semantics diverge | Inconsistent authorization | Cedar normative; monotonic composition; shared vectors | Differential tests |
| F-04 | Bundle rollback or downgrade | Reintroduces vulnerable rules | Signed monotonic version policy and explicit break-glass audit | Activation tests |
| F-05 | Mode switch races queued work | Wrong policy epoch | Epoch-bound permits and release-time reauthorization | Race tests |
| F-06 | Projection leaks omitted source content | Disclosure failure | Typed derivative schemas, source hashes, omission metadata, noninterference tests | Projection suite |
| F-07 | Adapter bypasses Styrene policy | Alternate egress path | Sealed-operation-only sandbox with exact bytes and audience | Sandbox tests |
| F-08 | Policy audit leaks message plaintext | Secondary disclosure | Hash-first provenance, field minimization, redaction and retention policy | Audit review |
| F-09 | Unknown/stale membership permits output | Authorization failure | Fail closed; bind membership version and revalidate delayed release | Negative tests |
| F-10 | OM helper is nondeterministic or unbounded | Availability and replay failure | Pure ABI, no ambient I/O, fuel/memory/time bounds | Runtime conformance |
| F-11 | Existing GhostNet crates encode old ownership | Architectural drift | Remove `ghostnet-styrene`; reassess store/adapter/CLI before further feature work | Workspace review |
| F-12 | Old design documents are mistaken for authority | Conflicting implementation | Canonical charter and current plan; legacy file explicitly marked | Documentation check |

## Release evidence

- signed bundle and publisher verification;
- activation epoch and effective configuration hash;
- engine/version and policy bundle provenance;
- hostile exact-byte, race, timeout, trap, rollback, and adapter tests;
- one-identity mode-switch demonstration;
- architecture-boundary script and full workspace validation.
