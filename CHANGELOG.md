# Changelog

All notable user-facing changes are recorded here. Versions follow semantic
versioning.

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
