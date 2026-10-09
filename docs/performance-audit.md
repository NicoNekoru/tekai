# Tekai performance audit

The second performance pass found remaining worst-case costs and cache
correctness bugs after the bounded runtime refactor in `6a1d384`. The most
urgent problems are stale PDFs after changes during compilation, stale cache
hits when lookup precedence changes, and engine processes surviving
cancellation. Cache retention limits alone do not prevent these failures.

The preview scan now rounds its byte limit down to a UTF-8 character boundary.
The experimental expander now rejects the three reproduced invalid inputs
without aborting.
PNG metadata reads now validate palette, transparency, and header bounds before
allocating their buffers.
Compiler fingerprint fast paths now check physical file identity as well as
mtime, so preserved timestamps no longer hide the reproduced source edits.
Recursive disk lookup now inventories canonical directories instead of
expanding every alias path. Ordinary Unix directory entries also avoid
redundant metadata reads and child-directory canonicalization.
PDF wrappers now check their caches before copying dictionaries or streams.
Inherited page attributes use iterative parent traversal and reject cycles
with a normal input error.
Duplicate watcher notifications no longer select an unchanged source's EOF
as a preview edit and replace a valid PDF with a placeholder.
Hot preview preparation now selects the embedded engine rather than depending
on a separate development executable. JPEG resolution metadata now uses a
checked slice parser with bounded APP1 storage.
Type 1 subroutine cleanup now releases unused entries beyond the emitted
prefix. Duplicate PDF object index entries release their rejected allocations,
and the TrueType reader no longer allocates an unused cmap entry array.
The other confirmed failures below remain open unless their section says otherwise.

## Measurement conditions

Measurements on 9 October 2026 used the release candidate on an Apple M4 Pro
running macOS. Fixtures and cache directories were temporary. No MacTeX
installation was required. Wall times are individual diagnostic observations,
not portable CI thresholds. Memory measurements are peak resident set sizes
reported by `/usr/bin/time -l`.

The bounded runtime refactor passed the local Rust, standalone-engine, editor,
and bundled-paper gates before its commit. The 99-page reference comparison
was pixel-identical. Those gates did not exercise the cases below. This audit
does not establish a leak-free runtime. Remote results are recorded separately
below and apply only to their tested commits.

`tools/audit_runtime.py` recreates these fixtures with isolated runtime and artifact caches
and process-group cleanup. Its report records failures rather than treating
them as a passing correctness suite. The timing samples are diagnostic.

```sh
python3 tools/audit_runtime.py --quick
python3 tools/audit_runtime.py --case lookup --case edit-race --case cancel
python3 tools/audit_runtime.py --case lint --case pdf --case cache
python3 tools/audit_runtime.py --case aux-concurrency
python3 tools/audit_runtime.py --case input-identity --case source-boundaries --case format-cache
python3 -B -m unittest discover -s tools -p test_audit_runtime.py
cargo build --locked --release -p tekai-pdftex --example audit_expansion
python3 tools/audit_runtime.py --case expansion
```

The runner requires macOS for RSS measurements. The cache-output checks use
`pdftotext` and explicitly record a skip if it is unavailable. It creates no
user project files and does not change the installed tekai binary.
An unmeasured minimal build extracts the bundle and materializes the default
format before timing samples. The current cancellation
fixture uses a long finite loop as a fallback in addition to process-group
cleanup. No probe requires removing or modifying a shared cache. CPU model
metadata is best effort when the sandbox denies system-information queries.
RSS is `null` with `rss_available = false` if the system timer cannot collect
it. The cancellation and concurrency checks explicitly record a skip when
process inspection is unavailable. The runner tests exercise permission
denials and owned-process cleanup without launching a compiler.
The runner checks the system timer with a harmless command before using it.
When the timer is denied, it measures wall time directly and preserves the
compiler's exit status rather than reporting the timer's permission failure
as a build failure.

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

### Source fingerprints now check physical file identity

Effective TeX fingerprints accepted a matching mtime without checking the
current size, inode, or ctime. Both an in-place edit and an atomic replacement
of the root source reproduced a stale cache hit when the driver preserved
mtime. The changed source was longer. The replacement also had a new inode.
The ordinary build skipped with `OLD-CONTENT`, while a forced build produced
`NEW-CONTENT-LONGER`.

Compiler fingerprints now store physical size separately from effective
source length. Metadata-only reuse on Unix requires matching size, mtime,
device, inode, and ctime. Generic files, root and included TeX sources,
preambles, and cited bibliography subsets use the same comparison.
Changed identity triggers content hashing. Identical replacements and edits
to ignored comments can therefore still hit the content cache.

Entries without an identity rehash instead of trusting mtime. Platforms
without the Unix change-time identity also rehash. The build-state version
is now 41, including the directory-graph, JPEG and PNG changes. Regression tests cover
same-length and longer edits, atomic replacement, legacy entries,
unchanged-content reuse, and the metadata-only
fast path. A bundled CLI test checks actual rebuilds after preserved-mtime
edits and cache hits after an identical replacement.

This does not fix changes during compilation or textual execution-boundary
inference. Relevant functions in `src/compiler.rs` are
`input_fingerprint_is_fresh`, `fingerprint_effective_tex_path_reusing`, and
`file_metadata_fingerprint`.

### Textual end markers can exclude active source from fingerprints

`effective_tex_bytes` treats the first textual `\end{document}` or
`\endinput` as an execution boundary. It does not account for conditional
execution or macro definitions. Three fixtures reproduced stale cache hits
after ordinary content edits with changed mtime.

- A root source placed `\end{document}` inside an inactive `\iffalse` block.
- A root source defined a finishing macro containing `\end{document}`.
- An included source placed `\endinput` inside an inactive conditional.

Each next build skipped and retained `OLD-CONTENT`. Forced builds produced
`NEW-CONTENT`. A conservative full-content fingerprint is safer than trying
to infer executed TeX from these textual markers. An engine-observed input
snapshot can support more precise reuse without guessing macro semantics.

### Replacing a format leaves its raw companion stale

`check_format_path` prefers any readable `.fmt.raw` companion without checking
which `.fmt` produced it. Two tiny formats defined different output text.
After the first run materialized `active.fmt.raw`, replacing `active.fmt` with
the second format still produced `OLD-FORMAT`. Removing only the temporary
fixture's raw companion caused the next run to produce `NEW-FORMAT`.

Key raw companions by source identity or content, and publish them atomically
in a writable cache. Do not modify shared installation trees merely to read a
format. The confirmed code is `crates/tekai-engine/src/kpathsea.rs` in
`check_format_path` and `materialize_raw_format_companion`.

### Resolver and build cache now share database identity checks

An external shared tree contains old and new versions of `auditchoice.sty`.
Replacing its `ls-R` atomically with a same-length database and preserving
mtime changes the resolver result. The ordinary build still skips and retains
the old PDF. A forced build uses the new package.

The shared-tree signature previously checked only path, mtime, and size,
while the resolver checked device, inode, and ctime as well. The compiler,
resolver, and shared-tree signature now use `file_identity::change_identity`
for those identity fields. Database-only signatures still inspect the database
rather than walking the installation tree.

A signature regression checks in-place edits and atomic replacement with
preserved size and mtime. A bundled CLI regression switches the database
between two packages in a separate fixture-owned tree. It verifies a rebuild,
the newly selected package path in the log, and a subsequent cache hit.
The release diagnostic now rebuilds and produces the new package's text after
the same database replacement instead of retaining the old PDF.
The higher-priority project-file case remains open.

## Confirmed process and preview failures

### Cancelling the parent leaves the engine running

The CLI waits through `Command::status` without owning cancellation of the
process tree. Editor cancellation kills the CLI process, not necessarily its
engine child.

After terminating the parent of an infinite-loop TeX build, its engine child
remained alive with parent PID 1 and consumed 88.8 percent CPU after half a
second. The diagnostic driver terminated its owned process group and reaped
the CLI. The finite-loop fixture also reproduced the orphan with 87.2 percent
CPU after half a second.

Repeated cancellations can accumulate CPU and memory use and leave old jobs
writing outputs. A shared process runner should own the child lifetime,
forward cancellation, terminate descendants, and reap them. Apply it to
ordinary builds, watch builds, and auxiliary tools. The VS Code and Neovim
plugins also need to agree with this ownership contract.

Relevant files are `src/compiler.rs`, `editors/vscode/src/extension.ts`,
and `editors/nvim/lua/tekai/init.lua`.

### Preview scanning aborted on non ASCII input

`hot_preview_definition_inputs` sliced a UTF-8 string at byte 8192 without checking a
character boundary. A body with an accented character spanning bytes 8191
through 8193 compiled successfully, then aborted the preview watcher with
exit code -6 during prewarming.

`hot_preview_definition_inputs` in `src/watch.rs` now uses the existing
`floor_char_boundary` helper. The scan remains at most 8,192 bytes.
A regression test covers two-, three-, and four-byte characters at every
position crossing the limit, plus exact boundaries. It checks that early
definition inputs remain available and later inputs stay outside the scan.

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

The PNG and symlink rows record the original failures. Their fixes are described below.

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

### Directory alias lookup now uses a canonical directory graph

`disk_entries` follows directory symlinks. Avoiding ancestor cycles does not
avoid a directed acyclic graph in which two aliases at each level point to
the same next physical directory. The number of logical paths doubles per
level. The cache byte limit limits retained entries, not traversal work.

The lookup of a missing file took 2.69 seconds with 15 physical directories
and exceeded the 12-second diagnostic timeout with 19 directories. There
were no directory cycles.

`directory_graph.rs` now inventories each reachable canonical directory once
and retains sorted, named edges for every alias. Recursive wildcard matching
uses explicit frames and memoized misses instead of building all logical
paths. Missing basenames return immediately after inventory. Qualified names,
literal alias components, and nested wildcard priority retain their search
semantics. Matching constructs a candidate spelling only at a terminal file.

The 24-layer binary-alias regression reads 25 directories and evaluates 50
states for a present basename with a missing suffix component. A 48-layer
fixture with no usable terminal spelling evaluates 66 states on macOS.
Adding a short usable alias evaluates 68 states and returns that alias.
These are operation-count assertions, not timing thresholds. The release
depth-12 missing-file diagnostic takes about 9 ms on this machine, compared
with the earlier 5.51-second sample. RSS is unavailable under the sandbox.

The matcher tracks physical ancestors inside cyclic components and includes
consumed symlink hops in its memo keys on macOS and Linux. Raw symlink targets
and leaf file symlinks contribute their hidden hops. A filesystem regression
checks the actual supported-platform hop boundary. Final lexical validation
still checks that a returned spelling can be opened.

Overlapping query roots reuse graph nodes with a fresh ancestor context.
Outputs in known directories update physical file membership once for every
alias. Outputs in newly created directories evict affected inventories for
rebuilding. Database-only paths still use the filename database rather than
scanning the tree. Ordinary Unix entries use directory-entry types and join
actual child names onto canonical parents. The 80-file regression records
zero explicit metadata followups and zero child-directory canonicalizations.

Inventory retention remains byte- and entry-bounded. Matcher memo retention
has a separate 32 MiB ceiling. These are allocation estimates, not a bound on
total process RSS. Adversarial cyclic matching, path-length failures, and
matching after memo admission stops can still take excessive time. Oversized
inventories retain the old streaming fallback, which can still expand aliases.
The redesign fixes the reproduced acyclic alias explosion, not every possible
filesystem graph. The higher-priority input cache bug also remains open.

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

### PDF wrappers now check their caches before cloning

Previously, `Object::dict` cloned the entire dictionary before checking whether
its cached wrapper existed. Dictionary key and value access repeated the clone
inside `pdftoepdf::copy_dict`. A one-page PDF of 113 KB with an 8,192-entry dictionary
took 2.83 seconds to import.

`Object::dict`, `Object::stream`, and the page resource and group getters now
copy their source only on a cache miss. Their owned boxes and raw-handle
lifetimes stay unchanged. Deterministic tests count the actual clone sites.
Three scans over a 1,024-entry dictionary make one cached dictionary copy.
Repeated getters for a 1 MiB stream make one payload copy and preserve the
read cursor. Further tests check stable keys, nested references, independent
output ownership, reset invalidation, missing values, and parallel counters.

The latest native full-profile samples for the 8,192-entry fixture decreased
from 3.09 seconds before this change to 68 ms afterward. These are individual
observations on the same machine, not timing thresholds or a claim that every
PDF operation is linear.

`build_pages` eagerly clones inherited resources for every page. Importing
only page 1 still clones the shared dictionary for all 256 pages. Keep page
metadata lazy or share inherited dictionaries by object identity.

Relevant code is in `crates/tekai-engine/src/xpdf.rs` and
`crates/tekai-engine/src/pdftoepdf.rs`.

### PDF parent cycles now return a controlled input error

A 428-byte malformed PDF has a valid page enumeration but a self-referencing
page-tree parent. Previously, `lookup_inherited` followed `Parent` recursively
with no visited set or depth limit. The import exceeded a 15-second timeout. The
diagnostic driver then killed the owned process group.

Traversal now borrows dictionaries iteratively and tracks complete object
identities, including generations. A repeated parent returns an error through
page construction and marks the document invalid. The import adapter frees the
unregistered document and reports `xpdf: cyclic PDF page Parent chain` with
a normal engine exit status. The diagnostic has static storage and remains
valid after deletion.

Local values still shadow ancestors, including null and wrong-typed values.
The nearest ancestor wins, broken or non-reference parents end lookup, and
page groups remain direct-only. The 4,096-parent regression has no new fixed
parent-depth cutoff. Existing bounded reference dereferencing is unchanged.
Cycle detection applies when an inherited lookup follows the chain, not to
every parent link in an otherwise unused graph.

### Duplicate watcher events no longer replace a valid preview

The full rotating-input fixture caught a duplicate notification after a
successful full preview build. Its snapshot already matched the source.
Selecting EOF as the edit point removed the input-only body from the snippet
and replaced the correct PDF with `Live preview source changed.`

Target selection now rejects a source identical to its remembered snapshot.
The existing ordinary build/cache path handles that event, preserving failure
handling rather than assuming the current PDF is valid. Four unit regressions
cover synchronized duplicates, genuine body edits, unknown snapshots and
prewarming, and a duplicate after preamble fallback. The native fixture still
requires every rotated input's text in the PDF. It does not accept the
placeholder or substitute a direct body marker for the included source.

This change can still trigger ordinary full/cache work. Multi-source preview
selection and output-generation tracking remain separate design concerns.

### Hot preview now works with the single executable install

The fresh ARM CI environment caught a dependency on an unshipped standalone
engine during preview prewarming. A source-tree executable could hide the
failure locally even with an empty executable search path.

Hot preview preparation now selects the exact embedded engine. Ordinary builds
and fallback behavior keep their original engine options. A unit test checks
the prepared engine and its fast-build settings. Native validation of an
isolated executable copy passes Unicode prewarming and all ten rotating input
edits without a sibling engine or a runner override.

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

### PNG metadata allocations now enforce format bounds

`parse_metadata_from_file` allocated `PLTE` and `tRNS` buffers using the declared
chunk length before validating format-specific bounds. A 1-by-1 RGB PNG with
an invalid 24 MiB palette was accepted and reached 77.5 MiB peak RSS. Its
valid three-byte-palette counterpart reached 29.5 MiB.

The parser now rejects invalid palette lengths before allocation. Palette
storage is at most 768 bytes. Transparency payloads must match their color
type, and indexed transparency cannot exceed the palette entry count, at most
256 bytes. Header validation checks dimensions, bit depth, color type,
compression, filter, and interlace values. Duplicate palette and transparency
chunks are rejected. These are [PNG format limits](https://www.w3.org/TR/png-3/#11PLTE),
not a new size policy for valid images.

Metadata errors now terminate the engine with a normal error instead of leaving
default image dimensions. Unit tests cover valid metadata and malformed
declarations without large payloads. A bundled CLI regression builds a valid
one-pixel image, then checks that invalid palette and transparency declarations
exit with code 1 and the expected metadata error. It uses empty executable
search paths and fixture-owned caches.
The release diagnostic also accepts the valid palette and rejects the 24 MiB
declaration. Its RSS measurement is unavailable under the sandbox, so it does
not establish a post-fix memory delta.

The active decoded frame still needs a separate memory policy. The decoder
library's allocation limit does not constrain a caller-allocated output buffer.
The compressed input also remains a whole-file read on the decode path.

The confirmed metadata code is in `crates/tekai-engine/src/pngshim.rs`.

### PNG chunk traversal now validates extents and advances forward

The fast copy path cast unsigned chunk lengths to signed integers and used
relative seeks. An IDAT declaration of `0xfffffff4` became minus twelve,
returning traversal to the same header. The metadata pass accepted that IDAT
header without inspecting its extent, so a tiny file could reach the loop.
The copy pass also ignored incomplete payload reads.

Metadata and copy traversal now share checked length and extent helpers.
Chunks must obey the [PNG chunk-length limit](https://www.w3.org/TR/png-3/#5Chunk-layout)
and fit within the open file's measured extent, including their CRC bytes.
Absolute next offsets advance forward. IDAT totals use checked 64-bit sums,
and every copied read must be complete. The second pass checks its total
against the first pass before closing the stream. Existing palette and
transparency bounds retain their specific diagnostics.

Fourteen native framing gates check first and later chunk failures with tiny
fixtures. A valid two-IDAT control requires the actual copy path and exact
concatenated compressed bytes in the PDF. Helpers also have deterministic
overflow and progress tests without large payload allocations.

This checks CRC extent, not CRC contents on the fast copy path. Matching
two-pass lengths does not establish input-content stability during edits.

### PNG decoding failures now stop before output copying

Four adapter entry points discarded `ensure_decoded` errors. Generated row
writers could then copy an unfilled allocated buffer into the PDF. Source
review traced this through transformed updates, row reads, whole-image reads
and the RGBA fast path. No unfixed native probe was run.

All four entry points now report `invalid PNG image data` through the existing
fatal error path. Nine native cases require the exact decoder error, a normal
engine exit and no surviving incomplete PDF. Valid grayscale, RGB and RGBA
controls force decoding and require exact color and alpha bytes in the output.
Safe Result-level tests also verify that failed decoding caches no pixels and
repeated attempts return the same error.

The decoded-frame allocation policy and whole-file compressed reads remain
open. The change prevents swallowed failures, not every image memory problem.

## Confirmed experimental expansion failures

These measurements exercise `tekai-pdftex`, the opt-in experimental expansion
library. They do not describe the default exact engine. The release example
`audit_expansion` uses finite in-memory inputs and has explicit fixture size
limits. It does not load formats or read project files.

### Local group changes duplicate the entire expansion state

The first local assignment in each group clones every macro, register, alias,
and conditional through `current_expansion_state`. A fixture preloads 1,000
macros, each with 32 replacement tokens, then enters nested groups. Each
mutating group changes only one small local macro.

| Group depth | Read-only peak RSS | Mutating peak RSS | Mutating expansion time |
| --- | --- | --- | --- |
| 1 | 7.1 MiB | 8.1 MiB | 0.137 ms |
| 16 | 7.1 MiB | 22.5 MiB | 1.891 ms |
| 64 | 7.1 MiB | 68.4 MiB | 9.789 ms |

Expansion timings exclude definition setup. Both variants at depth 64 emit
129 tokens. The extra memory comes from full-state copies rather than a
larger output. This is a nested-scope scaling cost, not evidence of a leak
after groups close.

Use a save stack containing only changed bindings and share immutable macro
replacement tokens. Global assignments need a defined invalidation rule for
saved bindings. They should not require duplicating every unrelated definition.
Relevant code is `crates/tekai-pdftex/src/expand.rs` in
`ensure_current_scope_snapshot`, `current_expansion_state`, and scoped setters.

### Finite invalid primitive inputs aborted expansion

Both integer-expression parsers check multiplication overflow and division
by zero, but use unchecked division for `i64::MIN / -1`. A finite count
assignment followed by an advance and division aborted the release example
with exit code -6. The direct expression and the same expression inside
`\edef` reproduced separate sites, `read_integer_product` and
`read_integer_product_from_pending`.

`\pdfunescapehex{é}` also aborted with exit code -6. `pdf_unescape_hex` checks
an even UTF-8 byte count, then assumes an even character count. The two-byte,
one-character argument reaches an invalid `expect` before digit validation.

Both division sites now use `checked_div`. Hex decoding validates each ASCII
digit before pairing it and no longer allocates an intermediate filtered
string. The three release probes now return `ExpandError` with normal process
exit instead of aborting. Regression tests cover both parser paths, zero
divisors, valid negative division, invalid Unicode and ASCII hex, whitespace,
and valid hex output. These are input-error handling fixes, not valid TeX
output comparisons.

The broader numeric range contract and unchecked sign changes and register
advances still need review. The nested-scope copying cost remains open.

## JPEG resolution metadata now uses checked reads

The previous generated EXIF reader ignored incomplete reads and followed TIFF
offsets through raw pointers. A twelve-byte JPEG containing only the EXIF
signature reached an out-of-bounds byte-order read. Signed resolution division
also allowed the minimum integer divided by minus one to abort.

`jpeg_exif.rs` now validates the signature, TIFF header, field-table extent,
tag types and counts, and every referenced rational through checked slices.
Division is checked. Resolution changes apply only after the complete metadata
parses. Invalid optional metadata leaves the existing fallback intact. APP1
storage cannot exceed 65,533 bytes, and the adapter requires a complete read.
Invalid framing returns a normal engine input error. Positive resolution
division, centimetre conversion and final truncation retain their old behavior.

Ten unit tests cover both byte orders, every truncation of a valid payload,
offset and field-count boundaries, zero padding, unsupported types, absent
tags and arithmetic traps. All 88 full-profile native JPEG gates pass,
including three framing rejections, invalid metadata and valid DPI controls.
Successful controls require a fresh compilation, exact image bytes in the PDF,
one 1-by-1 image object, and the expected natural dimensions. These checks do
not establish independent image decoding or validate every JPEG segment.

The unchecked-read finding came from source review. No unfixed native crash
probe was run. High-bit signed resolution semantics remain unchanged.

## Remaining image ownership concerns

The image loader also uses manual allocation and handle cleanup through
`readimage`, `deleteimage`, and `img_free`. Full reentrant or long-lived engine
use needs ownership checks for each image type before removing the current
per-pass process boundary. Per-process retention is not proof of an
accumulating watcher leak.

## Font and PDF object allocation cleanup

Source review found three lost allocations in ordinary exact-engine paths.
No malformed font or PDF parser probe was needed to identify them.

- `ttf_read_cmap` allocated a private `cmap_tab` entry array that had no
  subsequent reads or release. The declaration and allocation are removed.
  The active cmap table and parsed cmap cache are unchanged.
- `t1_flush_cs` freed only the emitted subroutine prefix before freeing its
  containing table. Allocated unused subroutines above `subr_max` lost their
  owners. Cleanup now releases that tail as well. The emission loop, original
  subroutine indices, interior holes, counts and footer are unchanged.
- `avlputobj` allocated a candidate before `avl_probe`. When an equal key
  already existed, the tree retained its original entry but the rejected
  candidate was not released. Insertion now frees the rejected candidate
  without a second tree lookup or replacing the accepted entry. The allocation
  failure branch also releases its candidate before the existing fatal error.

Nine new tests exercise actual ownership operations in these helpers.
Seven count Type 1 releases and cover entry ranges, initialized null entries,
borrowed `.notdef` names, repeated cleanup and an untouched emitted prefix.
Two use locally owned AVL trees to count allocations and releases for unique
entries and 128 rejected duplicates, retaining the first object's identity
and balancing allocations at teardown. These tests
do not mutate the engine's global object table or parse untrusted fonts.

The relevant files are `generated/backend/writettf.rs`, `writet1.rs` and
`avlstuff.rs` under `crates/tekai-engine/src`. These are ownership-only changes,
so build-state version 41 is unchanged. They do not make the engine reentrant
or establish complete font and object cleanup. Its current process exit still
contains other per-pass retention.

The same review found unmeasured font scaling costs. `ttf_read_post` restarts
the custom-name buffer scan for successive name indices. `ttf_reindex_glyphs`
searches glyph names for each encoding entry, and Type 1 `cs_mark` searches
charstrings by name. The successful cmap cache does not share all parsed font
metadata, and non-subset TrueType output still performs its normal full
metadata parse before copying. Descriptor and encoding ownership need a
separate cleanup contract.
Some interned names alias encoding-owned storage, so an unconditional tree
destructor could double-free them. None of these broader costs is fixed here.

## Experimental renderer source review

The following findings concern the opt-in renderer in
`crates/tekai-pdftex/src/native.rs`, not the default exact engine. They are
source-derived worst cases, not measured timing or memory results. No new
native renderer probes were run in this review.

### Source scanning and inline cleaning

`take_until_command_or_blank` independently searches the remaining source for
every marker even when a nearby blank line already bounds the paragraph.
Repeated fixed-size paragraphs can therefore rescan overlapping suffixes
quadratically. Nearest-command selectors in file loading and pre-body
collectors use the same pattern when some command kinds are absent.
`find_control` can also rescan a growing line prefix for comment detection
before rejecting an alphabetic suffix match.

`clean_inline_text_collecting` copies the remaining character iterator into a
new string for many short inline constructs, recursively processes that
suffix, and normalizes each assembled result. Repeated constructs can produce
quadratic copied bytes and linear recursion depth. Input expansion retains
whole source strings along recursive include chains without an aggregate byte
or depth budget. Separate adapter and option collectors reread package graphs,
and inherited option vectors are cloned along nested dependencies.
`clean_math_text` and the loose text fallback use the same suffix-copying and
continuation-recursion pattern. Punctuation normalization is not idempotent,
so removing repeated cleanup passes without characterization tests could
change current output.

Lookup and option refactors must preserve first-match and inherited-option
semantics. A visited key consisting only of a canonical path can suppress a
later visit with different inherited options. The native search walk also has
different symlink and ordering rules from the exact engine's indexed resolver.
Replacing it silently would change behavior.

### Layout, graphics and output

Blank lookahead at a fresh two-column page can inspect the same blank suffix
repeatedly before a wide top float. Invisible overflow records duplicate full
placements that output consumers skip but summary counts still include.
Caption tracing copies complete text before producing a bounded excerpt.
Title padding can rerun layout over an already parsed prefix just to recover
its cursor. SyncTeX generation scans all placements again for every page.
Any refactor must preserve deferred-float ordering and public summaries.
Positive source lengths can also expand into an unbounded number of blank
line records. The length parser and float-derived slot conversion have no
finite-value or aggregate admission check. Faster lookahead alone would not
bound that source-to-layout amplification.

Graphics prewarming limits workers but retains every successful asset in an
unbounded per-parse cache. Different pages of one PDF each reload the complete
file. Dimension reads load complete PNG and JPEG files. Imported PDF streams
are copied during collection, remapping and serialization. These Rust-owned
values drop with their cache and document owners within the native invocation,
so this is not evidence of cross-build retention.
The native PNG path is separate from the exact engine's fixed PNG adapter.

PDF output retains page streams, object buffers and the final byte buffer
together. Every page receives an XObject dictionary containing all document
images, including unused images, so resource output grows with pages times
images. Shared in-memory image payloads do not deduplicate serialized objects
for repeated placements. Streaming output and per-page resource sets need
object-identity and output-fidelity tests before implementation.

Public native draft and no-PDF options also need review. Draft graphics has
no production read in this file, and no-PDF mode skips emission only after
parsing and layout. Current experimental CLI callers leave both flags false.
These findings do not claim a default CLI regression or a Rust ownership leak.

## Remaining review targets

The production linter, watcher, watch-event collector, resolver, PDF wrapper,
PDF import adapter, PNG adapter, low-level support routines, compiler production
code, and editor integration code have received detailed function-level review.
Compiler orchestration, cache publication, fingerprints, and parsers have also
been traced across files. The experimental expansion production code and
report-only dependency walker have now received function-level review.
The experimental renderer's include and package paths, supported-document
parser and pre-body analysis, 55 layout helpers, graphics loading and PDF
serialization have received targeted function-level review. The remaining
renderer helpers and their interactions still need review.
The generated JPEG, JBIG2, and image-loader implementations have received
function-level review. Font parsing and cleanup, the AVL object index and
compression finalization have received targeted review. Other generated
backends remain review targets.

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
- Experimental expansion has no aggregate token or scope-storage budget.
  Recursive numeric and nested expansion helpers need depth and cancellation
  checks. Their practical limits still need measurements.
- Editor indexing repeats source reads and has broad cache invalidation.
- Persistent artifact caches do not have a global disk-retention policy.

## Local regression checks for the current fixes

The current fixes passed 531 workspace library tests in debug and release,
11 self-contained CLI tests, 13 native shared-tree CLI tests, and 149 Python tool tests.
Both bundled large-paper fixtures passed their build gate. The real TeX
reference test skipped locally because this machine has no system TeX
installation. Workspace and standalone-engine lint checks, formatting, and
release and standalone builds passed. The separate upstream comparison passed
all 99 paper and transparent-image pages with matching text and pixels.
These are local results, not remote CI success.

The full release diagnostic profile passed 162 gates, recorded 29 observations,
and reproduced seven known failures with no unexpected failures or skips.
All 267 supervised commands completed without a timeout. Both isolated
executable copies matched their sources and retained matching start/end hashes.
The rotating-input watcher completed all ten edits. This checkpoint adds 88
JPEG gates and 27 PNG framing, copy and decode gates to the prior 47 passes.
The allocation checkpoint reran the same complete profile. Its first restricted
local attempt failed the watcher gate and skipped process inspection. The
approved rerun passed all gates, including all ten watch edits and the owned
process check. The restricted attempt is not counted as a passing run.
The compiler gates and 99-page comparison exercise the exact engine.
`audit_expansion` exercises the experimental expander, not the renderer.
The newly identified renderer scan and copy bounds are not yet CI gates.

Release input-identity probes now rebuild and produce the changed text for both
in-place edits and atomic replacement with preserved mtime. The database probe
also produces the newly selected package's text. The PNG probe accepts the valid
palette and rejects the oversized declaration.

The latest lookup probe still reproduces a stale cache hit after adding a
higher-priority project input. Its recursive symlink fixture now returns the
missing-file result without enumerating every alias path. Lookup dependencies,
build-generation races, cancellation, format companions, and the other open
findings remain.

## Continuous performance checks

`tools/performance_ci.py` separates fixed correctness gates from timings and
open failures. The quick profile covers lookup, linter checks, source identity,
PNG bounds, JPEG metadata, Unicode preview, cache hits, and decoded images. The full profile
adds every audit fixture, rotating watcher inputs, and experimental expansion.
Successful probes for unresolved scaling costs are observations, not evidence
that their complexity or memory use is fixed.

Known failures require the exact reproduced behavior and a working control
where applicable. An unrelated error still fails the run. Missing dependencies
are explicit skips, with strict CI options rejecting those skips. Wall time
and RSS stay outside correctness evidence. Unavailable RSS remains unavailable
rather than becoming zero.

Expected engine errors require the child's normal exit status, not just the
parent CLI's exit code. A wrapped signal or panic cannot pass the PNG bounds,
deep-input capacity, or PDF-cycle gates. Missing-file graph checks also require
the exact fixture input in the not-found diagnostic. The PDF-cycle check
accounts for TeX log wrapping without accepting a different error phrase.

```sh
python3 -B -m unittest discover -s tools -p 'test_*.py'
python3 -B tools/performance_ci.py --engine target/release/tekai --profile quick
cargo build --release --locked -p tekai-pdftex --example audit_expansion
python3 -B tools/performance_ci.py --engine target/release/tekai --profile full --require-expansion
python3 -B tools/verify_bundled_papers.py --engine target/release/tekai --images
```

The performance workflow runs quick release diagnostics for pull requests and
full diagnostics on ARM64 and Intel macOS for manual or weekly runs. Core CI
can call either profile from the same branch commit without a release or pull
request. Linux tests the portable Python supervisor on Python 3.10 and 3.13,
not an unsupported Linux runtime. Scheduled runs require the workflow on the
default branch.

Every compiler probe has a finite input and a command deadline. The supervisor
owns each process group, stops descendants, and reaps the direct child. All
runtime and artifact caches, home directories, and temporary inputs are
fixture-owned. The runner copies selected executables into an isolated binary
directory and checks their streamed hashes before starting fixtures. It also
checks both source and copied executables at the end, rejecting changes or
disappearance. No development sibling engine is copied. Endpoint hashes do
not detect a temporary replacement restored between checks.
Reports distinguish passed gates, known failures, observations,
and skips, and CI uploads them even after a failure. Release library tests
enforce deterministic scan and retention assertions. The full profile also
runs the separate all-page upstream PDF comparison.

Release library tests disable the optional C entrypoint, matching the actual
embedding dependency features. Without this setting, fat LTO collides with
the Rust test harness entrypoint before any tests execute. The corrected
command retains fat LTO and all workspace library tests.

The parity runner now isolates all four candidate caches and uses owned
process groups with bounded output capture and deadlines. It rejects oversized
successful text output rather than weakening the equality check. SIGTERM
cleans up the main-thread command. Rendering workers clean up within their
individual deadlines, so interruption can wait for those workers.

## Fix order and regression checks

First define shared contracts for build generations, lookup dependencies,
file identity, bounded auxiliary scheduling, and process cancellation. Fixing the stale-output and orphan
cases takes priority over improving benchmark numbers. Then remove repeated
linter scans and PDF clones, tighten the remaining graph-matching limits,
and stream job-scoped artifact caching.

Regression checks should test outcomes and bounded operation counts rather
than relying only on timing thresholds. Required cases include edits after
input consumption, creation of a higher-priority file, same-size database
replacement with preserved mtime, source replacement with preserved mtime,
inactive end markers, format replacement, cancellation of a live engine, a symlink
DAG, long Unicode and escape-heavy lines, a shared-resource multipage PDF,
a cyclic PDF parent, oversized PNG metadata, and unrelated output sidecars.
The experimental engine also needs group-storage accounting and error tests
for arithmetic boundaries and invalid Unicode hex input.

The next source-derived scaling gates should use small in-memory inputs and
counters at actual scan, copy and lookup operations. Renderer scanners can
share a forward cursor while retaining distinct raw-marker and control-word
matching rules. Blank lookahead can share a monotonic index without storing
an extra vector, but must stop at page styles and output controls.
Font name indices should borrow bytes, retain original first-match order and
drop before cleanup. TrueType custom-name offsets must preserve the current
NUL-separated indexing, including embedded NULs. Invalid or already-used
Type 1 entries must still shadow later duplicate names. These are proposed
gates and refactors, not passing checks or implemented fixes.

Any output-affecting fixes still need the real-paper fidelity comparison and
the ARM64 and Intel CI gates. The current audit records failures and proposed
remedies. It does not claim these newly found failures are fixed.

The first full remote run at `205a81f` passed both core macOS jobs and the
99-page text/pixel comparison on both architectures. Its release jobs failed
before library tests because the optional C entrypoint collided with the test
harness, and its preview gates exposed the unshipped standalone dependency.
Both issues have local regression fixes. That failed run is not recorded as
remote CI success for the corrected checkpoint.

The corrected checkpoint at `ac5a9e8` passed all six jobs in
[full CI](https://github.com/NicoNekoru/tekai/actions/runs/38004576383).
ARM64 and Intel both passed the installed-TeX reference gate, core Rust
checks, 522 release library tests, 162 diagnostic gates and the 99-page
text/pixel comparison. Both reports record 29 observations, seven known
failures, no unexpected failures or skips, 267 completed commands and all ten
watch edits. Downloaded artifacts identify that exact commit and unchanged
isolated executable copies. Linux runner tests passed on Python 3.10 and 3.13.
These remote results predate the allocation changes and their nine new tests.
The newer checkpoint's CI must be checked separately from this successful run.
