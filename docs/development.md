# Development

## Workspace layout

The [verification policy](performance-audit.md) records revision-scoped
correctness, performance and ownership evidence. Diagnostic measurements stay
separate from blocking gates and do not turn a reproduced failure into CI
success. Historical investigations retain their original outcomes.

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
  --all-targets \
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
rustup run 1.89.0 cargo check --workspace --all-targets --locked
rustup run 1.89.0 cargo check -p tekai-engine --all-targets \
  --no-default-features --features standalone-binary --locked
python3 -B -m unittest discover -s tools -p 'test_*.py'
```

Use Python 3.13 for the macOS diagnostic and performance harnesses. Owned
process-group cleanup requires `os.waitid` with `WNOWAIT`, which some system
interpreters lack. The macOS jobs pin 3.13 and check those capabilities before
launching commands. Linux runner tests retain the 3.10/3.13 matrix.

Focused commands are useful during iteration:

```sh
cargo test --lib watch::tests
cargo test --lib lint::tests
cargo test --test lint --test cli_lint --test cli_format --test cli_check --test cli_help
cargo test --test compiler compiler_cache
cargo test --test compiler compiler_tekai_pdftex
cargo test -p tekai-engine
cargo test -p tekai-engine --no-default-features --features standalone-binary \
  --test standalone_cli --locked -- --test-threads=1
cargo test -p tekai-pdftex
```

Integration tests that depend on optional external programs skip when those
programs are unavailable. Do not interpret a skipped optional integration as
proof that the external workflow works on the current machine.
Run affected optional suites with `--show-output` to retain their skip notices.
Report missing tools explicitly. Hosted BasicTeX results and local bundled
results establish different coverage.

The `standalone_cli` target requires `standalone-binary` and executes the real
standalone executable through a private controlling PTY. Its two tests check
terminal input/output and SIGTERM cleanup of blocked pipe descendants. CI
integration discovery enables required features per package and requires both
tests to execute, rather than accepting a zero-test success. These are native
CLI tests, separate from unit tests of the shared supervisor helper.

### Invocation ownership

`tekai_engine::run_from_args` and `run_with_options` own the generated core,
argument storage, allocation arena, caches, decoders, streams and other mutable
resources for one call. Scoped thread-local pointers route access without
owning those resources. Repeated calls and independent threads have separate
state. A nested invocation on the same thread returns `NestedInvocation`.
Invocation-local environment overrides and working directories do not change
the host process environment or directory, and the library installs no signal
handlers.

Both entry functions are unsafe. Callers must supply trusted TeX input and
format files and keep internal engine calls within their invocation. Ownership
does not make unchecked translated pointers safe for arbitrary formats or
sandbox opted-in shell commands. Fatal diagnostics and help/version exits
return typed completions without exiting the host. Unexpected Rust panics
propagate after scoped teardown. Teardown releases partial resources; it does
not turn an unfinished PDF into a successfully finalized document.

Use `panic=unwind` for the executable hosting the engine. Abort-mode builds are
rejected because fatal diagnostic recovery uses Rust unwinding. CI builds both
performance artifacts with matching unwind settings, fat LTO and one codegen
unit. Passing `cargo test` alone does not certify the executable's panic
strategy, because Cargo tests use unwinding independently of release profiles.

Pipe children also have an ownership contract. The default `InvocationOwned`
policy creates private command groups and stops abandoned groups before reaping
their leaders. The host must not independently reap these children or configure
SIGCHLD for automatic reaping. `SupervisedProcess` inherits an isolated engine
worker group whose external supervisor must clean descendants before reaping
that worker. Ordinary explicit `pclose` and `system` completion preserves
intentional background shell side effects. Abandoned calls clean owned groups.
The library never transfers the host's terminal foreground group implicitly.

Private commands with a controlling terminal have an owned child-status
observer. An observed SIGTTIN or SIGTTOU stop terminates the owned group, wakes
blocked pipe I/O and reports a specific error. Bytes already transferred remain
counted. The failure is not silently converted to EOF. `pclose` and `system`
return failure as well. This also covers commands reopening `/dev/tty` after
standard descriptors were redirected. The observer joins before the child is
reaped, so it cannot signal a recycled PID or group afterward. Commands that do
not stop for terminal access retain ordinary completion semantics.

`PipeProcessPolicy::CallerManagedProcess` is an explicit embedding alternative.
It inherits the host process group and its existing terminal job control,
including whether that group is foreground or background. The invocation owns
and reaps only the direct shell child. The host accepts responsibility for
descendant cleanup and cancellation. This differs from `SupervisedProcess`'s
isolated external-worker contract. No policy silently starts a detached session,
substitutes null stdin for library commands or transfers the host terminal.

The CLI owns its signal registrations outside the engine. Direct engine CLI
invocation leases the foreground terminal to its worker only when the CLI
already owns that terminal. Worker suspension becomes a visible CLI stop.
continuation reacquires the worker terminal only after the shell foregrounds
the CLI. Completion and unwind restore foreground ownership before the worker
is reaped. Terminal changes mask SIGTTOU only in the calling thread and restore
its previous signal mask. SIGTERM receives bounded grace before group kill,
and ordinary nested shell/PGF workers stay in the outer owned group. A killed
supervisor, an independent child reaper or a command deliberately escaping its
group is outside this cleanup contract.

Automatically scheduled TeX and auxiliary-tool jobs run concurrently and are
noninteractive. They receive null stdin even when command output is visible.
Use direct engine CLI invocation for interactive terminal input, rather than
depending on a scheduled tool inheriting the user's terminal.

Use fresh task-owned target directories for minimum-toolchain checks rather
than mixing compiler versions in a development cache. Disable C and C++
compilers with `CC=/usr/bin/false CXX=/usr/bin/false` for candidate checks.
CI checks Rust 1.89 on ARM separately from the current-toolchain test matrix.
Before pushing a benchmark-policy change, run the full paired gate below with
actual locally built revisions, then run both hosted architectures. A local
ARM pass cannot certify hosted Intel behavior. Keep this preflight's reports
and binaries, and move its completed compilation directories to Trash when
reclaiming disk space. Leave unrelated developer targets and caches alone.

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
inspection. Current revision-scoped evidence lives in the
[verification summary](performance-audit.md) and final release notes.
The [archived parity report](archive/tekai-engine-parity-report.md) records an
earlier checkpoint.

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
Each build, dependency scan and native pass owns a fresh scoped session.
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
When resolving files through the engine crate directly, use
`lookup::with_lookup_session` or `with_lookup_policy` around the complete
operation so related lookups share one owned cache and release it afterward.
`lookup::reset` invalidates the active inventory; it is not a resource owner.
Never cache negative filesystem lookups across builds.

Whole-document cache publication requires evidence that every dependency was
stable during compilation. Files generated and consumed within a build do not
yet have consumed-byte receipts, so those documents rebuild even when their
outputs settle. This includes custom stage files and auxiliary-tool scripts
such as generated MetaPost input. Scheduler-managed PGF list-and-make jobs also
rebuild the parent until figure-child reads and generated recipe inputs have
complete provenance. Generated figures remain reusable through `make`. Recipe
hashes alone do not establish all dependencies of those children. Auxiliary
caches can still reuse unchanged figures and tool output. The compiler
classifies generated inputs after collecting TeX, auxiliary-tool and format
dependencies. A file's location under the output directory alone never makes it
a trusted generated artifact.

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
The bundled filename index and configured bundle/cache roots belong to their
invocation or public lookup context, rather than retaining the first call's
environment for the process lifetime. Native typesetting runs in a child per
scheduled pass; direct library calls also release their font/image resources
per invocation. Investigate
heap-tool warnings by ownership and repeated live measurements rather than
treating every at-exit allocation as a growing leak.

The pinned Rust decoder in `vendor/fdeflate` retains upstream attribution,
MIT/Apache-2.0 licenses and regression fixtures. Its local short-distance match
copying uses already-produced prefixes with bounded work and no unsafe code.
Both full-room dispatches keep their original admission checks. Independent
bytewise and upstream references cover every admitted short distance and length.
Streaming properties use separately encoded miniz data and retain failing seeds.
An upstream Adler-32 defect, reproduced in unchanged registry `fdeflate 0.3.7`,
required accounting for bytes published before partial queued-match returns.
The PNG checksum configuration remains unchanged. See the dependency's
[`SOURCE.md`](../vendor/fdeflate/SOURCE.md) for exact import and patch provenance.

Console and interrupt getters remain invocation-owned. Keeping these two
getters out of line prevents their TLS lookups from moving onto unused
token-reader entry paths. Actual polling, atomic consume-once behavior and
console selection remain unchanged. Sampling and candidate-only local image
controls guided these changes. They do not establish a hosted regression cause
or replace the unchanged strict gates against actual main on both architectures.

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
  --metadata /path/to/comparison-metadata.json --pairs 128 --budget 3600 --gate
python3 tools/verify_bundled_papers.py --engine target/release/tekai --images
```

The metadata JSON requires `runner_label`, `toolchain`, and a `revision`,
`build_command` and `expected_version` for each of `baseline` and `candidate`. CI creates it from the
actual checkouts and compiler. Local comparisons must record their actual build
conditions rather than borrowing metadata from a different runner. CI binds
each expected version to the unchanged package manifest from its recorded
checkout and verifies the exact `tekai --version` output. Poppler's
`pdftotext`, `pdfinfo` and `pdfimages` are required to check fixture output,
including every decoded color and alpha pixel in the image case.
CI also records each built artifact's SHA-256. The benchmark and attribution
controls bind any supplied build hash to the measured executable's bytes.

Report schema 6 declares four cases, CLI startup through `--version`, nested lookup
with 1000 unused folders, eight transparent images and warmed build-cache hits
with 1024 genuinely referenced `.tex` dependencies. The cache document includes
every dependency by its relative path across 32 directories. Unused padding
would test launch cost rather than state loading and freshness work.
Every generated project has a pinned `tekai.toml` covered by its input hash.
Builds select it explicitly, so ancestor configuration discovery cannot alter
the workload. Homes, temporary directories and XDG paths belong to the fixture.
The environment clears inherited TeX search options and selects bundled data.
Each artifact runs in both neutral slots,
with four independent binary copies, homes, outputs and caches. Equal-length
replica names reverse the artifact assignment between slots. Every cell has
equal input content. Cold initialization precedes two single-command warmups
per cell and a fixed calibration block. Each slot has two opposite-order pilot
pairs, each with 32 commands per artifact. Calibration uses the fastest of the
four cell medians of parent-observed pilot batch time divided by iteration count.
Single-command warmups and pilots stay outside inferential samples.
Matched iteration counts target twice the one-second minimum with a ceiling
of 512 iterations. Reports retain pilot commands, normalized rates, required
and selected counts, and any clipped headroom. An unreachable minimum is
inconclusive before inferential sampling. Iterations remain fixed afterward,
and unexpectedly short measured batches remain inconclusive. There are no
adaptive retries or replacement samples. CLI startup has its own predeclared
0.25-second minimum. All other cases retain the one-second minimum.
Local runs default to a 3600-second deadline. CI explicitly allows 5400 seconds,
with a 92-minute step limit, because matched batches of a much slower baseline
can consume the hour before the final case completes. Explicit local budgets
are also capped at 5400 seconds.
The prospective policy uses 128 baseline/candidate pairs and retains every
raw command sample. Four pairs form
one eight-batch comparison unit. For baseline A and candidate B, the slot
schedule is `A0 B0 B1 A1` followed by `B0 A0 A1 B1`. The first quad alternates
between units and cases. Each artifact occupies every quad position and each
slot-position combination. The unit ratio is the fourth root of the product
of its four candidate times divided by the product of its four baseline times.
For multiplicative timing effects, combining both quads before inference
cancels fixed slot effects and a repeatable slot-position profile. The default
produces 32 complete inference units. Exact median intervals use a familywise confidence
level of at least 95 percent across the four declared cases. At this sample
count the interval uses the ninth and twenty-fourth sorted unit ratios.
The previous ten-unit policy used the minimum and maximum. Historical reports
keep their original policy and outcome.

The sample count comes from a prospective exact binomial power calculation,
not trimming or rescoring previous CI samples. Let `q` be the probability that
an independent complete unit ratio is at or below the fixed 1.10 boundary.
The table shows the probability that the interval alone supports a pass,
assuming a stable distribution and ignoring the separate duration and order guards.
It does not estimate `q` on a hosted runner or guarantee CI success.

| Probability `q` | Previous 10 units | Current 32 units |
| --- | ---: | ---: |
| 0.80 | 10.74% | 82.54% |
| 0.90 | 34.87% | 99.67% |
| 0.95 | 59.87% | 99.998% |

Each command retains 12 monotonic boundaries, command start, capture setup,
capture files opened, `Popen` entry and return, capture files closed, launch end,
waiter dispatch and acceptance, completion, and cleanup start and end. These
separate observer work from process wait and cleanup. One owned worker handles
blocking waits across commands instead of creating a thread for every launch.
The `Popen` return timestamp includes mandatory child ownership registration.
Registration remains protected if a later diagnostic step is interrupted.
The scored duration remains command start through parent-observed completion.
It includes process launch and scheduling, and is not a pure loader or cache
algorithm measurement. Startup and dependency-heavy cache timings are separate
cases. Their difference is not an estimate of isolated cache work.
Diagnostic host observations at unit boundaries stay outside measured command durations.
These records cannot establish independence or remove shared-runner drift.
Raw command evidence is appended after complete units, outside command timers.
Small atomic checkpoints describe progress without rewriting the growing raw
sample array. The final `complete-report` restores every sample and binds it
to its `.raw.*.jsonl` sibling by hash and exact batch descriptors. Preserve
both files for diagnosis. A `compact-checkpoint` cannot stand in for completed
evidence. A 128-MiB report limit preserves completed evidence and reports overflow instead of dropping
samples. Journal or checkpoint errors fail the run.
On a failed batch, `partial_batches` retains its earlier completed commands
as diagnostic evidence. Incomplete batches never enter inference or the raw
completed-unit journal. Error checkpoints also preserve active evidence after
an interrupted or partial journal write. Deadline messages distinguish a
per-command timeout from exhaustion of the whole-run budget.

The cache oracle verifies that the initialized state records every referenced
dependency and the main input. At batch boundaries it records the state's
bounded content hash, size, file identity, timestamps, recorded input count and
input-path hash. Cache-hit batches must retain that same state. This catches
unexpected rewrites or missing dependency work without putting oracle reads
inside scored command durations.

Decision policy `paired-relative-gate-v2` keeps the 10 percent slowdown boundary.
A supported regression returns exit 1. Insufficient precision, short batches
or order bias returns exit 2, not success. CI rejects both. Exit 0 requires all
cases to pass. The 5 percent order-bias guard uses uncanceled ratios within each
slot. The exact interval's coverage assumes independent complete units and a
stable population median. Crossover cannot remove arbitrary copy or cache
effects, changing position profiles or serial dependence. A passing
hosted-runner comparison cannot prove universal performance.

Absolute-duration volatility is diagnostic under this policy. The previous
blocking 10 percent pooled per-artifact MAD rule mixed slots and time, even
when the balanced ratio canceled repeatable nuisance factors. The report still
retains all 128 batch durations per artifact and warns above 10 percent. It
also reports each slot's median and MAD, complete-unit ratio MAD, lag-one
correlation of chronological log ratios, and at most four chronological segment
medians. Undefined correlation is `null`, not zero. These observations cannot
certify independence, invalidate an interval by themselves, or establish leak
freedom. No new volatility threshold replaces the old veto.

The report format remains schema 6 because the added diagnostics are compatible
with its complete-report and raw-journal bindings. The explicit decision-policy
identifier records the changed verdict rules. Historical reports retain their
original policy and outcomes. Prospective generated-model tests cover canceled
slot/position factors, shared volatility, true slowdowns and unchanged blocking
guards. Correlated and drifting ratio models characterize the method's limits.
They do not prove that hosted observations satisfy its assumptions. Investigate
such concerns before relying on nominal coverage for a release claim. Do not
trim observations, extend sampling after seeing results or retry until green.
Binary/input hashes, output controls and owned-process deadlines are checked
separately. Reports are saved in `target/runtime-performance/report.json` and
its Markdown companion by default. Unit-boundary parent CPU, GC and resident
memory observations are diagnostic. Primary command samples do not measure
child peak memory.

Cache-hit command records also retain the binary's optional `elapsed_ms` as
untrusted diagnostic data. Missing or invalid values cannot change calibration
or gate outcomes. The parent measures the command's elapsed time independently.
Their difference includes parent capture setup and completion scheduling,
launch, CLI setup, the compiler prelude before its timer, destruction after its
final timestamp, serialization and exit overhead. It does not isolate loader
time.

### Separate diagnostic profiling

After a failed full paired CI comparison, a separate step profiles both original
executables if their builds and comparison metadata succeeded. It runs after
uploading the primary gate evidence. Passing full runs skip this step.
This is a fixed profiling pass with eight chronological repetitions per
artifact, slot and workload, a 30-second command limit and a 600-second total
budget. It uses separate fixtures, binary copies, homes and caches. Instrumented
timings cannot enter calibration, statistical samples or the primary decision.
The profile records completion or errors and per-metric availability, not
comparative pass or fail verdicts. Missing metrics are explicit rather than
reported as zero.

The report retains externally observed launch/finish timing and available
process CPU time, peak resident memory, page faults, context switches and I/O
counters. Per-command parent observations include CPU time, high-water RSS,
allocated Python blocks and GC inventory outside the command clock. High-water
RSS can rise because of intentionally retained evidence and does not establish
a leak. Host observations record available load, memory and swap information
before and after chronological units, outside profiled commands.
Each repeated cache measurement has an adjacent same-artifact `--version` control.
The two cache cells per artifact expose replica-specific effects across slots.
Resource measurements describe the system timer's
scope, not the whole CI job or proof of leak-free behavior.
Timer text shares the child's stderr channel and remains untrusted. Truncated
or duplicate resource and phase records cannot establish valid measurements.

Candidate processes can opt into a bounded `TEKAI_PROFILE ` JSON stderr record
with `TEKAI_DIAGNOSTIC_PROFILE=1`. Current protocol 4 retains protocol 2's
18 timed phases, two explicitly unmeasured native phases and 12 fixed route
counters, within a 16-KiB line
bound. The profiler accepts legacy protocol 1 and marks its finer phases and
counters unavailable. Phase durations are diagnostic and untrusted,
just like optional build timing. Older baseline binaries may not support the
record and must report phases as unavailable. The normal benchmark clears the
profiling opt-in from its environment. Unmeasured phases are not inferred from
the residual between external and internal timers.

Phase records aggregate a fixed inventory of completed calls and bounded work
counters for state input counts, freshness checks and TeX execution routes.
The finer routes separate state reading from cache parsing, metadata checks
from content/effective-source hashing, memo work, lookup-session resets,
mode-key generation and report serialization. Their scopes overlap, so their
durations must not be summed into a total. Dedicated exact-engine children skip
the parent CLI profiler. Normal
engine completion and fatal diagnostics return typed results to the child
runner. Parent cache/build commands can report
argument/configuration setup, state loading, freshness checks and TeX subprocess
waits. Engine format initialization remains explicitly unmeasured.
Phase protocol version 3 names these unavailable observations as child-process
internals rather than claiming that normal Rust engine entry never returns.
Protocol 4 identifies build-state parsing as JSON. The reader still validates
versions 1 through 3 using their original labels and reasons, including TOML
parsing in versions 2 and 3. These parent protocols do not supply measured
child-internal durations.

The `tekai` CLI's engine workers use a separate exact-one
`TEKAI_ENGINE_PROFILE=1` opt-in.
The library's `run_profiled_with_options` creates a fresh invocation-owned
collector and returns its immutable snapshot after complete owner destruction.
Default entry does not create the collector or start profiling clocks. The
worker emits one best-effort `TEKAI_ENGINE_PROFILE ` protocol-2 JSON line within
4096 bytes. The supervisor parent emits no engine record. Unexpected Rust
panics retain normal propagation and produce no completed record.

The five lifecycle segments are owner construction, invocation setup, engine
body, cleanup and owner destruction. Shared boundaries partition the completed
successful library lifecycle. Cleanup includes flushing, stream completion and
scope removal. Nested format loading, PNG writing, PNG decoding and actual PDF
codec calls overlap engine execution and must not be added to the lifecycle
total. Format loading includes allocation, reads, validation and byte swapping.
PNG writing includes image packing, PDF buffer writes and nested decode and
codec work. Its duration must not be added to those nested observations.
Fixed counters record
decode attempts, bounded-cache hits, decoded bytes, compression calls and actual
consumed/produced compression bytes. Failed calls remain visible. Unreached
phases have null durations. CLI preparation has a separate timer outside the
library elapsed time. The reader retains the original seven-phase inventory
for protocol 1 and requires all nine phases for protocol 2. The additional
phases do not replace samples or change the primary timing gate.

That post-failure profiling pass also records a bounded, candidate-only
image-route control. The public
engine command and private engine worker use the same executable copy and the
original image fixture in each of two replicas. Eight pairs per replica balance
both launch orders. These commands have no resource wrapper or profiling opt-in.
Every output must pass the original image oracles. Binary and input hashes,
native cache metadata inventories, actual completion and balance remain
recorded. Observer work and owned-group cleanup stay outside completion clocks.
The control can distinguish a same-binary route difference on that runner.
It cannot identify an engine phase as the cause or replace public-command
samples in the strict gate. Partial controls remain explicit errors.

```sh
python3 -B tools/profile_runtime.py --candidate target/release/tekai \
  --baseline /path/to/previous/tekai \
  --metadata /path/to/comparison-metadata.json \
  --primary-report /path/to/comparison.json \
  --output target/runtime-performance/profiling/profile.json \
  --repeats 8 --timeout 30 --budget 600 --image-route-controls
```

Profiling and startup controls are separate artifacts. Neither can
replace a failed or inconclusive primary result. They can perturb later runner
conditions, so later observations cannot retrospectively explain a scheduling
event without supporting time-aligned evidence.

CI no longer automatically repeats the whole timing suite twice after a
failure. The bounded profiler supplies chronological same-artifact controls.
The manual whole-suite attribution runner has been retired. Use the bounded
profiler and rotated executable-copy startup controls when a specific result
needs diagnosis. Keep their provenance and timing separate from the primary
gate. Never rescore, exclude samples or retry until a result becomes green.

The old optional watch benchmark was removed. Watch retention now has one
bounded diagnostic route in `performance_ci.py`; fixed queue and snapshot
budgets remain library gates. Full CI separately compares every paper page
and the transparent-image case with pinned upstream pdfTeX.

### Historical performance evidence

The [archived maintainer measurements](archive/performance-investigation-2026-10.md#previous-maintainer-measurements)
retain the dated pre-gate timings and preview-memory observations. They are
historical diagnostics, not evidence for a new release candidate. Use the
strict paired gate above and record the final candidate's own results.

## Build disk usage

Development builds disable incremental compilation and use packed line-table
debug information. Tests inherit these settings. Assertions, overflow checks
and test unwinding remain enabled. Filename and line-number backtraces remain
available, but local-variable, parameter and type inspection requires full
debug information. Release builds retain their separate fat-LTO profile.

Opt into full debugger information for the command that needs it. Incremental
compilation is also available when faster edit/rebuild cycles matter more than
disk usage. Changing either setting creates additional cached artifact variants.

```sh
CARGO_PROFILE_DEV_DEBUG=full CARGO_PROFILE_TEST_DEBUG=full cargo test --locked
CARGO_INCREMENTAL=1 cargo build --locked
```

On macOS, packed debug information uses `dsymutil` and produces `.dSYM` folders.
Keep those folders alongside the binaries when diagnosing backtraces. A failed
`dsymutil` invocation can emit a warning without failing compilation, so check
build warnings before relying on source-line symbolization. The profile options
are documented in the [Cargo profile reference](https://doc.rust-lang.org/cargo/reference/profiles.html).

Measure a fresh build directory, not a long-lived `target/debug` containing
old crate versions, feature combinations, test executables, and incremental
compilation caches. Do not clear another developer's cache to measure a change.

```sh
size_target=$(mktemp -d "${TMPDIR:-/tmp}/tekai-size.XXXXXX")
cargo build --locked --target-dir "$size_target"
du -sk "$size_target"
cargo test --workspace --locked --no-run --target-dir "$size_target"
du -sk "$size_target"
```

On 2026-10-11, fresh ARM64 macOS targets with Rust 1.99.0 and identical locked
build inputs gave the following Cargo-target totals. Every variant disabled incremental
compilation and completed both commands above, including all 19 workspace test
executables. The C and C++ compiler commands were disabled. Size measurements
preceded test execution and additional toolchain or lint checks.

| Debug information | After build | After build + workspace test compilation |
| --- | ---: | ---: |
| Full, unpacked | 1,216,652 KiB | 2,272,552 KiB |
| Full, packed | 1,011,720 KiB | 2,142,448 KiB |
| Line tables, packed, new default | 880,512 KiB | 1,787,912 KiB |
| Line tables, off, rejected | 830,004 KiB | 1,573,532 KiB |

The selected profile reduces the matched test-compilation total by 21.3%, about
473 MiB. This comparison isolates debug-information changes. It does not measure
the additional savings from disabling previously enabled incremental caches.
Packed line tables retained source-line backtraces in an existing caught-panic
test. The smaller `off` variant lost local stack-frame source lines on this Mac
and was rejected. Its panic header still showed a source location, which alone
does not demonstrate debug-information lookup.

The full/unpacked target retained 452 compiler object files totaling
625,901,800 apparent bytes. Packed line tables retained none and produced 21
`.dSYM` folders totaling 214,184 KiB. The totals above include these folders.
They are `du -sk` filesystem-accounted sizes, counting hard links once. APFS
clones can share physical extents, so these figures do not prove independent
physical bytes or peak filesystem consumption. Concurrent local tests also
prevent a causal build-time comparison. These are dated measurements, not
cross-platform size limits.

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

The [archived native-engine proposal](archive/native-engine-proposal-2026-10.md)
is research history. It is not the supported architecture.

## Documentation maintenance

- Keep the root README as the short orientation and quick start.
- Put user-facing command/configuration details in `docs/usage.md`.
- Put contributor gates and measurement procedure in this document.
- Keep historical benchmark numbers dated and scoped to their exact commands.
- Distinguish the experimental `tekai-pdftex` crate from the legacy
  `--engine tekai-pdftex` CLI mode, which selects the exact standalone engine.
  The default `tekai-engine` mode uses the embedded exact engine.
- Update `--help`, config parsing, tests, and docs together when adding a flag.

## Release checklist

1. Update `CHANGELOG.md` and the package version.
2. Run the full Rust, help, and rendered-PDF gates above.
   Check the advertised minimum Rust toolchain. Preserve local optional-test
   skip evidence and the final candidate's full paired comparison on both
   hosted architectures. Failed and inconclusive performance results block
   release.
3. Confirm every commit subject is a single printable ASCII line.
4. Tag `v<version>` on `main` and create the matching GitHub release.
5. Update `Formula/tekai.rb` in `NicoNekoru/homebrew-tap` with the release URL
   and SHA-256, then run `brew audit --strict --online` and `brew test`.
6. Install through the public tap and verify `tekai --version`.
