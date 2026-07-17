# Development

## Workspace layout

The workspace contains three Rust packages:

| Package | Role |
| --- | --- |
| `tekai` | CLI, build scheduler, caches, watcher, linter, and integration tests. |
| `tekai-engine` | Self-contained exact engine used by the default direct path. |
| `tekai-pdftex` | Experimental native replacement renderer and engine-v2 research. |

Keep the fidelity boundary explicit. A successful `tekai-engine` build is
expected to preserve rendered pdfTeX output. A successful experimental native
build only proves that the supported subset executed; it does not imply general
pdfTeX parity.

## Local verification

Match the CI gates before handing off a change:

```sh
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy -p tekai-engine \
  --bin tekai-engine \
  --no-default-features \
  --features standalone-binary \
  --locked -- -D warnings
cargo build -p tekai-engine \
  --bin tekai-engine \
  --no-default-features \
  --features standalone-binary \
  --locked
cargo test --workspace --locked -- --test-threads=1
cargo build --release --locked
```

Focused commands are useful during iteration:

```sh
cargo test --lib watch::tests
cargo test --test compiler_cache
cargo test --test compiler_tekai_pdftex
cargo test -p tekai-engine
cargo test -p tekai-pdftex
```

Integration tests that depend on optional external programs skip when those
programs are unavailable. Do not interpret a skipped optional integration as
proof that the external workflow works on the current machine.

## PDF fidelity gate

For changes to the embedded engine, scheduler pass selection, font/image/PDF
code, or output-affecting caches, compare the real large examples:

- `examples/arXiv-2605.26379v1/main.tex` (48 pages in the reference build);
- `examples/arXiv-2511.08544v3/main.tex` (50 pages in the reference build).

Build a system-pdfTeX reference and the candidate with equivalent source,
environment, auxiliary state, and options. Render both at a fixed 144 DPI:

```sh
mkdir -p tmp/pdfs/reference tmp/pdfs/candidate
pdftoppm -r 144 -png reference.pdf tmp/pdfs/reference/page
pdftoppm -r 144 -png candidate.pdf tmp/pdfs/candidate/page
```

Compare every page's pixel data and page count. Also compare `pdfinfo`,
`pdffonts` when font work is involved, and `pdftotext` output. Raw PDF byte
equality is not required because timestamps, identifiers, object ordering, and
compression may legitimately differ.

Keep temporary rendered pages under `tmp/pdfs/` and remove them after the
inspection. The current checked evidence lives in
[`output/pdf/tekai-engine-parity-report.md`](../output/pdf/tekai-engine-parity-report.md).

## Performance measurement

Measure release binaries and realistic papers:

```sh
cargo build --release --locked
target/release/tekai build examples/arXiv-2605.26379v1/main.tex --report-json
target/release/tekai build examples/arXiv-2511.08544v3/main.tex --report-json
```

Use separate output directories for cold runs, report medians over repeated
matched trials, and preserve equivalent auxiliary/cache state when comparing
engines. A smoke test on `examples/minimal.tex` is not evidence about the large
paper bottleneck.

For `watch --preview`, benchmark warmed body edits on copies under `/tmp` or
another non-ignored tree. Do not place watched copies under `target/`; the watch
filter intentionally ignores it. Separate initial build/prewarm time from the
warmed edit latency and verify structural changes still use the conservative
whole-preview fallback.

Performance changes are accepted only with the relevant correctness gate. In
particular, final-build optimizations require rendered parity, and watch changes
must preserve dependency filtering and structural fallbacks.

## Code organization

- `src/compiler.rs` owns orchestration and exact-engine dispatch. Keep
  scheduler policy separate from engine implementation.
- `src/watch.rs` may optimize body edits, but structural or mixed changes must
  remain conservative.
- `src/lint.rs` is a scanner, not a LaTeX parser. Avoid claiming general TeX
  semantics from lint-only source analysis.
- `crates/tekai-engine/src/generated` is the checked-in Rust engine core. Keep
  hot-path changes narrow and validate them on real documents.
- `crates/tekai-pdftex/src/native.rs` is experimental. Unsupported behavior
  should be named or fall back, never silently approximated as exact.

The long-term native-engine design is in
[`crates/tekai-pdftex/ARCHITECTURE.md`](../crates/tekai-pdftex/ARCHITECTURE.md).

## Documentation maintenance

- Keep the root README as the short orientation and quick start.
- Put user-facing command/configuration details in `docs/usage.md`.
- Put contributor gates and measurement procedure in this document.
- Keep historical benchmark numbers dated and scoped to their exact commands.
- Do not describe `tekai-pdftex` as exact; the default `tekai-engine` path is
  the parity-preserving embedded engine.
- Update `--help`, config parsing, tests, and docs together when adding a flag.

## Release checklist

1. Update `CHANGELOG.md` and the package version.
2. Run the full Rust, help, and rendered-PDF gates above.
3. Confirm every commit subject is a single printable ASCII line.
4. Tag `v<version>` on `main` and create the matching GitHub release.
5. Update `Formula/tekai.rb` in `NicoNekoru/homebrew-tap` with the release URL
   and SHA-256, then run `brew audit --strict --online` and `brew test`.
6. Install through the public tap and verify `tekai --version`.
