# tekai

[![CI](https://github.com/NicoNekoru/tekai/actions/workflows/ci.yml/badge.svg)](https://github.com/NicoNekoru/tekai/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`tekai` is a self-contained typesetting engine and Rust CLI for fast,
fidelity-preserving LaTeX builds, live previews, dependency-aware caching,
TeX linting, and safe source formatting.

The default `tekai-engine` + `direct` path runs Tekai's exact engine inside the
`tekai` executable. It owns typesetting and explicit convergence, bibliography,
index, external-tool, and cache handling without shelling out to a system
`pdflatex` engine. The two bundled large papers render pixel-identically to the
reference system at the repository's dated 144 DPI parity gate.

## Install

Homebrew on macOS:

```sh
brew tap NicoNekoru/tap
brew trust --formula NicoNekoru/tap/tekai
brew install tekai
tekai --version
```

Tap setup and formula trust are only needed once. For later upgrades, run
`brew upgrade tekai`. Homebrew versions without `brew trust` can use
`brew install NicoNekoru/tap/tekai` for the first installation.

The formula builds `tekai` from source. The executable includes its engine,
LaTeX format, a pinned package/font bundle, and BibTeX. Default builds and
checks do not need TeX Live, MacTeX, `kpsewhich`, or a runtime download.

Tekai 0.4.0 supports macOS. Linux portability work is not part of this release.

To install from a checkout instead:

```sh
cargo install --path . --locked
```

## Quick start

No TeX installation is needed for documents supported by the bundled runtime.
See [the package manifest](runtime/packages.lock.json) for its contents.
Additional project packages can be supplied through `TEXINPUTS`. Workflows
that need Biber, MakeIndex, or converters are not built in; they fail unless
you explicitly opt into installed programs with `--external-tools`.

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
# Create a documented tekai.toml containing every default setting.
tekai init

# Lint, then build only if lint passes.
tekai check path/to/main.tex --allow-warnings

# Apply safe lint fixes, then lint and build.
tekai check path/to/main.tex --fix

# Apply safe lint fixes without compiling.
tekai format path/to/paper --config path/to/paper/tekai.toml

# Check formatting and lint policy without changing files.
tekai format path/to/paper --check --config path/to/paper/tekai.toml

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

## Editor integrations

First-party subprojects provide the same lint/build/preview loop inside VS Code
and Neovim:

- [`editors/vscode`](editors/vscode/README.md) publishes Tekai diagnostics to
  the Problems panel and can render a refreshing PDF preview tab;
- [`editors/nvim`](editors/nvim/README.md) publishes through `vim.diagnostic`
  and drives a configurable external PDF viewer.

Both integrations support exact builds, fast one-shot previews, live
`watch --preview`, configurable root files, and `% !TEX root = ...` comments.
See [Editor integrations](docs/editors.md) for their shared CLI contract.

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
- built-in BibTeX, bibliography freshness checks, and reusable outputs;
- MakeIndex, Xindy, glossaries, nomenclature, and split-index workflows;
- common source-driven external tools, including SVG/EPS conversion,
  Asymptote, MetaPost, Gnuplot, PythonTeX, minted, and PGF externalization;
- recorder- and source-derived dependency tracking;
- settled build-state and preamble-format caches;
- JSON reports for editors, benchmarks, and CI.

Installed auxiliary programs run only with `--external-tools`. Without that
opt-in, unsupported auxiliary workflows fail without launching a program.
`--shell-escape` and external engines/runners are separate explicit opt-ins.

## Configuration

Projects may check in `tekai.toml`. Root-document commands find the nearest
config at or above the document, even when invoked from a parent directory.
CLI arguments explicitly supplied by the user override config values.
`lint` and `format` use the current directory's `tekai.toml` by default.
Pass `--config path/to/paper/tekai.toml` when targeting a paper from elsewhere.

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
indent_style = "tabs" # defaults to "spaces"
max_line_length = 120
prose_wrap = "unwrapped" # or "hardwrap"; omit for neutral behavior

[lint.rules]
"math/inline-dollar" = "error"
"line/length" = "warn"
```

The linter scans `.tex`, `.ltx`, and `.cls` sources, while package `.sty` files
remain build/watch dependencies without becoming lint targets. `format` applies
conservative math-delimiter and indentation fixes without compiling.
`format --check` reports needed changes without writing files. `check --fix`
applies the same fixes to the root document's source graph before linting and
building. Prose wrapping is reported but never rewritten automatically.
Both commands respect disabled rules and suppression comments. Remaining lint
errors fail the command, as do warnings unless `--allow-warnings` is passed.
Needed formatting changes always fail `format --check`, even when warnings are
allowed. See [the formatting reference](docs/usage.md#formatting) for options,
file selection, and JSON reports.

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
| `src/lint.rs` | Single-pass TeX-aware linting, safe fixes, prose policy, and suppression handling. |
| `crates/tekai-engine` | Self-contained exact typesetting engine. |
| `crates/tekai-pdftex` | Experimental native renderer and next-engine architecture. |
| `editors/vscode` | VS Code diagnostics, builds, and embedded/system PDF preview. |
| `editors/nvim` | Neovim diagnostics, commands, and live external PDF preview. |
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
