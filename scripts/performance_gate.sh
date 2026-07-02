#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUNS="${TEXPILOT_PERF_RUNS:-3}"
MAX_RATIO="${TEXPILOT_MAX_LATEXMK_RATIO:-1.0}"
TOLERANCE="${TEXPILOT_GATE_ABSOLUTE_TOLERANCE:-0.05}"
PROFILE="${TEXPILOT_PERF_PROFILE:-release}"
PAPERS="${TEXPILOT_PERF_PAPERS:-examples/arXiv-2605.26379v1/main.tex examples/arXiv-2511.08544v3/main.tex}"
SCENARIOS="${TEXPILOT_PERF_SCENARIOS:-clean warm-edits}"

texpilot_args=()
if [[ -n "${TEXPILOT_BIN:-}" ]]; then
  texpilot_args=(--texpilot "$TEXPILOT_BIN")
else
  texpilot_args=(--profile "$PROFILE")
fi

read -r -a papers <<< "$PAPERS"
read -r -a scenarios <<< "$SCENARIOS"

for paper in "${papers[@]}"; do
  for scenario in "${scenarios[@]}"; do
    "$ROOT/scripts/benchmark_paper.py" "$paper" \
      --scenario "$scenario" \
      --runs "$RUNS" \
      --gate \
      --max-latexmk-ratio "$MAX_RATIO" \
      --gate-absolute-tolerance "$TOLERANCE" \
      "${texpilot_args[@]}"
  done
done
