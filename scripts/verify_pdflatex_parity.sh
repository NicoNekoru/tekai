#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKDIR="${TEXPILOT_PARITY_WORKDIR:-/tmp/texpilot-pdflatex-parity}"
REQUESTED_CASES="${TEXPILOT_PARITY_CASES:-all}"
PDFTOPPM="${PDFTOPPM:-pdftoppm}"

CASES=(
  "iclr|https://github.com/ICLR/Master-Template/raw/master/iclr2026.zip|iclr2026/iclr2026_conference.tex"
  "neurips|https://media.neurips.cc/Conferences/NeurIPS2026/Formatting_Instructions_For_NeurIPS_2026.zip|neurips_2026.tex"
  "icml|https://media.icml.cc/Conferences/ICML2026/Styles/icml2026.zip|example_paper.tex"
)

LOCAL_CASES=(
  "biblatex-biber|$ROOT/examples/biblatex-biber|main.tex"
  "arxiv-2511|$ROOT/examples/arXiv-2511.08544v3|main.tex"
  "arxiv-2605|$ROOT/examples/arXiv-2605.26379v1|main.tex"
)

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 127
  fi
}

need cargo
need curl
need unzip
need pdflatex
need bibtex
need "$PDFTOPPM"
need shasum
need diff

case_requested() {
  local name="$1"
  local requested token
  requested="${REQUESTED_CASES//,/ }"

  if [[ -z "$requested" || "$requested" == "all" ]]; then
    return 0
  fi

  for token in $requested; do
    if [[ "$token" == "$name" ]]; then
      return 0
    fi
  done
  return 1
}

case_name_known() {
  local wanted="$1"
  local spec name ignored1 ignored2
  for spec in "${CASES[@]}" "${LOCAL_CASES[@]}"; do
    IFS='|' read -r name ignored1 ignored2 <<<"$spec"
    if [[ "$name" == "$wanted" ]]; then
      return 0
    fi
  done
  return 1
}

validate_case_filter() {
  local requested token
  requested="${REQUESTED_CASES//,/ }"

  if [[ -z "$requested" || "$requested" == "all" ]]; then
    return 0
  fi

  for token in $requested; do
    if ! case_name_known "$token"; then
      echo "unknown parity case: $token" >&2
      echo "known cases: iclr neurips icml biblatex-biber arxiv-2511 arxiv-2605" >&2
      exit 2
    fi
  done
}

validate_case_filter

rm -rf "$WORKDIR"
mkdir -p "$WORKDIR"

cargo build --manifest-path "$ROOT/Cargo.toml" --locked >/dev/null
TEXPILOT="$ROOT/target/debug/texpilot"

copy_tree() {
  local src="$1"
  local dest="$2"
  mkdir -p "$dest"
  (cd "$src" && tar cf - .) | (cd "$dest" && tar xf -)
}

job_name_for() {
  local main="$1"
  basename "$main" .tex
}

line_requests_rerun() {
  local line lower
  line="$1"
  lower="$(printf '%s' "$line" | LC_ALL=C tr '[:upper:]' '[:lower:]')"
  [[ "$lower" == *"rerun to get"* ]] ||
    [[ "$lower" == *"rerun latex"* ]] ||
    [[ "$lower" == *"rerun lualatex"* ]] ||
    [[ "$lower" == *"rerun xelatex"* ]] ||
    [[ "$lower" == *"label(s) may have changed"* ]] ||
    [[ "$lower" == *"reference(s) may have changed"* ]] ||
    [[ "$lower" == *"citation(s) may have changed"* ]] ||
    { [[ "$lower" == *"file \`"* ]] && [[ "$lower" == *"' has changed"* ]]; } ||
    { [[ "$lower" == *"file \""* ]] && [[ "$lower" == *"\" has changed"* ]]; }
}

needs_rerun() {
  local log="$1"
  [[ -f "$log" ]] || return 1
  while IFS= read -r line; do
    if line_requests_rerun "$line"; then
      return 0
    fi
  done <"$log"
  return 1
}

run_baseline_bibliography_tools() {
  local job="$1"
  local aux bib_job

  if [[ -f "$job.bcf" ]]; then
    need biber
    biber "$job" >/dev/null
  fi

  while IFS= read -r -d '' aux; do
    if grep -q '\\bibdata' "$aux"; then
      bib_job="${aux#./}"
      bib_job="${bib_job%.aux}"
      bibtex "$bib_job" >/dev/null
    fi
  done < <(find . -type f -name '*.aux' -print0)
}

run_pdflatex_baseline() {
  local dir="$1"
  local main="$2"
  local job
  job="$(job_name_for "$main")"

  (
    cd "$dir"
    pdflatex -interaction=nonstopmode -halt-on-error -file-line-error "$main" >/dev/null
    run_baseline_bibliography_tools "$job"
    for _ in 1 2 3 4 5 6 7 8; do
      pdflatex -interaction=nonstopmode -halt-on-error -file-line-error "$main" >/dev/null
      if ! needs_rerun "$job.log"; then
        return 0
      fi
    done
    echo "pdflatex baseline did not converge for $main" >&2
    return 1
  )
}

render_hashes() {
  local pdf="$1"
  local outdir="$2"
  rm -rf "$outdir"
  mkdir -p "$outdir"
  "$PDFTOPPM" -r 144 -png "$pdf" "$outdir/page" >/dev/null
  find "$outdir" -type f -name '*.png' -print0 | sort -z | xargs -0 shasum
}

compare_pdfs() {
  local expected="$1"
  local actual="$2"
  local case_dir="$3"

  render_hashes "$expected" "$case_dir/expected-pages" >"$case_dir/expected.sha"
  render_hashes "$actual" "$case_dir/actual-pages" >"$case_dir/actual.sha"
  sed -E 's#  .*/#  #' "$case_dir/expected.sha" >"$case_dir/expected.norm.sha"
  sed -E 's#  .*/#  #' "$case_dir/actual.sha" >"$case_dir/actual.norm.sha"
  diff -u "$case_dir/expected.norm.sha" "$case_dir/actual.norm.sha"
}

selected_count=0

for spec in "${CASES[@]}"; do
  IFS='|' read -r name url main_rel <<<"$spec"
  if ! case_requested "$name"; then
    continue
  fi
  selected_count=$((selected_count + 1))
  echo "== $name =="

  case_dir="$WORKDIR/$name"
  archive="$case_dir/source.zip"
  unpacked="$case_dir/source"
  baseline="$case_dir/baseline"
  actual="$case_dir/actual"

  mkdir -p "$case_dir" "$unpacked"
  curl -fsSL "$url" -o "$archive"
  unzip -q "$archive" -d "$unpacked"

  main_parent="$(dirname "$main_rel")"
  main_file="$(basename "$main_rel")"
  if [[ "$main_parent" == "." ]]; then
    source_dir="$unpacked"
  else
    source_dir="$unpacked/$main_parent"
  fi

  copy_tree "$source_dir" "$baseline"
  copy_tree "$source_dir" "$actual"

  run_pdflatex_baseline "$baseline" "$main_file"
  "$TEXPILOT" build "$actual/$main_file" --out-dir "$actual/texpilot-out" --quiet >/dev/null

  job="$(job_name_for "$main_file")"
  compare_pdfs "$baseline/$job.pdf" "$actual/texpilot-out/$job.pdf" "$case_dir"
  echo "matched $name rendered PDF pages"
done

for spec in "${LOCAL_CASES[@]}"; do
  IFS='|' read -r name source_dir main_file <<<"$spec"
  if ! case_requested "$name"; then
    continue
  fi
  selected_count=$((selected_count + 1))
  echo "== $name =="

  case_dir="$WORKDIR/$name"
  baseline="$case_dir/baseline"
  actual="$case_dir/actual"

  mkdir -p "$case_dir"
  copy_tree "$source_dir" "$baseline"
  copy_tree "$source_dir" "$actual"

  run_pdflatex_baseline "$baseline" "$main_file"
  "$TEXPILOT" build "$actual/$main_file" --out-dir "$actual/texpilot-out" --quiet >/dev/null

  job="$(job_name_for "$main_file")"
  compare_pdfs "$baseline/$job.pdf" "$actual/texpilot-out/$job.pdf" "$case_dir"
  echo "matched $name rendered PDF pages"
done

if [[ "$selected_count" -eq 0 ]]; then
  echo "no pdflatex parity cases selected" >&2
  exit 2
fi

echo "all pdflatex parity checks passed"
