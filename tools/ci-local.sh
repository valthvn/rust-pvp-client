#!/usr/bin/env bash
# Local CI: the checks ci.yml runs, scoped to what this repository changes (our pvp-* crates and the
# client executable that links them), sized for a 4-core, 8 GB machine. Horizon's own code is not
# rebuilt or re-tested here: we don't modify it (its upstream CI covers it).
#
#   tools/ci-local.sh          # fmt, clippy -D warnings, tests, client build
#   tools/ci-local.sh --quick  # without the client build
#
# Writes .local/ci/<commit>.log and prints one summary line; exit status 0 only when every step passed.
set -uo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root/engine"
crates=(pvp-core pvp-ui pvp-modules pvp-client)
pkgs=(); for c in "${crates[@]}"; do pkgs+=(-p "$c"); done
commit=$(git rev-parse --short HEAD)
dirty=$(git status --porcelain | grep -q . && echo "+dirty" || true)
mkdir -p .local/ci
log=".local/ci/${commit}${dirty}.log"
: > "$log"
# Two jobs keep the peak memory of rustc + the linker within 8 GB.
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-2} CARGO_TERM_COLOR=never
status=0
step() {
  local name=$1; shift
  local t0=$SECONDS
  echo "== $name: $*" >> "$log"
  if "$@" >> "$log" 2>&1; then
    echo "ok   $name ($((SECONDS - t0)) s)" | tee -a "$log"
  else
    echo "FAIL $name ($((SECONDS - t0)) s) — see $log" | tee -a "$log"
    status=1
  fi
}
step fmt cargo fmt "${pkgs[@]}" --check
step clippy cargo clippy --locked "${pkgs[@]}" --all-targets -- -D warnings
step test cargo test --locked "${pkgs[@]}"
if [[ "${1:-}" != --quick ]]; then
  step build cargo build --locked -p pvp-client
fi
summary="ci-local ${commit}${dirty}: $([[ $status == 0 ]] && echo PASS || echo FAIL) ($(date -u +%Y-%m-%dT%H:%MZ))"
echo "$summary" | tee -a "$log"
exit $status
