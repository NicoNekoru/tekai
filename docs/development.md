# Development

## Workspace layout

The workspace contains three Rust packages:

| Package | Role |
| --- | --- |
| `texpilot` | CLI, build scheduler, caches, watcher, linter, and integration tests. |
| `pdftex-rust` | Faithful Rust-owned pdfTeX port used by the default direct path. |
| `texpilot-pdftex` | Experimental native replacement renderer and engine-v2 research. |

Keep the fidelity boundary explicit. A successful `pdftex-rust` build is
expected to preserve rendered pdfTeX output. A successful experimental native
build only proves that the supported subset executed; it does not imply general
pdfTeX parity.

## Local verification

Match the CI gates before handing off a change:

```sh
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy -p pdftex-rust \
  --bin pdftex-rust \
  --no-default-features \
  --features rust-binary \
  --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --locked
```

Focused commands are useful during iteration:

```sh
cargo test --lib watch::tests
cargo test --test compiler_cache
cargo test --test compiler_texpilot_pdftex
cargo test -p pdftex-rust
cargo test -p texpilot-pdftex
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
[`output/pdf/pdftex-rust-parity-report.md`](../output/pdf/pdftex-rust-parity-report.md).

## Performance measurement

Measure release binaries and realistic papers:

```sh
cargo build --release --locked
target/release/texpilot build examples/arXiv-2605.26379v1/main.tex --report-json
target/release/texpilot build examples/arXiv-2511.08544v3/main.tex --report-json
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
- `crates/pdftex-rust/src/generated` is the checked-in Rust engine core. Keep
  hot-path changes narrow and validate them on real documents.
- `crates/texpilot-pdftex/src/native.rs` is experimental. Unsupported behavior
  should be named or fall back, never silently approximated as exact.

The long-term native-engine design is in
[`crates/texpilot-pdftex/ARCHITECTURE.md`](../crates/texpilot-pdftex/ARCHITECTURE.md).

## Documentation maintenance

- Keep the root README as the short orientation and quick start.
- Put user-facing command/configuration details in `docs/usage.md`.
- Put contributor gates and measurement procedure in this document.
- Keep historical benchmark numbers dated and scoped to their exact commands.
- Do not describe `texpilot-pdftex` as exact; the default `pdf-latex` path is
  the parity-preserving embedded engine.
- Update `--help`, config parsing, tests, and docs together when adding a flag.
