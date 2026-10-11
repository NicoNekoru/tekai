# tekai-pdftex

`tekai-pdftex` is the experimental native TeX/STY-to-PDF replacement-engine
track for the `tekai` workspace.

It is separate from [`tekai-engine`](../tekai-engine/README.md).

- `tekai-engine` preserves pdfTeX's algorithms and powers the default exact
  `--engine tekai-engine --runner direct` path.
- `tekai-pdftex` explores a faster fused expansion, in-memory document,
  layout, asset, and PDF architecture.

## CLI modes

```sh
cargo run -- build path/to/main.tex --engine tekai-pdftex
cargo run -- build path/to/main.tex --engine tekai-pdftex-certified
```

The legacy `--engine tekai-pdftex` CLI mode selects the exact standalone
`tekai-engine` executable. It does not run this crate's approximate renderer.

`tekai-pdftex-certified` runs the native backend for trace/coverage evidence,
then produces the delivered PDF with installed pdfLaTeX. The approximate
renderer does not establish general page parity.

## Correctness boundary

The target is a high-performance TeX/STY-to-PDF engine, not a preview-only
renderer or a source-level LaTeX parser. Features belong on the native hot path
when they improve executed TeX semantics or visible PDF fidelity. Reproducing
legacy `.aux`, `.log`, `.fls`, bibliography, or SyncTeX file shape is optional
unless it affects the delivered PDF or required interoperability.

The native renderer already supports the two checked-in large papers as a
useful development corpus, but it still substitutes simplified font, math,
paragraph, page-building, float, environment, and bibliography behavior. The
systematic differences are documented in the
[historical divergence audit](../../docs/archive/pdftex-native-divergence-audit.md).

## Architecture

The active code is `expand.rs`, `native.rs`, token handling and tracing.
The [archived proposal](../../docs/archive/native-engine-proposal-2026-10.md)
records a possible future design. Proposal-only contract, roadmap and report
modules have been retired. Their own constant-list tests did not verify the
engine that ships.

## Development

```sh
cargo test -p tekai-pdftex
cargo test --test compiler compiler_tekai_pdftex
cargo clippy -p tekai-pdftex --all-targets --locked -- -D warnings
```

Any change presented as a fidelity improvement should include fixed-DPI
rendered comparison against the exact engine. Any change presented as a
performance improvement should include matched release-mode timings on the
large examples and name the unsupported/fallback boundary.
