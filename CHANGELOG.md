# Changelog

All notable user-facing changes are recorded here. Versions follow semantic
versioning.

## 0.6.0 - Unreleased

### Rust engine and embedding

- Correct the CLI and engine distribution to GPL-2.0-or-later. Restore exact
  upstream pdfTeX provenance and copyright notices, preserve existing MIT
  grants for independently authored material and keep third-party licenses.
- Complete the replacement of production C utilities and internal C function
  ABIs with Rust implementations and shared record types. This includes
  allocation, stdio, strings, formatting, numeric scans, option parsing, regex,
  calendar handling, compression and the BibTeX bridge. Native C/C++ library
  builds are removed. Necessary OS interfaces and format-file layouts remain.
- Own generated core state, arguments, buffers, caches and resource sessions per
  invocation. Repeated calls and independent threads have separate state, while
  nested calls on one thread are rejected. Unsafe Rust entries return typed
  normal/fatal completion and tear down before propagating unexpected panics.
  The former C entrypoint/static-archive integration is no longer supported.
- Require `panic=unwind` for recoverable engine exits and teardown, including
  matched CI release builds. Input and format files must still be trusted.
  Ownership is not a sandbox or a guarantee that every malformed format is safe.
- Check font-table offsets, names, glyph indices, backend numeric fields and
  regex capacities before access or storage. Reject nonfinite/overflowing
  conversions and unrepresentable paragraph widths with normal diagnostics,
  rather than wrapping layout totals or aborting on debug overflow.

### Builds, subprocesses and caches

- Own subprocess groups through cleanup and reaping, forward CLI cancellation
  to nested shell/PGF work and cap scheduled tools at eight children. Direct
  engine CLI workers lease the foreground terminal and preserve STOP/CONT job
  control. Scheduled engine and auxiliary jobs are noninteractive with null stdin.
- Report private library-command SIGTTIN/SIGTTOU stops as I/O failures and clean
  their owned groups without transferring the host terminal. The explicit
  `CallerManagedProcess` policy inherits host job control and assigns descendant
  cleanup to the host. Ordinary completed `pclose`/`system` background side
  effects remain caller-managed.
- Index recursive project/shared trees once per owned lookup context, reuse
  overlapping roots and replace exponential wildcard matching with dynamic
  programming. Preserve lookup priority, databases and newly generated files.
  Bound lookup indexes, decoded PNG pixels, preview snapshots and watch queues.
- Read large raw-file requests directly into their destination after draining
  buffered input and pushback. Retain decoder validation, pipe job control and
  configured read-ahead for small requests.
- Vendor the pinned Rust PNG decoder dependency with bounded prefix copying for
  short-distance matches. Fix Adler-32 accounting when queued matches drain
  across partial output buffers. Preserve the configured checksum policy.
- Resolve invocation-owned console and interrupt handles at their actual call
  sites instead of hoisting unused TLS lookups onto token-reader entry.
- Store dependency state as compact JSON. Validate complete source bytes,
  compilation generations and lookup identities before atomic publication.
  Old TOML states rebuild once. Comments or textual end markers no longer hide
  source changes, and unproven generated-input/PGF child provenance prevents
  whole-document cache reuse while up-to-date figures remain reusable by `make`.
- Serialize equal/nested output folders, coordinate cleanup with active builds
  and validate selected bibliography inputs before restoring cached outputs.
  Recover malformed owned cache metadata and privately materialize compressed
  formats without trusting adjacent raw companions or writing shared installs.
  Validate compressed format input before optional-cache fallback, including
  nonstandard suffixes and output-directory selection. Malformed higher-priority
  compressed formats fail instead of selecting a lower-priority format.

### Lint, formatting and verification

- Replace repeated lint/fix prefix scans with per-pass source indexes, forward
  cursors and one-pass edit assembly. Bound checked long-line, Unicode and
  malformed-input routes with work counters and independent differential tests.
  Edit ordering remains `O(k log k)`, not a claim of universal linter linearity.
- Reduce development-cache growth with disabled incremental compilation and
  packed line-level debug information. Keep line backtraces and optional full
  debugger information. Release builds retain fat LTO and one codegen unit.
- Require real repeated/parallel invocation, malformed-input, cancellation and
  standalone PTY tests, plus fixed-DPI paper/image parity and deterministic work
  bounds. Strict paired CI retains every sample, balanced crossover units, the
  10% regression boundary, duration/order guards and conditional exact median
  intervals. Raw absolute MAD is diagnostic. Failed or inconclusive gates still
  block CI, and historical verdicts are not rescored.
- Add opt-in invocation-owned engine timings for initialization, execution,
  cleanup and destruction, with nested format-loading, PNG-writing, decode
  and codec observations. Keep
  same-binary public/worker route controls separate from strict gate samples.
- Protect benchmark observer handoff against interrupted dispatch and allocation
  failures. Keep native-child ownership through cleanup and preserve the first
  exception when cleanup also fails. Successful timing boundaries are unchanged.

## 0.5.0 - 2026-10-07

- Share standard personal and site-wide TeX addition trees with installed TeX
  tools. Add Kpathsea-style path expansion, ordered default insertion,
  database-only lookup, and an isolated `TEKAI_TEXMF_MODE=bundled` setting.
- Add `tekai locate` with JSON lookup diagnostics and invalidate build caches
  when new shared packages shadow previous inputs.
- Require real BasicTeX lookup and compilation comparisons in CI on macOS
  Apple Silicon and Intel, while retaining the no-TeX large-paper gate.
- Fix native file size and modification-date primitives on Intel macOS by
  linking the metadata function matching its 64-bit inode structure.
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
