# Changelog

All notable user-facing changes are recorded here. Versions follow semantic
versioning.

## Unreleased

- Reduce debug-build disk usage by embedding TeX archives and the format as
  read-only object data instead of duplicating them in compiler metadata on
  macOS/Linux ARM64 and x86-64. Keep full debug information and all runtime files.
- Consolidate the 105 compiler integration tests into one executable so they
  share one embedded engine. Use `cargo test --test compiler compiler_cache`
  (or another module filter) for focused compiler suites.

## 0.4.0 - 2026-10-03

- Add `tekai format [PATH ...]` to apply safe lint fixes without compiling,
  with read-only `--check`, JSON reports, quiet output, and lint warning policy.
- Deduplicate overlapping lint and format targets by their resolved file paths.
- Include the first TeX error, source context, and log path in direct-build
  failures, including quiet and JSON builds.
- Preserve check diagnostics in JSON when lint passes but compilation fails.
- Accept continuation indentation in multiline braced arguments without
  requiring it in unindented macro bodies or stripping it during auto-fix.
- Expand the VS Code integration with a PDF.js viewer, SyncTeX navigation,
  project-aware editing features, per-paper config lookup, compiler diagnostics,
  and an explicit one-time warning override.
- Keep editor checks strict by default, including Neovim checks.
- Document shorter Homebrew install and upgrade commands.

## 0.3.0 - 2026-09-15

- Bundled pinned LaTeX packages, fonts, maps, and a matching format so default
  builds and checks need no TeX Live, MacTeX, `kpsewhich`, or runtime downloads.
- Added in-process BibTeX and native file lookup, including explicit recursive
  project search paths. Read PythonTeX cache metadata without running Python.
- Added ICLR's `eso-pic` dependency and verified Times small caps, bold, and
  italic fonts without installed TeX tools.
- Required `--external-tools` for installed auxiliary programs. Unsupported
  workflows fail explicitly instead of silently changing the output. External
  engines, runners, and shell escape remain separate opt-ins.
- Made root-document commands discover the nearest project configuration and
  stopped text-mode checks before compilation when linting fails.
- Fixed scheduled EPS output names when a document explicitly prioritizes EPS,
  so opt-in conversion does not require TeX to run another shell command.
- Added dependency-free package, font, bibliography, and large-paper regression
  tests before optional TeX installation in CI.

## 0.2.0 - 2026-07-20

- Added first-party VS Code and Neovim integrations for diagnostics, exact and
  fast builds, root-document discovery, and live PDF previews.
- Added `tekai init [PATH]` to create a complete, documented default config
  without replacing an existing file unless `--force` is passed.
- Made `check --report-json` include the exact diagnostics that gated its build,
  and made the Neovim check command replace annotations from that same report.
- Fixed `--synctex` on the default embedded engine so it emits a usable
  `.synctex.gz` sidecar instead of forwarding an unsupported option.

## 0.1.0 - 2026-07-17

- Introduced the `tekai` CLI for direct, converged LaTeX builds, checks, linting,
  cleaning, and dependency-aware watch mode.
- Shipped the self-contained exact Tekai typesetting engine.
- Added reusable build, bibliography, auxiliary, and preamble caches.
- Added low-latency preview watching with conservative structural fallbacks and
  optional idle-time final builds.
- Added orchestration for common bibliography, index, glossary, graphics, code,
  and externalization tools.
- Added JSON reports for editor, CI, and benchmark integrations.
- Added Ruff-style `check --fix` support for conservative math-delimiter and
  indentation repairs before linting and building.
- Added configurable space/tab indentation and hard-wrapped/unwrapped prose
  policies, while preserving neutral prose behavior when no policy is set.
- Excluded package `.sty` files from lint targets while retaining them as build
  and watch dependencies.
- Kept the experimental `tekai-pdftex` renderer as a separate, explicitly
  non-parity engine track.
- Standardized the project, binary, config, caches, environment variables,
  documentation, and diagnostics on the `tekai` name.
