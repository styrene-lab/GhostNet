#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

failures=0

fail() {
  printf 'BOUNDARY VIOLATION: %s\n' "$1" >&2
  failures=$((failures + 1))
}

require_text() {
  local file="$1"
  local pattern="$2"
  local description="$3"
  if ! grep -Eq "$pattern" "$file"; then
    fail "$description ($file)"
  fi
}

scan_production_sources() {
  local pattern="$1"
  local description="$2"
  local output

  local roots=()
  [[ -d crates ]] && roots+=(crates)
  [[ -d src ]] && roots+=(src)
  [[ -d adapters ]] && roots+=(adapters)

  if (( ${#roots[@]} == 0 )); then
    return
  fi

  output="$(
    find "${roots[@]}" \
      -type f \( -name '*.rs' -o -name 'Cargo.toml' \) \
      ! -path '*/tests/*' \
      ! -name '*_test.rs' \
      ! -name '*.test.rs' \
      -print0 2>/dev/null \
      | xargs -0 grep -En "$pattern" 2>/dev/null || true
  )"

  if [[ -n "$output" ]]; then
    fail "$description"
    printf '%s\n' "$output" >&2
  fi
}

require_text WORKSPACE-CHARTER.md '^\*\*Canonical and binding for this workspace\.\*\*' \
  'workspace charter must declare canonical status'
require_text README.md 'WORKSPACE-CHARTER\.md' \
  'README must link the workspace charter'
require_text docs/ghostnet-policy-client-overlay.md 'WORKSPACE-CHARTER\.md' \
  'operating modality architecture must link the workspace charter'
require_text docs/ghostnet-styrene-integration-plan.md 'WORKSPACE-CHARTER\.md' \
  'operating modality integration plan must link the workspace charter'

for schema in net-definition incident-transition operational-report detached-signed-envelope projection adapter-capability operating-modality-profile; do
  if [[ ! -f "schemas/v1/${schema}-v1.schema.json" ]]; then
    fail "required Phase 0 schema is missing: ${schema}-v1"
  fi
done

# GhostNet production code may use released public Styrene extension crates, but
# it must not import daemon internals, reach into a sibling checkout, or expose
# a second communications execution facade.
scan_production_sources \
  '(^|[^[:alnum:]_])(styrened::|use[[:space:]]+styrened|extern[[:space:]]+crate[[:space:]]+styrened)' \
  'production GhostNet source imports styrened internals'
scan_production_sources \
  'path[[:space:]]*=[[:space:]]*"[^"]*(styrene-rs|styrene-rns|styrened)' \
  'production GhostNet manifest uses a Styrene source-tree path dependency'
scan_production_sources \
  '(ed25519_dalek::SigningKey|x25519_dalek::StaticSecret|PrivateKey|SecretKey)' \
  'production GhostNet source handles private signing/key material'
scan_production_sources \
  '(CREATE[[:space:]]+TABLE[^;]*(messages|receipts|propagation|path_table|identities)|struct[[:space:]]+(MeshTransport|NetOpsStore))' \
  'production GhostNet source duplicates Styrene substrate persistence or transport ownership'

scan_production_sources \
  '(trait[[:space:]]+StyrenePort|fn[[:space:]]+(apply_tx_policy)[[:space:]]*\(|async[[:space:]]+fn[[:space:]]+(apply_tx_policy)[[:space:]]*\()' \
  'production GhostNet source exposes a parallel Styrene communications facade'

if find . -path './.git' -prune -o -type d -name 'styrene-netops' -print | grep -q .; then
  fail 'styrene-netops belongs neither in GhostNet nor Styrene proper; use ghostnet-doctrine'
fi

if grep -RInE \
  --exclude='WORKSPACE-CHARTER.md' \
  --exclude='check-architecture-boundary.sh' \
  --exclude-dir='.git' \
  --exclude-dir='.flynt' \
  --exclude-dir='.omegon' \
  '(client[- ](only|guarded).*(machine[- ]enforced|zero RF)|machine[- ]enforced.*client[- ](only|guarded))' \
  . >/tmp/ghostnet-boundary-wording.$$ 2>/dev/null; then
  fail 'documentation conflates client-guarded suppression with machine-enforced silence'
  cat /tmp/ghostnet-boundary-wording.$$ >&2
fi
rm -f /tmp/ghostnet-boundary-wording.$$

if (( failures > 0 )); then
  printf '\nArchitecture boundary check failed with %d violation(s).\n' "$failures" >&2
  exit 1
fi

printf 'Architecture boundary check passed.\n'
printf '  GhostNet: signed OM profiles, doctrine, artifacts, projections, workflows and views\n'
printf '  Styrene: identity, authorization, policy, messaging, tunnels, egress, persistence and receipts\n'
