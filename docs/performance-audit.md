# Tekai performance audit

The second performance pass found remaining worst-case costs and cache
correctness bugs after the bounded runtime refactor in `6a1d384`. The most
urgent problems are stale PDFs after changes during compilation, stale cache
hits when lookup precedence changes, and engine processes surviving
cancellation. Cache retention limits alone do not prevent these failures.

## Measurement conditions

Measurements on 9 October 2026 used the release candidate on an Apple M4 Pro
running macOS. Fixtures and cache directories were temporary. No MacTeX
installation was required. Wall times are individual diagnostic observations,
not portable CI thresholds. Memory measurements are peak resident set sizes
reported by `/usr/bin/time -l`.

The bounded runtime refactor passed the local Rust, standalone-engine, editor,
and bundled-paper gates before its commit. The 99-page reference comparison
was pixel-identical. Those gates did not exercise the cases below. This audit
does not establish remote CI success or a leak-free runtime.

`tools/audit_runtime.py` recreates these fixtures with isolated runtime and artifact caches
and process-group cleanup. Its report records failures rather than treating
them as a passing correctness suite. The timing samples are diagnostic.

```sh
python3 tools/audit_runtime.py --quick
python3 tools/audit_runtime.py --case lookup --case edit-race --case cancel
python3 tools/audit_runtime.py --case lint --case pdf --case cache
python3 tools/audit_runtime.py --case aux-concurrency
```

The runner requires macOS for RSS measurements. The cache-output checks use
`pdftotext` and explicitly record a skip if it is unavailable. It creates no
user project files and does not change the installed tekai binary.
Bundle extraction happens before timing samples. The current cancellation
fixture uses a long finite loop as a fallback in addition to process-group
cleanup. No probe requires removing or modifying a shared cache. CPU model
metadata is best effort when the sandbox denies system-information queries.
RSS is `null` with `rss_available = false` if the system timer cannot collect
it. The cancellation and concurrency checks require process inspection
permission.

## Confirmed cache correctness failures

### Changes during compilation can publish a stale PDF as fresh

`direct_build` fingerprints recorded inputs after the engine exits.
`write_build_state` therefore records the version currently on disk rather
than the bytes that produced the PDF.

The fixture prints `OLD-CONTENT`, writes a marker immediately, then executes
a million TeX loop iterations. After the marker appears, the driver replaces
the source with `NEW-CONTENT`. The build succeeds with the old text. Its next
ordinary build reports `skipped = true` and `tex_runs = 0`, leaving the old PDF.
Both an ordinary settled build and a `--once` build reproduced this failure.

The fix needs a build generation or input-open snapshot and a validation step
before cache publication. Merely queuing another watch event is insufficient
because that event can trigger the incorrect cache hit.

Relevant functions in `src/compiler.rs` are `direct_build`, `write_build_state`,
`recorded_inputs_with_root_mode`, `fingerprint_effective_tex_path_reusing`,
and `input_fingerprint_is_fresh`.

### New higher priority files do not invalidate the build cache

A fixture resolves `choice.tex` through `TEXINPUTS=project//:`. Initially it
uses `z/choice.tex`. Adding `a/choice.tex` changes the resolver result, but the
existing recorded input remains unchanged. `locate` selects the new file while
the ordinary build skips and retains `OLD-CHOICE`. A forced build produces
`NEW-CHOICE`.

`environment_signature` includes expanded search paths but not the membership
or lookup decisions of explicit and project trees. A correct cache dependency
must cover search precedence, including previously absent higher-priority
candidates. Repeatedly walking every directory on each cache check would fix
some invalidation cases at the cost of restoring the original performance
problem. The resolver and build cache need to share lookup dependencies.

Relevant code is in `src/compiler.rs`, `crates/tekai-engine/src/search.rs`,
and `crates/tekai-engine/src/lookup.rs`.

### Database identity checks disagree across caches

An external shared tree contains old and new versions of `auditchoice.sty`.
Replacing its `ls-R` atomically with a same-length database and preserving
mtime changes the resolver result. The ordinary build still skips and retains
the old PDF. A forced build uses the new package.

`lookup.rs` checks device, inode, ctime, mtime, and size. The shared-tree
signature in `search.rs` checks only path, mtime, and size. Both caches should
use one identity definition. Where identity cannot establish content
stability, use a content digest or conservative invalidation.

## Confirmed process and preview failures

### Cancelling the parent leaves the engine running

The CLI waits through `Command::status` without owning cancellation of the
process tree. Editor cancellation kills the CLI process, not necessarily its
engine child.

After terminating the parent of an infinite-loop TeX build, its engine child
remained alive with parent PID 1 and consumed 88.8 percent CPU after half a
second. The diagnostic driver killed and reaped its owned process group.

Repeated cancellations can accumulate CPU and memory use and leave old jobs
writing outputs. A shared process runner should own the child lifetime,
forward cancellation, terminate descendants, and reap them. Apply it to
ordinary builds, watch builds, and auxiliary tools. The VS Code and Neovim
plugins also need to agree with this ownership contract.

Relevant files are `src/compiler.rs`, `editors/vscode/src/extension.ts`,
and `editors/nvim/lua/tekai/init.lua`.

### Preview scanning can abort on non ASCII input

`hot_preview_definition_inputs` slices a UTF-8 string at byte 8192 without checking a
character boundary. A body with an accented character spanning bytes 8191
through 8193 compiled successfully, then aborted the preview watcher with
exit code -6 during prewarming.

Use a byte parser or round the limit down to a character boundary. Add a
regression test with multibyte characters at every bounded scan boundary.
The confirmed site is `src/watch.rs` in `hot_preview_definition_inputs`.

## Confirmed scaling and memory problems

| Case | Smaller input | Larger input | Observed cost |
| --- | --- | --- | --- |
| Recursive symlink DAG lookup | 7 physical directories, 4 levels | 19 physical directories, 16 levels | 7 ms to more than 12 seconds |
| Formatting short math lines | 20,000 lines | 80,000 lines | 0.87 to 12.97 seconds |
| Linting one backslash line | 20,000 bytes | 80,000 bytes | 0.51 to 8.15 seconds |
| Importing one PDF dictionary | 512 entries | 8,192 entries | 0.063 to 2.83 seconds |
| Importing page 1 with shared resources | 1-page, 27 KB PDF | 256-page, 50 KB PDF | 33.2 to 130.5 MiB peak RSS |
| Saving a minimal build cache | No unrelated outputs | 128 MiB of unrelated bibliography outputs | 29.4 to 145.8 MiB peak RSS |
| Reading a PNG palette | 3-byte palette | Invalid 24 MiB palette | 29.5 to 77.5 MiB peak RSS, both builds accepted |
| Scheduling EPS conversion | 4 jobs | 32 jobs on 14 CPUs | 4 and 32 simultaneous converter processes |

### Auxiliary tools have no shared concurrency limit

The bibliography, index, EPS, SVG, Asymptote, PythonTeX, MetaPost, and gnuplot
job runners start an OS thread for each discovered job. Their outer task
groups also run concurrently. There is no shared ceiling on threads,
subprocesses, or the combined memory cost of the jobs. The PGF make runner
does cap its own concurrency, but that limit does not govern the other tools.

An isolated converter stand-in delayed each job by half a second, then wrote
a tiny valid PDF. Four jobs produced four simultaneous converters. Thirty-two
jobs produced thirty-two simultaneous converters on this 14-CPU machine.
The timing is artificial and says nothing about real conversion speed. The
process count verifies that the scheduler launches every job at once.

All image references in that fixture were inside `\iffalse`. TeX skipped
them, but source preflight still ran all converters. The rendered one-page
document needed none of those images.

Use one bounded executor across tool categories, with explicit cancellation
and per-job resource ownership. Avoid eagerly converting resources that the
document does not use where that can be established safely. A TeX source
scan cannot in general determine arbitrary macro and conditional behavior.

Relevant code is in `src/compiler.rs`, including
`run_bibtex_jobs_if_stale_for_jobs`, `run_makeindex_jobs_if_stale`,
`run_eps_conversion_jobs_if_needed`, and `run_svg_conversion_jobs_if_needed`.

### Directory aliases still multiply traversal work

`disk_entries` follows directory symlinks. Avoiding ancestor cycles does not
avoid a directed acyclic graph in which two aliases at each level point to
the same next physical directory. The number of logical paths doubles per
level. The cache byte limit limits retained entries, not traversal work.

The lookup of a missing file took 2.69 seconds with 15 physical directories
and exceeded the 12-second diagnostic timeout with 19 directories. There
were no directory cycles.

A physical directory index should separate scanned directory identity from
logical alias paths. Deduplicating canonical paths without retaining alias
semantics would break explicit lookup paths and precedence. This needs a
graph-aware resolver rather than an extra cache-size adjustment.

The confirmed function is `disk_entries` in
`crates/tekai-engine/src/lookup.rs`.

### The linter and formatter repeat prefix scans

`line_bounds` starts at the beginning of the source for each diagnostic.
`byte_offset_for_line_column` repeats that work. Formatting many math lines
therefore performs quadratic work. Reversed `replace_range` calls also move
already-edited tails when edits change lengths.

`lint_math` calls `column_for_byte` while visiting each character, counting the
prefix again. Escape checks scan backwards through consecutive backslashes
in several passes. An 80,000-byte backslash line took 8.15 seconds to lint.

Build one line-offset index, carry the current Unicode column and escape
parity through scans, and apply edits into a new buffer in one forward pass.
Keep suppression, verbatim, and math-safety behavior unchanged.

The confirmed code is in `src/lint.rs`.

### PDF wrappers clone values before checking their caches

`Object::dict` clones the entire dictionary before checking whether its cached
wrapper exists. Dictionary key and value access repeat this clone inside
`pdftoepdf::copy_dict`. A one-page PDF of 113 KB with an 8,192-entry dictionary
took 2.83 seconds to import.

`Object::stream` has the same ordering problem for stream content. Page
resource and group wrappers also clone before checking their caches. Check
cache availability first, then borrow or share immutable parsed objects.

`build_pages` eagerly clones inherited resources for every page. Importing
only page 1 still clones the shared dictionary for all 256 pages. Keep page
metadata lazy or share inherited dictionaries by object identity.

Relevant code is in `crates/tekai-engine/src/xpdf.rs` and
`crates/tekai-engine/src/pdftoepdf.rs`.

### PDF parent cycles can hang the engine

A 428-byte malformed PDF has a valid page enumeration but a self-referencing
page-tree parent. `lookup_inherited` follows `Parent` recursively with no
visited set or depth limit. The import exceeded a 15-second timeout. The
diagnostic driver then killed the owned process group.

Use an iterative traversal with object-identity cycle detection and a defined
depth limit. Report malformed inheritance rather than looping indefinitely.

### Settled caches load unrelated outputs into memory

`collect_settled_aux_cache_file_paths` scans the entire output tree and collects
all files with selected sidecar extensions. `capture_settled_aux_cache_files`
loads them into vectors, including files unrelated to the current job. The
PDF is also read in full, and sidecar snapshots can add another copy.

A minimal one-page build with 128 MiB of unrelated `.bbl` files reached
145.8 MiB peak RSS. Without those files it reached 29.4 MiB.

Use an artifact manifest for the current job and recorded dependencies.
Stream copies and hashes where possible. Bound any snapshots required for
pass comparison separately from the durable cache.

`direct_build` captures its reported elapsed time before fingerprinting and
saving these caches. Its report excludes that work, so phase timings should
include publication or label it separately.

Relevant functions are in `src/compiler.rs`.

### PNG metadata parsing accepts unbounded palette lengths

`parse_metadata_from_file` allocates `PLTE` and `tRNS` buffers using the declared
chunk length before validating format-specific bounds. A 1-by-1 RGB PNG with
an invalid 24 MiB palette was accepted and reached 77.5 MiB peak RSS. Its
valid three-byte-palette counterpart reached 29.5 MiB.

Validate chunk lengths and structure before allocating. The decode cache
budget does not cover metadata buffers or the active decoded frame. The
decoder library's allocation limit does not constrain a caller-allocated
output buffer, so frame dimensions also need checked arithmetic and an
explicit memory policy.

The confirmed metadata code is in `crates/tekai-engine/src/pngshim.rs`.

## Remaining review targets

The production linter, watcher, watch-event collector, resolver, PDF wrapper,
PDF import adapter, PNG adapter, low-level support routines, and editor integration code have received detailed
function-level review. Compiler orchestration, cache publication, fingerprints,
and selected parser paths have also been traced across files. Review of the
remaining compiler helpers and experimental engine is ongoing.

The generated engine is approximately 115,000 lines. It has not received a
complete manual function-by-function review in this pass. Generated and
experimental code must not be described as fully audited on the strength of
search results or the passing parity gate.

The following concerns need targeted measurements before they should become
confirmed performance findings.

- Preview input inlining has a depth limit and per-file limits, but no total
  byte or fanout limit. Long-line snippet fallback can exceed its nominal
  snippet budget.
- Search-path brace expansion bounds nesting depth but not result count or
  total bytes.
- Background preamble compilation can detach on early error returns before
  its join point.
- Auxiliary fingerprints and snapshots read whole files, sometimes through
  parallel workers.
- Experimental macro scopes clone all definition and register state when
  first modified. Nested scopes can multiply the copying cost.
- Editor indexing repeats source reads and has broad cache invalidation.
- Persistent artifact caches do not have a global disk-retention policy.

## Fix order and regression checks

First define shared contracts for build generations, lookup dependencies,
file identity, bounded auxiliary scheduling, and process cancellation. Fixing the stale-output and orphan
cases takes priority over improving benchmark numbers. Then remove repeated
linter scans and PDF clones, replace alias traversal with a physical directory
graph, and stream job-scoped artifact caching.

Regression checks should test outcomes and bounded operation counts rather
than relying only on timing thresholds. Required cases include edits after
input consumption, creation of a higher-priority file, same-size database
replacement with preserved mtime, cancellation of a live engine, a symlink
DAG, long Unicode and escape-heavy lines, a shared-resource multipage PDF,
a cyclic PDF parent, oversized PNG metadata, and unrelated output sidecars.

Any output-affecting fixes still need the real-paper fidelity comparison and
the ARM64 and Intel CI gates. The current audit records failures and proposed
remedies. It does not claim these newly found failures are fixed.
