# tekai

[![CI](https://github.com/NicoNekoru/tekai/actions/workflows/ci.yml/badge.svg)](https://github.com/NicoNekoru/tekai/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`tekai` is a self-contained typesetting engine and Rust CLI for fast,
fidelity-preserving LaTeX builds, live previews, dependency-aware caching, and
opinionated TeX linting.

The default `tekai-engine` + `direct` path runs Tekai's exact engine inside the
`tekai` executable. It owns typesetting and explicit convergence, bibliography,
index, external-tool, and cache handling without shelling out to a system
`pdflatex` engine. The two bundled large papers render pixel-identically to the
reference system at the repository's dated 144 DPI parity gate.

## Install

Homebrew on macOS:

```sh
brew install NicoNekoru/tap/tekai
tekai --version
```

The formula builds `tekai` from source. TeX Live or MacTeX remains the package,
font, format-data, and auxiliary-tool distribution; Tekai supplies the engine
and build system.

Tekai 0.1.0 supports macOS. Linux portability work is not part of this release.

To install from a checkout instead:

```sh
cargo install --path . --locked
```

## Quick start

Requirements:

- a TeX Live installation for LaTeX packages, fonts, and Kpathsea databases;
- any auxiliary programs used by the document, such as BibTeX, Biber,
  MakeIndex, `bib2gls`, or `pythontex`.

Compile a paper:

```sh
tekai build path/to/main.tex
```

During development, `cargo run --` can replace `tekai`:

```sh
cargo run -- build examples/minimal.tex
```

Common workflows:

```sh
# Lint, then build only if lint passes.
tekai check path/to/main.tex --allow-warnings

# Fast live preview; produce an exact final build after 1.5 seconds of idle time.
tekai watch path/to/main.tex \
  --preview --final-after-idle-ms 1500 --allow-warnings

# Emit a machine-readable build report.
tekai build path/to/main.tex --report-json

# Inspect and remove the configured output directory safely.
tekai clean --dry-run
tekai clean
```

See [Usage and configuration](docs/usage.md) for the full command, engine,
preview, cache, configuration, and lint reference.

## Build modes

| Mode | Purpose |
| --- | --- |
| `--engine tekai-engine --runner direct` | Default exact build through the self-contained Tekai engine and scheduler. |
| `--runner latexmk` | System `latexmk` baseline or compatibility escape hatch. |
| `--engine xe-latex` / `lua-latex` / `tectonic` | Installed external engine with the selected runner's supported orchestration. |
| `--engine tekai-pdftex` | Experimental approximate native renderer; unsupported inputs fall back to the exact pdfTeX path. |
| `--engine tekai-pdftex-certified` | Run native diagnostics, then deliver the exact pdfTeX artifact. |

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

Projects may check in `tekai.toml`. CLI arguments explicitly supplied by the
user override config values.

```toml
[build]
engine = "tekai-engine"
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

The checked-in [tekai.toml](tekai.toml) is a complete starting point.

## Fidelity contract

For the default embedded engine, visible PDF output is the correctness gate.
PDF bytes may differ because object ordering, timestamps, identifiers, or
compression differ; fixed-DPI page pixels and extracted document text must not.

The exact engine and the experimental renderer are intentionally separate:

- [`crates/tekai-engine`](crates/tekai-engine/README.md) is the self-contained
  exact engine used by default.
- [`crates/tekai-pdftex`](crates/tekai-pdftex/README.md) is the native
  replacement-engine research track.

Current evidence and known limits are recorded in:

- [Tekai engine parity report](output/pdf/tekai-engine-parity-report.md)
- [experimental native-renderer divergence audit](output/pdf/pdftex-native-divergence-audit.md)

## Repository guide

| Path | Responsibility |
| --- | --- |
| `src/compiler.rs` | Direct build scheduler, caches, dependencies, auxiliary tools, and engine dispatch. |
| `src/watch.rs` | Watch filtering, live preview, debounce, and structural fallbacks. |
| `src/lint.rs` | Single-pass TeX-aware linting and suppression handling. |
| `crates/tekai-engine` | Self-contained exact typesetting engine. |
| `crates/tekai-pdftex` | Experimental native renderer and next-engine architecture. |
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
skip when it is unavailable. CI also checks the standalone `tekai-engine` binary
feature set; the exact commands are documented in
[docs/development.md](docs/development.md).

## Support and security

Use [GitHub issues](https://github.com/NicoNekoru/tekai/issues) for reproducible
bugs and feature requests. Include `tekai --version`, the selected engine and
runner, the failing command, and a minimal TeX fixture when possible.

TeX documents can invoke external programs when shell escape is enabled. Only
use `--shell-escape` with documents you trust. Report security issues through
the process in [SECURITY.md](SECURITY.md), not a public issue.

## License

Tekai is available under the [MIT License](LICENSE).
