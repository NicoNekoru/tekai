# pdfTeX Rust Substitute Parity Report

Generated: 2026-07-05

## Summary

The `texpilot-pdftex` engine path now runs the Rust `pdftex-rust` binary for normal direct builds instead of the simplified native renderer. On the two large arXiv examples, the Rust engine produced rendered output identical to the pdfTeX reference at 144 DPI.

## Targets

| Case | Source | Reference pages | Rust pages | Changed rendered pages | Changed pixels | Max RMS |
|---|---|---:|---:|---:|---:|---:|
| arxiv-2605 | `examples/arXiv-2605.26379v1/main.tex` | 48 | 48 | 0 | 0 / 93,063,168 | 0.000000 |
| arxiv-2511 | `examples/arXiv-2511.08544v3/main.tex` | 50 | 50 | 0 | 0 / 96,940,800 | 0.000000 |

## Build Timings

| Case | Engine | Elapsed ms | TeX runs | PDF-producing TeX runs | BibTeX runs |
|---|---|---:|---:|---:|---:|
| arxiv-2605 | pdfTeX reference | 4952.723 | 3 | 1 | 1 |
| arxiv-2605 | Rust pdfTeX substitute | 4040.741 | 3 | 1 | 1 |
| arxiv-2511 | pdfTeX reference | 6605.574 | 2 | 1 | 1 |
| arxiv-2511 | Rust pdfTeX substitute | 4962.763 | 2 | 1 | 1 |

## PDF Sanity

| Case | Engine | Pages | Page size | PDF version | File size |
|---|---|---:|---|---|---:|
| arxiv-2605 | pdfTeX reference | 48 | 612 x 792 pts | 1.7 | 6,933,114 bytes |
| arxiv-2605 | Rust pdfTeX substitute | 48 | 612 x 792 pts | 1.7 | 6,878,298 bytes |
| arxiv-2511 | pdfTeX reference | 50 | 612 x 792 pts | 1.7 | 8,241,113 bytes |
| arxiv-2511 | Rust pdfTeX substitute | 50 | 612 x 792 pts | 1.7 | 12,260,512 bytes |

The PDFs are not byte-identical, but their rendered PNG pages are byte-identical. The remaining byte-level differences are outside visible PDF output, such as object ordering, IDs, timestamps, and binary packing.

## Commands

```bash
cargo test --test compiler_texpilot_pdftex
cargo test -p texpilot-pdftex icml_line_breaking_uses_active_body_metric
cargo test -p texpilot-pdftex native_neurips_pdf_embeds_nimbus_type1_fonts_when_available
cargo build --release
cargo build --release -p pdftex-rust --bin pdftex-rust --no-default-features --features rust-binary
```

Rendered comparison used Poppler:

```bash
pdftoppm -r 144 -png reference.pdf reference/page
pdftoppm -r 144 -png rust.pdf rust/page
```
