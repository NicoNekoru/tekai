# Verification policy and scoped guarantees

Final publication evidence belongs to the [GitHub release notes](https://github.com/NicoNekoru/tekai/releases).
This page records dated preparation measurements and the required release
checks. It is not a rolling publication status.

The 12 October 2026 preparation snapshot includes core checkpoint `4329103`,
followed by private-terminal handling in
`1d3c7b9`, harness ownership in `411daa3` and the stream probe in `dead5ab`.
Protected-system-command cleanup fixes are `7c99359` and `8a7b166`.
The lint rewrite is `a86f394` and CI-profile fix is `0764e7b`. Local results span
checkpoints through versioned production code at `e19c6e7`. The final migrated
hosted gates had not run when this snapshot was recorded. These measurements
alone do not establish a completed release verification.
Supported commands and measurement policy live in
[development instructions](development.md). Historical reproductions, native
utility and one-shot ownership checkpoints, and older timing comparisons remain
in the [October investigation record](archive/performance-investigation-2026-10.md).
Their verdicts apply to their named revisions.

## Remote evidence

| Run | Tested candidate | Result and scope |
| --- | --- | --- |
| [38277929669](https://github.com/NicoNekoru/tekai/actions/runs/38277929669) | `238cb16`, quick | Both macOS core jobs, ARM Rust 1.89 MSRV check and Python 3.10/3.13 runner tests passed. All four ARM paired cases passed with 128 pairs and 32 complete units each. Runtime diagnostics failed on 31 malformed-image cases under CI's forced `panic=abort` build. Parity and profiling were skipped. Quick collected no Intel timing. |
| [38254058813](https://github.com/NicoNekoru/tekai/actions/runs/38254058813) | `01b3eba` against actual `main` at `0b6e92e`, full | All eight jobs passed. Each macOS architecture exercised BasicTeX lookup, 170 runtime cases with zero failures or skips, all 99 reference pages with matching pixels and extracted text, and four strict paired cases with 128 pairs and 32 complete units each. This predates the complete core-state migration. |

`0764e7b` changes CI and build-provenance checks to require matching
`panic=unwind` release profiles. That prospective fix does not repair the failed
quick report or establish a passing runtime gate for the migrated candidate.
A passing ARM comparison cannot substitute for Intel measurements, and skipped
parity or profiling is missing evidence. Optional tests that return early must
not be counted as exercised compatibility workflows.

## Local migrated-candidate evidence

These gates ran on local checkpoints. They do not establish final remote
release evidence.

| Gate | Observed local result |
| --- | --- |
| Workspace units | 964 passed, four intentional child helpers ignored |
| Engine units | 472 passed, three intentional child helpers ignored |
| Architecture checks | Six passed |
| In-process invocation checkpoint | Five passed |
| PDFtex tests | 212 passed |
| Rust 1.89 workspace compile check | Versioned 0.6.0, all targets and features passed |
| Strict workspace Clippy | Versioned 0.6.0, all targets and features passed |
| CLI help | Versioned 0.6.0, every command passed the long-help/example gate |
| CLI cancellation | Four passed in 19.41 seconds after the nested PGF fixture guard correction |
| CLI supervisor, including PTY handling | Ten passed, one child helper ignored |
| Private-pipe stdio, including eight PTY cases | 41 passed, two child helpers ignored |
| Actual standalone CLI routing | Two passed, no failures or skips |
| Bundled CLI gate with ignored cases included | All 19 passed, including both large papers |
| Python harness suite | 408 passed |
| Lint/format at `a86f394` | 46 unit/property and 38 CLI/integration tests passed |
| Full versioned release runtime gate at `e19c6e7` | All 170 passed, zero failures, skips or timeouts |
| Local pinned-reference PDF comparison | All 99 pages matched pixels and extracted text |

The engine now uses shared Rust record types and Rust internal calls. Rust
implementations replace production C string/memory utilities, floating scans,
option parsing and regex matching. Core fields, argv, buffers and resource
sessions are invocation-owned, with non-owning scoped thread routing. Repeated
entry, nested rejection, independent threads, fatal/help completion and unwind
teardown have local tests. Necessary OS interfaces remain separate. Builds with
C and C++ compilers disabled do not prove the absence of OS FFI or memory defects.

Miri found a real move-provenance defect in environment `CString` views. The
raw-owner correction passed Stacked and Tree Borrows probes with seeds 0 through
3. Six actual environment tests passed Stacked Borrows, excluding the native-child
and 32 MiB capacity cases. The core constructor passed Tree Borrows, and Xpdf
typed drop, retained argv and memory-stream close passed Stacked Borrows.
The actual StreamSession constructor, registry, handle moves, scope reborrow,
close and residual finish now pass Stacked and Tree Borrows seeds 0 through 3.
Its production initializer accepts a nonce, letting this probe omit only OS
clock discovery. No alias, leak or isolation checks were disabled. These scoped
checks do not cover every resource path or establish leak freedom.

CLI foreground-terminal and private-pipe terminal-stop tests pass. Default
private commands report terminal stops as ordinary I/O errors without changing
the host terminal. CallerManagedProcess explicitly retains host job control
and host responsibility for descendants. Ordinary engine exits require
unwind-compatible builds. Trusted input and formats are still required.

Review also found test-runner cleanup signalling numeric groups after command
reaping. A shared non-reaping exit-observation owner now retains the leader
until group cleanup and observer publication, resumes stopped owned jobs, and
allows a full TERM grace for nested CLI cleanup before forceful termination.
Native regressions cover lost authority, residual children, stopped supervisors
and escaped output descriptors.

The first versioned full runtime check stopped after 18 passing cases because
Darwin denied metadata for a completed setuid-root `/bin/ps` child. Its failed
report remains intact. The fixes use fresh non-reaping exit proof for the exact
reserved leader, while other members still require complete zombie metadata and
two matching bounded group lists. Native helper, supervisor and abandoned-pipe
regressions passed. The subsequent full gate passed all 170 checks with unchanged
source and executable hashes. Concurrent workspace tests make its elapsed time
correctness evidence only, not a performance comparison.

The lint/format rewrite replaces repeated coordinate and escape-prefix scans
with per-pass indexes and forward cursors, applies edits in one forward build,
and indexes malformed balanced arguments. It also removes deep math-stack
copies and temporary indentation strings. Bounded legacy differentials cover
Unicode/CRLF locations, escape parity, masking, name/payload parsing and edit
precedence. Deterministic work counters exercise 80,000-line, long-line,
backslash and malformed-argument fixtures. A local debug audit collected 15
samples with zero timeouts. Formatting
40,000 and 80,000 math lines took 0.547 and 1.122 seconds. The 80,000-backslash
and long-line fixtures took 0.0343 and 0.0202 seconds. These diagnostic timings
do not replace strict paired release measurements. Existing edit sorting is
O(k log k), and the checked routes do not establish general linter linearity.

## Final-candidate gates and strict timing policy

Before release, the final frozen candidate must repeat both macOS core jobs,
the Rust 1.89 compile guard, property tests and deterministic work bounds,
in-process ownership and targeted Miri checks, bundled cache and cancellation
fixtures, malformed-input regressions, all-page PDF pixels/text, and strict
paired release timing on both macOS architectures. Record the exact candidate,
actual `main` baseline, toolchain, matched build settings, executable/fixture
hashes and raw journals. Required runtime assertions have no known-failure
exemption. Local results and historical green runs cannot stand in for these
gates.

Decision policy `paired-relative-gate-v2` retains 128 pairs, 32 balanced
crossover units, the 10% practical relative slowdown boundary and
multiplicity-adjusted median-ratio intervals. Fixed minimum batch durations,
the 5% order-bias guard, complete matched samples, output oracles, executable
and fixture integrity, cache-state checks and owned-process deadlines remain
blocking. Insufficient interval precision remains inconclusive with exit 2,
and CI rejects it. Pooled per-artifact absolute-duration MAD is diagnostic,
not an independent veto.

Historical reports retain their original policy and verdicts. No samples are
discarded, rescored or retried until green. Neither diagnostic profiles nor
hosted-runner passes prove universal performance or the interval assumptions.
See the [measurement policy](development.md#paired-performance-gate) for
commands, inference assumptions and retained-artifact requirements.

## Scoped cache correctness guarantees

- Build generations validate ordinary inputs and lookup identities before
  publication. Complete source bytes are hashed without guessing execution from
  textual end markers. Metadata and change times detect edits but are not a
  snapshot filesystem.
- Lookup-directory and alias dependencies detect new higher-priority inputs.
  Warm checks inspect metadata without recursively enumerating unchanged trees.
  Cold dependency inventory can still revisit engine-traversed directories.
- Compressed formats materialize in a private content-addressed cache. Source
  dependencies and corrupt-entry recovery avoid trusting stale adjacent raw
  companions or writing shared installations.
- Arbitrary generated-and-consumed inputs without consumed-byte receipts decline
  whole-document cache publication. Classification follows collection of TeX,
  auxiliary-tool and format dependencies, while provable ordinary inputs still
  receive edit validation. Location under an output folder alone does not make
  a source trusted.
- Scheduler-managed PGF list-and-make recipes remain unproven inputs. The parent
  rebuilds while `make` can reuse up-to-date figures. Build-state schema 45
  invalidates earlier unsafe states. Whole-document caching of this workflow
  requires complete figure-child dependency provenance.
- Current-pass and whole-build generated observations each have a 4 MiB budget.
  They preserve earlier-pass outputs without enumerating historical output
  trees. Oversized coverage declines publication rather than becoming a hit.
- Global bibliography restoration validates the currently selected dependencies.
  Publication is serialized, source aliases remain dependencies and malformed
  owned metadata causes a rebuild. Atomic replacement does not promise
  power-loss durability.

Subprocess ownership forwards CLI cancellation, stops owned descendants and
reaps direct children. Auxiliary commands share an eight-child ceiling.
Supervisor SIGKILL and commands that deliberately escape the owned group are
outside cleanup guarantees. Library callers retain their own signal policy.
Explicitly completed `system`/`pclose` background commands remain caller-managed
side effects. CLI jobs use an external supervisor and inherited worker groups.
All local terminal and cancellation checks above passed. These results do not
substitute for the final hosted gates.

Equal and nested output folders use sorted, deduplicated descriptor locks.
Cleanup holds an exclusive lease while builds share it. Persistent lock
retention is bounded by 769 files. These guarantees require cooperating upgraded
processes. Hash collisions can serialize unrelated folders, continuous builds
can postpone cleanup, and custom caches inside another build's output folder
still need separate placement constraints.

## Remaining limits and performance work

Operation-count evidence is still needed for unrelated settled-cache sidecar
loading, shared PDF resource copies, expansion-state copies and repeated
auxiliary-phase recorder reads. Streaming avoids complete recorder copies and
duplicate retained paths, but does not cap the longest row or every unique
dependency set.

Full-byte hashing uses 64 KiB of scratch per active call. Output snapshots have
at most eight hash workers, bounding their hash scratch at 512 KiB. This does
not bound total process memory or concurrent independent builds. Checked font
paths do not make every font/format reader safe for hostile input, and the
compact name-layout planner is not a globally optimal packing solver.

The experimental renderer remains research code without general page parity.
Release support remains macOS. Linux Python runner tests do not establish Linux
engine support. The exact engine remains a Rust library with fat-LTO release
builds and unwind-compatible exit handling.
