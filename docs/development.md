# Development

## Workspace layout

The ongoing [performance audit](performance-audit.md) records confirmed
worst-case costs, cache failures, and review coverage. Its diagnostic runner
is separate from correctness gates and does not turn a reproduced failure
into CI success.

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
cargo test --lib lint::tests
cargo test --test lint --test cli_lint --test cli_format --test cli_check --test cli_help
cargo test --test compiler compiler_cache
cargo test --test compiler compiler_tekai_pdftex
cargo test -p tekai-engine
cargo test -p tekai-pdftex
```

Integration tests that depend on optional external programs skip when those
programs are unavailable. Do not interpret a skipped optional integration as
proof that the external workflow works on the current machine.

### Property based tests

The lookup graph, watch inbox and linter use bounded `proptest` generators.
Lookup results are compared with an exhaustive flattened-route model, including
wildcard precedence, aliases, cycles, excluded trees and newly created outputs.
Watch events are compared with a separate queue model across drains, errors and
rescans, including exact count and byte limits. Linter properties check Unicode
coordinates and compare safe math fixes with independently constructed output.
Fixing that output again must make no changes.

Keep generators finite and models independent of the production algorithm.
Focused boundary regressions and scan/retention assertions remain useful. Tests
that merely repeat roadmap text or check a plan's own constant list do not
establish runtime behavior.

CI records `PROPTEST_RNG_SEED`, the case count and any shrunk regression files.
Replay a failing property with the recorded decimal seed and its test-name
filter. For example, replace the seed below with the one from the failed run.

```sh
PROPTEST_RNG_SEED=12345 PROPTEST_CASES=512 \
  cargo test --locked --lib property_scalar_coordinates_round_trip
PROPTEST_RNG_SEED=12345 PROPTEST_CASES=512 \
  cargo test --locked -p tekai-engine --lib generated_trees_match_flattened_wildcard_precedence
```

Commit shrunk `proptest-regressions` files for real failures alongside the fix.
Do not fix the random seed permanently in a property or discard a regression
just because a later random run passes.

The `cli_texmf` suite exercises native shared-tree lookup without installed TeX.
CI runs it on macOS 15 ARM64 and Intel, then installs BasicTeX and runs its real
reference gate with `TEKAI_REQUIRE_SYSTEM_TEX=1`. That gate fails rather than
skipping when `kpsewhich`, `pdflatex`, `mktexlsr`, or the CTAN `xspace` package
is missing. It compares selected package/font filenames, path ordering, brace/tilde
expansion, and database-only lookup, then compiles a real package with both
engines. Fixture trees and filename databases are temporary and separate from
the paper directory, so recursive project lookup cannot hide a broken test.

```sh
cargo test --locked --test cli_texmf -- --test-threads=1
TEKAI_REQUIRE_SYSTEM_TEX=1 cargo test --locked --test cli_texmf real_tex_installation -- --nocapture
```

The workflow also supports manual dispatch. Local runs without TeX verify the
native cases and explicitly skip the reference case. No TeX installation is
needed to develop the resolver or run the bundled large-paper gate.

Compiler integration suites live under `tests/compiler/` and are registered
as modules in `tests/compiler.rs`. They share one executable rather than
linking the embedded engine into each suite. Add new compiler suites there;
retain the shared test guard at the start of each test, since some tests
temporarily change process-wide environment variables. Module-name filters
replace the former individual `--test compiler_*` targets. CLI and lint test
targets, including the dependency-free `cli_self_contained` gate, are unchanged.

Editor-only changes have focused gates in their package directories:

```sh
(cd editors/vscode && npm install && npm test)
(cd editors/nvim && nvim --headless -u NONE -l tests/run.lua)
```

The VS Code test compiles the complete extension before exercising its JSON and
watch-output protocol. The Neovim test loads the plugin in a clean headless
instance and verifies its protocol plus user-command registration.

## PDF fidelity gate

For changes to the embedded engine, scheduler pass selection, font/image/PDF
code, or output-affecting caches, compare the real large examples:

- `examples/arXiv-2605.26379v1/main.tex` (48 pages in the reference build);
- `examples/arXiv-2511.08544v3/main.tex` (50 pages in the reference build).

The bundled-runtime gate needs no TeX installation:

```sh
cargo test --locked --test cli_self_contained -- --include-ignored --test-threads=1
cargo build --release --locked
python3 tools/verify_bundled_papers.py --engine target/release/tekai
```

The first command includes both large-paper builds with an empty `PATH`.
CI runs it before installing optional compatibility tools. The second command
uses Poppler and a small, checksum-pinned upstream pdfTeX reference extracted
temporarily. It compares all pages with the same bundled format, packages,
fonts, and auxiliary files. These are verification tools, not CLI dependencies.
See [`runtime/README.md`](../runtime/README.md) for rebuilding the data and
matching format without MacTeX.

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

### Lookup sessions and retention limits

Mutable filesystem lookup is implemented in `tekai-engine/src/lookup.rs`.
Each build, dependency scan and native pass starts a fresh thread-local session.
Recursive roots are indexed by basename once, and an ancestor index supplies
filtered views for overlapping search entries. Matching repeated `//` wildcards
uses a dynamic-programming table and preserves outer-first directory priority.
Database-only entries retain their database order and reparse when the database's
identity, length or timestamps change. Explicit symlink paths remain searchable.

The session observes a stable external filesystem during a pass. `open_output`
registers newly created TeX files in existing inventories. Completed shell
commands and pipes invalidate the inventory because opted-in external commands
can create arbitrary inputs. Symlink-directory aliases are retained, including
initially empty aliases, so a generated file becomes visible through each name.
A new build or one-shot `locate` sees external edits.
Call `lookup::reset` before a new scheduler-resolution session when using the
engine crate directly. Never cache negative filesystem lookups across builds.

Retained data has explicit admission and eviction rules.

| Data | Retention budget | Lifecycle or fallback |
| --- | --- | --- |
| Mutable lookup indexes | 32 MiB charged cost, at most 128 indexes | Evict least recently used roots. Oversized trees/databases stream without retaining their full index, with at most 128 oversize markers. |
| Decoded PNG cache | 16 MiB including charged overhead, at most 256 entries | Evict least recently used pixels. Active readers own their `Arc` independently. Oversized images decode without admission. |
| Preview source snapshots | 16 MiB charged cost, at most 256 files and 2 MiB per file | Remove inactive dependencies. Uncached sources use the existing conservative snippet selection. |
| Queued watch paths | 1 MiB charged cost or 4096 unique paths, with one wakeup | Coalesce repeats and discard output/access events before queueing. Overflow or watcher errors request a normal whole-root rebuild. |

These are retention budgets, not a total process-memory limit. A currently
decoded image, native font tables and the typesetting workspace can require
additional memory. Container and path overhead are charged conservatively.
The immutable bundled filename index is process-owned `OnceLock` data, not a
history of document lookups. Native typesetting runs in a child per pass, so its
global font/image tables cannot accumulate across watcher rebuilds. Investigate
heap-tool warnings by ownership and repeated live measurements rather than
treating every at-exit allocation as a growing leak.

### Paired performance gate

CI keeps deterministic scan counts and retention budgets separate from timing.
The paired timing gate builds an immutable baseline and candidate with the same
pinned compiler and release settings on one runner. Pull requests use their
exact base commit. Manual runs accept `baseline_sha` or resolve the default
branch once. A default-branch run comparing itself uses its exact first parent
instead. Both commits and the selection policy are recorded.

Both releases build at the same canonical workspace and Cargo target paths.
CI builds the baseline first, records its actual revision, settings and hash,
then renames its owned target tree out of the way. The candidate checkout is
restored and builds into a fresh target at the original path. Failed partial
baseline builds stay quarantined rather than becoming candidate build inputs.
A missing normal-completion record, signal exit or failed target isolation
blocks target reuse. Candidate restoration is still attempted after baseline
failure, and candidate correctness checks can run when isolation succeeds.
Source changes or untracked residue block a valid comparison. Reports
distinguish original build paths from the frozen executable's storage path.
Equal build paths reduce environment differences but do not require equal
binaries or establish the cause of a measured slowdown.

```sh
python3 -B tools/benchmark_runtime.py --candidate target/release/tekai \
  --baseline /path/to/previous/tekai \
  --metadata /path/to/comparison-metadata.json --pairs 40 --gate
python3 tools/verify_bundled_papers.py --engine target/release/tekai --images
```

The metadata JSON requires `runner_label`, `toolchain`, and a `revision` and
`build_command` for each of `baseline` and `candidate`. CI creates it from the
actual checkouts and compiler. Local comparisons must record their actual build
conditions rather than borrowing metadata from a different runner. Poppler's
`pdftotext`, `pdfinfo` and `pdfimages` are required to check fixture output,
including every decoded color and alpha pixel in the image case.
CI also records each built artifact's SHA-256. The benchmark and attribution
controls bind any supplied build hash to the measured executable's bytes.

The benchmark covers nested lookup with 1000 unused folders, eight transparent
images and warmed build-cache hits. Each side has equal input content and
separate binaries, homes, outputs and caches. Cold initialization precedes two
single-command warmups and a fixed calibration block. Two opposite-order pilot
pairs each run 32 commands per side. Calibration uses the fastest per-side
median of parent-observed pilot batch time divided by its iteration count.
Single-command warmups and pilots stay outside inferential samples.
Matched iteration counts target twice the one-second minimum with a ceiling
of 512 iterations. Reports retain pilot commands, normalized rates, required
and selected counts, and any clipped headroom. An unreachable minimum is
inconclusive before inferential sampling. Iterations remain fixed afterward,
and unexpectedly short measured batches remain inconclusive. There are no
adaptive retries or replacement samples. The whole-run deadline stays at
900 seconds.
Forty baseline/candidate pairs alternate execution order and retain all raw
command samples. Adjacent opposite-order pairs form 20 geometric-mean ratio
blocks. Exact median intervals use a familywise confidence level of at least
95 percent across the three declared cases.

The declared slowdown boundary is 10 percent. A statistically demonstrated
regression returns exit 1. Insufficient precision, excessive variability or
order bias returns exit 2, not success. CI rejects both. Exit 0 requires all
cases to pass. The intervals assume independent blocks and stable conditions,
so even a passing hosted-runner comparison cannot prove universal performance.
Binary/input hashes, output controls and owned-process deadlines are checked
separately. Reports are saved in `target/runtime-performance/report.json` and
its Markdown companion by default. They make no peak-memory claim.

Cache-hit command records also retain the binary's optional `elapsed_ms` as
untrusted diagnostic data. Missing or invalid values cannot change calibration
or gate outcomes. The parent measures the command's elapsed time independently.
Their difference includes parent capture setup and completion scheduling,
launch, CLI setup, serialization and exit overhead. It does not isolate loader
time.

Full CI adds advisory attribution controls only after a completed non-green
timing comparison. It saves the primary evidence first, then swaps the same
executable artifacts' roles and compares the candidate artifact with itself.
Current binary and verifier hashes must still match the primary report.
Each control keeps separate provenance, logs and reports. The primary gate
keeps its original result. These later measurements help investigate
artifact-associated or role-associated effects under changing runner conditions.
Incomplete comparisons and execution or integrity errors skip attribution.

The old optional watch benchmark was removed. Watch retention now has one
bounded diagnostic route in `performance_ci.py`; fixed queue and snapshot
budgets remain library gates. Full CI separately compares every paper page
and the transparent-image case with pinned upstream pdfTeX.

### Previous maintainer measurements

Before the paired CI gate, matched warm trials on 2026-10-09 on an Apple M4 Pro
compared the installed
0.5.0 release with the optimized refactor. Each timing is the median of three
runs after warming the same case. Paper copies, output directories and caches
were separate for each binary. The added folders contained unused data files.

| Case | 0.5.0 | Refactor |
| --- | ---: | ---: |
| Minimal native pass, 4000 unused leaf folders | 1.309 s | 0.100 s |
| 50-page paper, no added folders | 2.612 s | 1.870 s |
| 50-page paper, 1000 unused leaf folders | 69.085 s | 1.867 s |
| 48-page paper, 1000 unused leaf folders | 8.250 s | 0.672 s |
| Cached build, 4000 unrelated personal-tree folders | 75.8 ms | 13.5 ms |
| Cached build, 4000 unrelated site-tree folders | 73.8 ms | 9.7 ms |
| 32 unique transparent PNGs, peak RSS | 162.0 MiB | 49.3 MiB |

The 25-include preview rotation was one continuous run per binary. Release
0.5.0 grew from 29.3 to 55.3 MiB as inactive sources accumulated. The refactor
started at 28.9 MiB and reached a roughly 35.1 MiB plateau after three edits.
It remained there through edit 23, with lower RSS at the last sample.
The complete sample series is in the historical benchmark JSON. Simple unpadded startup
timings differ by a few milliseconds, so this table is evidence about scaling
and retention rather than a promise that every small build is faster.

A final candidate-only recheck after the generated-file alias fixes measured
1.724 s for the padded 50-page paper, 0.624 s for the padded 48-page paper and
49.0 MiB peak RSS for 32 PNGs. Preview RSS stayed between 29.4 and 30.9 MiB
through all 25 rotating includes. Those samples are recorded separately in
`target/runtime-performance/final-candidate.json`.

## Build disk usage

Measure a fresh build directory, not a long-lived `target/debug` containing
old crate versions, feature combinations, test executables, and incremental
compilation caches. Do not clear another developer's cache to measure a change:

```sh
size_target=$(mktemp -d "${TMPDIR:-/tmp}/tekai-size.XXXXXX")
cargo build --locked --target-dir "$size_target"
du -sk "$size_target/debug"
cargo test --workspace --locked --no-run --target-dir "$size_target"
du -sk "$size_target/debug"
```

On 2026-10-03, matched fresh ARM64 macOS builds with Rust 1.98.1, the locked
dependencies, and the default debug profile gave these results against 0.4.0:

| Artifact/workflow | Before | After |
| --- | ---: | ---: |
| Debug directory after `cargo build --locked` | 2.73 GiB | 1.55 GiB |
| Debug directory after build + workspace test compilation | 9.57 GiB | 3.64 GiB |
| Engine `.rlib` | 725,666,456 bytes | 168,645,152 bytes |
| Engine `lib.rmeta` inside that `.rlib` | 559,786,456 bytes | 2,767,936 bytes |

Directory figures are allocated disk space from `du`, counting hard-linked
files once, and exclude executing tests, release builds, and runtime caches.
Test compilation is about 62% smaller. No debug information was disabled and
no package, font, source, or license file was removed. The CLI executable stays
about the same size: its pinned embedded assets are still required offline.
These are dated measurements, not cross-platform size limits.

`tekai-engine/build.rs` embeds the three large immutable files directly into
read-only object data on macOS/Linux ARM64 and x86-64. Cargo tracks all three
inputs; the assembler records each embedded region's actual byte length rather
than using a separately measured length that could become stale during
compilation. Other targets retain a static `include_bytes!` fallback. Do not
reintroduce large `const` byte slices: they copy asset data
into Rust metadata and incremental caches. The engine test compares every
embedded byte with the checked-in inputs.

Old generated artifacts are not removed by this change. An explicitly chosen
cache cleanup can reclaim those separately, at the cost of recompilation.

## Code organization

- `src/compiler.rs` owns orchestration and exact-engine dispatch. Keep
  scheduler policy separate from engine implementation.
- `src/watch.rs` may optimize body edits, but structural or mixed changes must
  remain conservative.
- `src/lint.rs` is a scanner, not a LaTeX parser. Avoid claiming general TeX
  semantics from lint-only source analysis. Keep automatic fixes deterministic,
  idempotent, suppression-aware, and conservative around ambiguous TeX. Validate
  fixer changes on copies of the large examples, confirm non-TeX files are
  unchanged, and use rendered parity when whitespace edits are broad.
  `format`, `format --check`, and `check --fix` share the same fix computation.
  Test read-only previews, idempotence, file selection, config overrides,
  suppressions, JSON reports, and exit policy when changing that contract.
- `crates/tekai-engine/src/generated` is the checked-in Rust engine core. Keep
  hot-path changes narrow and validate them on real documents.
- `crates/tekai-pdftex/src/native.rs` is experimental. Unsupported behavior
  should be named or fall back, never silently approximated as exact.
- `editors/vscode` and `editors/nvim` are thin clients of the public CLI. Keep
  their diagnostic and build-report decoding aligned with `--report-json`.

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
