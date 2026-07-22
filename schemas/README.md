# GhostNet artifact schemas

These JSON Schema 2020-12 documents define the Phase 0 wire contracts. Schema IDs are immutable: any change that alters accepted canonical bytes requires a new schema ID.

Canonical bodies use the restricted RFC 8785 profile implemented by `ghostnet-doctrine`:

- UTF-8 JSON objects, arrays, strings, booleans, and integers only;
- no floating-point numbers or `null` values;
- integers in the interoperable range `[-(2^53-1), 2^53-1]`;
- object members ordered by UTF-16 code units;
- no insignificant whitespace;
- SHA-256 references encoded as `sha256:` plus 64 lowercase hexadecimal characters.

The signed-envelope schema carries a canonical body as an object. Its `body_hash` covers only the canonical body bytes. Signature input is:

```text
ghostnet-artifact-v1 NUL schema-id NUL canonical-body
```
