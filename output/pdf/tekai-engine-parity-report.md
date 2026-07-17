# Tekai engine parity report

Initial measurement: 2026-07-05; rendered parity revalidated: 2026-07-17.

## Summary

The default `tekai-engine` + `direct` path runs Tekai's self-contained exact engine
rather than the simplified experimental renderer. On the two large arXiv
examples, the Tekai engine produced rendered output identical to the reference
system at 144 DPI.

This is a dated measurement snapshot. Current usage and engine naming are
documented in [`docs/usage.md`](../../docs/usage.md); rerun the parity gate after
output-affecting changes instead of treating the historical timings below as a
permanent benchmark.

## 0.1.0 release gate

On 2026-07-17, both papers were rebuilt from the release binary with isolated
output directories. The reference used `--runner latexmk`; the candidate used
the self-contained direct runner. Extracted text matched, page counts matched,
and every 144 DPI rendered PNG matched byte for byte.

| Case | Pages | Changed rendered pages | Reference elapsed | Tekai elapsed | Tekai TeX runs |
| --- | ---: | ---: | ---: | ---: | ---: |
| arxiv-2605 | 48 | 0 | 14.508 s | 1.653 s | 3 |
| arxiv-2511 | 50 | 0 | 16.379 s | 2.850 s | 2 |

These are single release-gate runs, not a repeated performance benchmark.

## Targets

| Case | Source | Reference pages | Rust pages | Changed rendered pages | Changed pixels | Max RMS |
|---|---|---:|---:|---:|---:|---:|
| arxiv-2605 | `examples/arXiv-2605.26379v1/main.tex` | 48 | 48 | 0 | 0 / 93,063,168 | 0.000000 |
| arxiv-2511 | `examples/arXiv-2511.08544v3/main.tex` | 50 | 50 | 0 | 0 / 96,940,800 | 0.000000 |

## Build Timings

| Case | Engine | Elapsed ms | TeX runs | PDF-producing TeX runs | BibTeX runs |
|---|---|---:|---:|---:|---:|
| arxiv-2605 | pdfTeX reference | 4952.723 | 3 | 1 | 1 |
| arxiv-2605 | Tekai engine | 4040.741 | 3 | 1 | 1 |
| arxiv-2511 | pdfTeX reference | 6605.574 | 2 | 1 | 1 |
| arxiv-2511 | Tekai engine | 4962.763 | 2 | 1 | 1 |

## Resource Usage

On 2026-07-09, the TeX Live `ls-R` file index was changed from duplicated full
paths to interned directories plus compact file links. The comparison used the
same release configuration before and after that isolated change, one exact
embedded pdfTeX pass (`--once --force --quiet`), warmed filesystem caches,
seven alternating trials, and a unique output directory per trial. Values
below are medians from `/usr/bin/time -l` on macOS.

| Case | Metric | Full-path index | Compact index | Change |
|---|---|---:|---:|---:|
| arxiv-2605 | Real time | 0.96 s | 0.91 s | -5.2% |
| arxiv-2605 | User CPU | 0.88 s | 0.85 s | -3.4% |
| arxiv-2605 | Peak RSS | 214,007,808 bytes | 163,069,952 bytes | -23.8% (-48.6 MiB) |
| arxiv-2511 | Real time | 1.78 s | 1.75 s | -1.7% |
| arxiv-2511 | User CPU | 1.66 s | 1.63 s | -1.8% |
| arxiv-2511 | Peak RSS | 229,654,528 bytes | 178,307,072 bytes | -22.4% (-49.0 MiB) |

The compact parser itself took a median 14.1 ms versus 61.7 ms for the previous
parser against TeX Live 2026's 5.3 MiB database. Full builds were then rerun and
all 98 rendered pages remained identical to the system-pdfTeX references at
144 DPI.

## PDF Sanity

| Case | Engine | Pages | Page size | PDF version | File size |
|---|---|---:|---|---|---:|
| arxiv-2605 | pdfTeX reference | 48 | 612 x 792 pts | 1.7 | 6,933,114 bytes |
| arxiv-2605 | Tekai engine | 48 | 612 x 792 pts | 1.7 | 6,878,298 bytes |
| arxiv-2511 | pdfTeX reference | 50 | 612 x 792 pts | 1.7 | 8,241,113 bytes |
| arxiv-2511 | Tekai engine | 50 | 612 x 792 pts | 1.7 | 12,260,512 bytes |

The PDFs are not byte-identical, but their rendered PNG pages are byte-identical. The remaining byte-level differences are outside visible PDF output, such as object ordering, IDs, timestamps, and binary packing.

## Commands

```bash
cargo test --test compiler_tekai_pdftex
cargo test -p tekai-pdftex icml_line_breaking_uses_active_body_metric
cargo test -p tekai-pdftex native_neurips_pdf_embeds_nimbus_type1_fonts_when_available
cargo build --release
cargo build --release -p tekai-engine --bin tekai-engine --no-default-features --features standalone-binary
```

Rendered comparison used Poppler:

```bash
pdftoppm -r 144 -png reference.pdf reference/page
pdftoppm -r 144 -png rust.pdf rust/page
```
