#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

run_step() {
  local name="$1"
  shift
  echo "== $name =="
  "$@"
}

run_step "rustfmt" cargo fmt --manifest-path "$ROOT/Cargo.toml" --check
run_step "clippy" cargo clippy --manifest-path "$ROOT/Cargo.toml" --locked --quiet -- -D warnings
run_step "tests" cargo test --manifest-path "$ROOT/Cargo.toml" --locked --quiet

if [[ "${TEXPILOT_VERIFY_SKIP_PARITY:-0}" == "1" ]]; then
  echo "== pdflatex parity =="
  echo "skipped because TEXPILOT_VERIFY_SKIP_PARITY=1"
else
  run_step "pdflatex parity" "$ROOT/scripts/verify_pdflatex_parity.sh"
fi

if [[ "${TEXPILOT_VERIFY_SKIP_PERF:-0}" == "1" ]]; then
  echo "== latexmk performance gate =="
  echo "skipped because TEXPILOT_VERIFY_SKIP_PERF=1"
else
  run_step "latexmk performance gate" "$ROOT/scripts/performance_gate.sh"
fi

echo "all release verification checks passed"
