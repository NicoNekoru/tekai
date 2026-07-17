# texpilot

`texpilot` is a Rust CLI for fast, fidelity-preserving LaTeX builds, live
previews, dependency-aware caching, and opinionated TeX linting.

The default `pdf-latex` + `direct` path runs the workspace's Rust-owned pdfTeX
port inside the `texpilot` executable. It retains pdfTeX's typesetting behavior
while replacing `latexmk` orchestration with explicit convergence, bibliography,
index, external-tool, and cache handling. The two bundled large papers render
pixel-identically to system pdfTeX at the repository's 144 DPI parity gate.

## Quick start

Requirements:

- a current stable Rust toolchain;
- a TeX Live installation for LaTeX packages, fonts, and Kpathsea databases;
- any auxiliary programs used by the document, such as BibTeX, Biber,
  MakeIndex, `bib2gls`, or `pythontex`.

Build the release binary and compile a paper:

```sh
cargo build --release --locked
target/release/texpilot build path/to/main.tex
```

During development, `cargo run --` can replace `target/release/texpilot`:

```sh
cargo run -- build examples/minimal.tex
```

Common workflows:

```sh
# Lint, then build only if lint passes.
target/release/texpilot check path/to/main.tex --allow-warnings

# Fast live preview; produce an exact final build after 1.5 seconds of idle time.
target/release/texpilot watch path/to/main.tex \
  --preview --final-after-idle-ms 1500 --allow-warnings

# Emit a machine-readable build report.
target/release/texpilot build path/to/main.tex --report-json

# Inspect and remove the configured output directory safely.
target/release/texpilot clean --dry-run
target/release/texpilot clean
```

See [Usage and configuration](docs/usage.md) for the full command, engine,
preview, cache, configuration, and lint reference.

## Build modes

| Mode | Purpose |
| --- | --- |
| `--engine pdf-latex --runner direct` | Default exact pdfTeX build through the embedded `pdftex-rust` engine and native scheduler. |
| `--runner latexmk` | System `latexmk` baseline or compatibility escape hatch. |
| `--engine xe-latex` / `lua-latex` / `tectonic` | Installed external engine with the selected runner's supported orchestration. |
| `--engine texpilot-pdftex` | Experimental approximate native renderer; unsupported inputs fall back to the exact pdfTeX path. |
| `--engine texpilot-pdftex-certified` | Run native diagnostics, then deliver the exact pdfTeX artifact. |

`--fast` changes visible output by replacing expensive graphics and externalized
content with preview placeholders. `--once` intentionally skips convergence.
Use neither for a final artifact. `watch --preview` combines both for the live
path and conservatively falls back to a full preview for structural edits.

## What the direct runner handles

The direct runner owns the work normally delegated to `latexmk`:

- TeX rerun detection and bounded convergence;
- BibTeX and Biber discovery, freshness checks, and reusable outputs;
- MakeIndex, Xindy, glossaries, nomenclature, and split-index workflows;
- common source-driven external tools, including SVG/EPS conversion,
  Asymptote, MetaPost, Gnuplot, PythonTeX, minted, and PGF externalization;
- recorder- and source-derived dependency tracking;
- settled build-state and preamble-format caches;
- JSON reports for editors, benchmarks, and CI.

If the requested workflow needs a program that is not installed, the build
reports that missing external dependency instead of silently changing output.

## Configuration

Projects may check in `texpilot.toml`. CLI arguments explicitly supplied by the
user override config values.

```toml
[build]
engine = "pdflatex"
runner = "direct"
bib = "auto"
out_dir = "build"
draft_prepass = "auto"
max_runs = 8

[build.env]
TEXINPUTS = "tex//:"
BIBINPUTS = "bib//:"

[lint]
indent_size = 2
max_line_length = 120

[lint.rules]
"math/inline-dollar" = "error"
"line/length" = "warn"
```

The checked-in [texpilot.toml](texpilot.toml) is a complete starting point.

## Fidelity contract

For the default embedded pdfTeX path, visible PDF output is the correctness
gate. PDF bytes may differ because object ordering, timestamps, identifiers, or
compression differ; fixed-DPI page pixels and extracted document text must not.

The exact port and the experimental renderer are intentionally separate:

- [`crates/pdftex-rust`](crates/pdftex-rust/README.md) is the faithful default
  pdfTeX implementation.
- [`crates/texpilot-pdftex`](crates/texpilot-pdftex/README.md) is the native
  replacement-engine research track.

Current evidence and known limits are recorded in:

- [pdfTeX Rust parity report](output/pdf/pdftex-rust-parity-report.md)
- [experimental native-renderer divergence audit](output/pdf/pdftex-native-divergence-audit.md)

## Repository guide

| Path | Responsibility |
| --- | --- |
| `src/compiler.rs` | Direct build scheduler, caches, dependencies, auxiliary tools, and engine dispatch. |
| `src/watch.rs` | Watch filtering, live preview, debounce, and structural fallbacks. |
| `src/lint.rs` | Single-pass TeX-aware linting and suppression handling. |
| `crates/pdftex-rust` | Rust-owned exact pdfTeX engine. |
| `crates/texpilot-pdftex` | Experimental native renderer and next-engine architecture. |
| `tests` | CLI and document-workflow integration tests. |
| `examples` | Minimal, BibLaTeX, and two large real-paper fixtures. |

For contributor workflows, test gates, performance measurement, and PDF parity
procedure, see [Development](docs/development.md).

## Verification

The normal local gate is:

```sh
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --locked
```

Some integration tests require the corresponding TeX auxiliary program and
skip when it is unavailable. CI also checks the standalone `pdftex-rust` binary
feature set; the exact commands are documented in
[docs/development.md](docs/development.md).
