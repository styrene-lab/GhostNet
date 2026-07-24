# GhostNet artifact schemas

These JSON Schema 2020-12 documents define the Phase 0 wire contracts. Schema IDs are immutable: any change that alters accepted canonical bytes requires a new schema ID.

Canonical bodies use the restricted RFC 8785 profile implemented by `ghostnet-doctrine`:

- UTF-8 JSON objects, arrays, strings, booleans, and integers only;
- no floating-point numbers or `null` values;
- integers in the interoperable range `[-(2^53-1), 2^53-1]`;
- object members ordered by UTF-16 code units;
- no insignificant whitespace;
- SHA-256 references encoded as `sha256:` plus 64 lowercase hexadecimal characters.

The Operating Modality profile schema is a GhostNet-owned contribution contract. It identifies domain resources, Cedar policy inputs, commands, views, roles, and required generic Styrene capabilities. It does not define installation, trust, activation, identity, policy execution, or egress behavior; those remain Styrene-owned.

Resource hashes cover the exact file bytes at the declared relative path. Bundle signing and trust envelopes are supplied by the generic Styrene OM substrate rather than duplicated here.

The initial profile set demonstrates three intentionally distinct operating points:

- `GHOSTNET_OPEN_STARTER` adds no OM-specific transmission restriction and exposes only a simple participant workflow. “Open” does not bypass Styrene identity, RBAC, disclosure, destination, or exact-byte safety checks.
- `NYC_EMS1_FIRE` is the higher-stakes operational example with confirmation-sensitive restricted transmission.
- `GHOSTNET_GHOST_RX_ONLY` has no transmit permit and requires `enforcement.hardware-receive-only` at activation. A software-only deny is not represented as physical RF silence.

The signed-envelope schema carries a canonical body as an object. Its `body_hash` covers only the canonical body bytes. Signature input is:

```text
ghostnet-artifact-v1 NUL schema-id NUL canonical-body
```
