# texpilot-pdftex architecture

This document describes the target architecture for the native
TeX/STY-to-PDF engine. The goal is a clean final build that is faster than one
external pdfTeX pass and eventually below one second on the bundled large arXiv
examples, while preserving rendered-PDF fidelity.

The compatibility contract is the PDF rendering. TeX/STY semantics, boxes,
glue, penalties, fonts, math layout, page breaking, and pdfTeX primitives matter
because they affect the PDF. Legacy sidecars such as `.aux`, `.toc`, `.bbl`,
`.brf`, `.idx`, `.fls`, logs, and SyncTeX are adapters or diagnostics. They must
not define the hot path.

## Current Limits

The repository currently has two useful but incomplete engine shapes.

`crates/pdftex-rust` is the faithful Rust-owned pdfTeX port. It is valuable as a
debug oracle, compatibility reference, and transitional engine, but its internal
architecture still inherits pdfTeX's pass-shaped Web2C model. The hot profile is
dominated by `getnext`, `macrocall`, `endtokenlist`, `zscantoks`, conditionals,
and linked-list node copy/free work. That means it spends much of the build
creating, replaying, and destroying short-lived token and node lists.

`crates/texpilot-pdftex/src/native.rs` is the fast native subset. It already
collapses aux-style reruns into in-memory document state for supported papers,
but it is closer to a document compiler than a TeX execution engine: it expands
to source, then performs separate source sweeps for metadata, labels,
citations, floats, lists, body rendering, layout, and PDF writing. That is a
good prototype, not the final architecture.

The target engine keeps the fidelity discipline of `pdftex-rust` and the
in-memory convergence discipline of `native.rs`, but uses a new dataflow. The
design below is intentionally adversarial about its own costs: a replacement
engine is not successful if it merely trades pdfTeX's token-list churn for
overlay lookups, checkpoint bloat, string-keyed registries, or cross-thread
synchronization.

## Target Dataflow

The hot path should be a single native engine run:

```text
memory-mapped format snapshot
  + preindexed distribution/project files
  + root TeX source
        |
        v
fused token executor
        |
        +--> document/layout state
        +--> cross-reference/bibliography/index state
        +--> asset/font prefetch jobs
        |
        v
paragraph builder -> page builder -> output routine
        |
        v
parallel PDF object writer -> final PDF
```

There are no ordinary aux-file reruns in this flow. Cross references,
bibliography labels, contents, bookmarks, float lists, backrefs, and index data
are typed in-memory registries. The engine may iterate in memory when a fixed
point is needed, but it must not restart the whole TeX process or rediscover
state through sidecar files. Fixed-point iteration is also not allowed to become
an unbounded replay mechanism; the default path should solve common reference
and citation convergence from typed registries and re-run only bounded document
regions when page-dependent text genuinely changes.

## Feasibility Check

The architecture is feasible only if these claims survive measurement:

- the snapshot removes LaTeX/package boot work without adding per-lookup overlay
  overhead in the executor;
- fused expansion preserves TeX's mouth/gullet/stomach ordering, including
  mutable catcodes, `\futurelet`, `\afterassignment`, `\aftergroup`,
  `\expandafter`, `\noexpand`, `\csname`, `\scantokens`, `\read`, `\write`,
  `\every...` token lists, `\the`, marks, inserts, and output-routine effects;
- fast macro paths bail out to exact generic paths for delimited parameters,
  `\long`/`\outer`, unusual catcodes, and expansion-sensitive argument scanning;
- typed document state is derived from executed TeX semantics, not from a
  parallel source parser that can drift from macro behavior;
- the page builder implements enough of TeX's box/glue/penalty and output
  routine semantics to explain every pixel-level divergence;
- asset and PDF parallelism does not add locks, channels, or memory copies that
  cost more than the serial work it replaces.

The project should treat a failed claim as a design bug, not as a reason to add
another compatibility layer. The fallback remains `pdftex-rust` or external
pdfTeX until the native path passes rendered parity gates.

## Components

### Distribution Index

The distribution index replaces repeated Kpathsea-style discovery on the clean
build hot path, but it must not become a large string-search service in the
middle of execution.

Responsibilities:

- preindex TeX tree files from `ls-R` and project-local roots;
- resolve `.tex`, `.sty`, `.cls`, `.bib`, `.bst`, font metrics, font programs,
  images, and included PDFs through one API;
- fingerprint files needed for invalidation;
- expose cheap negative lookups for missing optional inputs;
- preserve enough path-order behavior to match pdfTeX for rendered output.

The index is persistent and versioned by TeX distribution roots, environment
variables, engine version, and relevant project config.

Hot-path constraints:

- all successful lookups return interned path ids plus canonical byte paths;
- common extension probes use precomputed extension buckets, not filesystem
  stats;
- negative results are cached by `(base path id, name id, extension set)`;
- content fingerprints are lazy and restricted to actually used files;
- resolver tracing records why a path won so path-order bugs are debuggable
  without verbose hot-path logging.

### Format Snapshot

The format snapshot is the cold-start killer. It stores a LaTeX-ready base
state that can be memory-mapped instead of rebuilt for every clean build.

Snapshot contents:

- interned control sequence table;
- primitive dispatch table;
- catcode tables;
- eqtb/register state;
- macro definitions and token bodies;
- font metric declarations and hyphenation patterns;
- package/kernel state that is stable across documents.

The runtime should not use a hash-map overlay for hot state. The base snapshot
is immutable and shared across builds, but startup copies small hot arrays into
document-local contiguous memory:

- eqtb-equivalent command/register tables;
- active catcode table stack;
- small integer/dimension/skip registers;
- primitive dispatch ids and flags.

Large cold data stays memory-mapped:

- macro/token bodies from the format;
- string/interner tables;
- hyphenation tries and language data;
- font metric tables and distribution indexes.

This hybrid is less elegant than a universal copy-on-write layer, but it avoids
creating a new bottleneck where every primitive or control sequence lookup pays
an overlay probe.

### Token Representation

The executor should not preserve pdfTeX's linked token-list storage.

Use compact tokens in the hot path:

```text
Token = packed u32
  kind/cmd bits
  catcode or command id
  payload: character, interned control sequence id, register id, or side-table id
```

Long-lived macro bodies and token registers live in arena-backed token slices.
Short-lived macro arguments use stack frames or borrowed spans when possible.
Only TeX-visible persistence requires owned token materialization.

This directly attacks the `getnext` plus `macrocall` plus `endtokenlist` cost
cluster: the engine should usually advance through frames, not allocate a list,
push it through global memory, replay it, and free it.

Catcode rule: source byte spans may be tokenized lazily only while they remain
under the current input stream's active catcode table. Macro definitions,
arguments, token registers, marks, and `\write` payloads store canonical tokens
with the catcodes fixed at the moment TeX would have fixed them. Borrowing is an
optimization, not a semantic shortcut.

### Fused Executor

The fused executor replaces separate scanner, expander, macrocall, conditional,
and token-list replay loops with one state machine.

Core state:

- input frame stack: source byte spans, token slices, macro replacement spans,
  token-register spans, and synthetic primitive frames;
- current catcode table;
- mutable eqtb/register arrays;
- grouping stack;
- primitive dispatch table;
- document sink;
- diagnostics sink.

The executor has a tiny hot loop plus cold handlers. The hot loop handles
token-slice replay, ordinary character tokens, common control sequence dispatch,
and fixed-arity macro calls. Rare primitives, diagnostics, file I/O, and complex
parameter scanning live behind cold functions so the instruction cache is not
filled with every TeX feature.

The loop is shaped like:

```text
while let Some(token) = next_token_or_expansion_frame() {
    match dispatch_class(token) {
        expandable_macro => push_expansion_frame_without_list_churn(),
        expandable_primitive => execute_expandable_primitive(),
        conditional => execute_or_skip_with_specialized_skipper(),
        assignment => mutate_engine_state(),
        box_or_math_or_paragraph_token => send_to_typesetting_state(),
        output_primitive => send_to_pdf/page state(),
    }
}
```

Important fast paths:

- token-slice frame replay with no function call per token in common cases;
- specialized macro calls for fixed arity and delimited arguments;
- specialized `\if...` false-branch skipping that tracks nesting without fully
  materializing every skipped token as `curcmd/curchr`;
- interned control sequence lookup with cached hash and direct id comparison;
- primitive dispatch by dense id, not string matching;
- no allocation for common single-token and small-braced arguments;
- batch allocation only when a macro result must become a durable token list.

Correctness escape hatches:

- every fast path has a single exact generic path;
- a fast path records why it declined so performance misses can be counted;
- primitive handlers can request scanner/expander modes explicitly instead of
  smuggling state through globals;
- `\write`, `\read`, shell escape policy, and output-routine side effects are
  modeled as engine effects, not preprocessed strings.

### Document State

The executor emits typed events directly into the document model. The document
model owns functional state that legacy TeX exposes through aux files. It must
not be a second parser over expanded source.

Registries:

- labels and page references;
- citation uses, bibliography entries, and rendered citation labels;
- table of contents and bookmark entries;
- float/list entries;
- index entries;
- theorem/equation/section counters;
- PDF metadata and destination anchors.

The page builder may need a local fixed point when page numbers affect rendered
text. That should be solved by reprocessing affected regions from retained
execution checkpoints, not by restarting the whole document through sidecar
files.

Performance constraints:

- registries use dense ids for labels, citations, floats, destinations, and
  bibliography entries after initial interning;
- document events are buffered by paragraph/page region, not sent through a
  dynamic-dispatch callback for every token;
- sidecar export serializes from typed registries after PDF generation and is
  never required for PDF-only output;
- retained checkpoints are coarse and bounded: section/page-region checkpoints
  by default, with full replay only as a verifier/debug mode.

### Typesetting Core

The typesetting core should follow TeX's fidelity-sensitive algorithms, but not
its memory layout.

Required pieces:

- horizontal/vertical/math lists stored in typed arenas;
- Knuth-Plass line breaking with the same badness, demerit, penalty, glue, and
  hyphenation semantics needed by pdfTeX parity;
- page builder with insertions, top/bottom floats, marks, penalties, and output
  routine hooks;
- math layout using TeX font metrics and math parameters;
- box/glue/kern/penalty nodes represented as compact typed records;
- deterministic diagnostics hooks for parity investigation.

The node representation should support contiguous spans and typed node arrays.
Linked-list shape is allowed only where mutation requires it. Most paragraphs
and pages should be processed as compact vectors.

Future bottleneck to avoid: a fully object-oriented node graph would replace
pdfTeX's linked-list churn with pointer chasing of our own. The default should
be struct-of-arrays or compact typed arenas for hot node classes, with side
tables for rare node payloads. Layout algorithms should consume slices and
indices, not heap objects.

### Asset And Font Pipeline

Asset and font work should leave the expansion thread as soon as dependencies
are known, but only at coarse granularity.

Parallel jobs:

- image metadata probe;
- PNG/JPEG/PDF inclusion decode or copy-through;
- font metric and map lookup;
- Type 1/OpenType font program loading;
- glyph subset planning;
- PDF stream compression.

The expansion/layout thread receives stable handles and dimensions. The PDF
writer joins completed jobs when object bytes are needed.

Fast paths:

- copy-through JPEG and eligible PNG streams;
- avoid full PNG decode when the PDF-visible transform does not need pixels;
- persistent image/font cache keyed by content hash plus options;
- pre-resolved font maps from the distribution index.

Parallelism constraints:

- no per-token or per-node cross-thread messages;
- job submission is batched per discovered asset/font, not per use site;
- the layout thread never waits for full image decode when dimensions and PDF
  copy-through metadata are enough;
- PDF object assembly uses owned buffers per worker and one final ordered write;
- cache keys include rendering-affecting options, but not legacy diagnostics.

### PDF Backend

PDF object generation should be deterministic and parallel where object
independence allows it.

Responsibilities:

- assign stable object ids from the page/resource graph;
- build page content streams from layout output;
- emit font/image/form objects from prepared asset jobs;
- compress streams according to a performance policy;
- write xref/trailer deterministically;
- optionally export debug sidecars after the PDF path is complete.

Rendered parity does not require byte-for-byte PDF identity. Compression level,
object ordering, and metadata are allowed to differ unless a downstream cache
contract explicitly depends on them.

Future bottleneck to avoid: building every object as a formatted `String` before
copying into the final PDF. The writer should size or stream objects into byte
buffers directly, and it should keep compression policy independent from pixel
parity so draft/debug modes can choose cheaper output without affecting the
final fidelity gate.

## Fixed-Point Model

The fixed-point loop is in memory and bounded.

1. Execute the document once from the snapshot into document/layout state.
2. Build pages and compute page-dependent registries.
3. If page-dependent text changed, first update typed late-bound fields such as
   references, citations, bookmarks, and backrefs without re-executing TeX.
4. If macro execution genuinely depends on those fields, invalidate affected
   section/page-region checkpoints.
5. Re-execute only those retained regions or cheap summaries.
6. Repeat until rendered state is stable or a deterministic diagnostic explains
   why the document requires unsupported behavior.

The full source must not be reparsed through `.aux` to discover that labels,
citations, TOC entries, bookmarks, or backrefs changed.

Unbounded fixed-point iteration is a failure. The engine should report the
changed field, source region, and primitive/package feature that forced another
iteration. That makes pathological cases debuggable and prevents the native
path from hiding a slow rerun loop inside one process.

## Bottleneck Ledger

Each architectural win creates a possible new bottleneck. The implementation
should track these counters from the first skeleton:

```text
resolver.lookup_count
resolver.filesystem_stat_count
snapshot.hot_array_copy_bytes
snapshot.cold_page_faults
executor.tokens_read
executor.tokens_from_source
executor.tokens_from_slices
executor.macro_calls
executor.fast_macro_calls
executor.materialized_token_lists
executor.generic_scanner_fallbacks
executor.conditional_skip_tokens
document.events_emitted
document.registry_intern_lookups
layout.nodes_created
layout.node_bytes
layout.paragraphs_broken
layout.page_builder_iterations
fixed_point.iterations
fixed_point.regions_replayed
assets.jobs_submitted
assets.decode_bytes
assets.copy_through_bytes
pdf.objects
pdf.compressed_bytes
pdf.format_string_bytes
```

The subsecond target is not credible until these counters show that the new
engine erased the old work classes instead of renaming them.

## Integration With Current Repo

The new engine should live under `crates/texpilot-pdftex` as a v2 core rather
than inside generated `crates/pdftex-rust`.

Suggested module layout:

```text
crates/texpilot-pdftex/src/
  engine/
    mod.rs
    dist.rs
    snapshot.rs
    interner.rs
    token.rs
    catcode.rs
    executor.rs
    primitive.rs
    macro_call.rs
    condition.rs
    state.rs
    document.rs
    fixed_point.rs
    layout/
      paragraph.rs
      page.rs
      math.rs
      node.rs
    assets.rs
    fonts.rs
    pdf.rs
    diagnostics.rs
```

`crates/pdftex-rust` remains:

- the debug verifier;
- a source of algorithmic fixtures;
- a fallback engine while the new core is incomplete;
- a profiling baseline for functions that the new engine should erase or fuse.

`src/compiler.rs` should eventually dispatch `Engine::TexpilotPdftex` directly
to the v2 core when support probing succeeds. Certified mode may continue to run
pdfTeX as an oracle until parity gates are strong enough to retire it.

Do not route v2 through the existing approximate `native.rs` string compiler.
That path remains useful as a feature prototype and comparison point, but the
v2 executor must own token semantics and feed typed layout state directly.

## Migration Plan

### Phase 1: Skeleton And Instrumentation

Add the `engine/` module with inert data structures and a report-only runner.
The runner should emit a machine-readable stage report without producing a PDF.

Exit gate:

- both arXiv examples produce a dependency/input coverage report;
- report names unsupported primitive/package/asset classes;
- no new fallback behavior is hidden;
- the bottleneck ledger is emitted with zero/placeholder counters so later work
  cannot add invisible cost centers.

### Phase 2: Snapshot And Resolver

Build the distribution index and snapshot API. Initially snapshot only the
interner, primitive table, catcodes, and minimal LaTeX boot state, then expand.

Exit gate:

- cold-start resolver time is measurable and cached;
- project and TeX distribution lookup order matches pdfTeX for all files used by
  the arXiv examples;
- the engine can load a base state without executing the full preamble;
- hot state uses contiguous copied arrays, not a universal hash-map overlay.

### Phase 3: Fused Expansion Core

Implement compact tokens, input frames, grouping, assignments, registers,
macros, conditionals, and the primitive subset required to boot the target
documents.

Exit gate:

- focused fixtures match pdfTeX token/assignment behavior;
- hot-path reports show token-list materialization counts;
- common macro expansion uses frame replay, not owned-list churn;
- adversarial TeX fixtures cover mutable catcodes, `\futurelet`,
  `\afterassignment`, `\aftergroup`, `\expandafter`, `\noexpand`, `\csname`,
  `\scantokens`, `\read`, `\write`, delimited parameters, and conditionals.

### Phase 4: Typed Document Events

Route executor output into typed registries and paragraph/math/list builders.
Replace the string-sweep analyses in `native.rs` for labels, citations, TOC,
bookmarks, floats, and index data.

Exit gate:

- in-memory registries reproduce the rendered reference/citation text needed by
  the target examples;
- PDF-only mode does not materialize legacy sidecars;
- legacy sidecars can still be exported for debugging from typed state;
- registry and document-event counters show no per-token dynamic-dispatch sink.

### Phase 5: Fidelity Layout Core

Implement paragraph breaking, math layout, page building, output routines, font
metrics, and boxes with parity-first fixtures.

Exit gate:

- rendered pages are near-identical to pdfTeX on progressively larger slices of
  each arXiv document;
- divergence reports identify box position, glyph, image, or page-break causes;
- the old approximate string layout is no longer used for supported documents;
- node and page-builder counters show compact arena/vector behavior, not
  pointer-heavy graph traversal.

### Phase 6: Parallel Backend And Subsecond Push

Move asset/font/PDF object work off the executor thread and tune the hot loop
with profile-guided fast paths.

Exit gate:

- clean final builds beat one external PDF-producing pdfTeX pass;
- both arXiv examples build final native PDFs below one second on the benchmark
  host;
- rendered parity remains the release gate;
- parallel-worker counters show useful off-thread work without per-token or
  per-node synchronization.

## Performance Budget

For the large examples, the target budget is approximately:

```text
snapshot mmap + hot copy      20-60 ms
resolver used-file lookups    10-30 ms
fused expansion/execution   180-300 ms
paragraph/page/math layout  200-320 ms
assets/fonts/pdf writer     120-220 ms
fixed-point repair            0-80 ms
orchestration/reporting      10-30 ms
```

This budget is intentionally aggressive. It requires removing whole classes of
work, not making the existing `getnext` loop a little faster.

## Design Rules

- Preserve rendered output decisions; delete legacy compatibility costs unless
  they affect those decisions.
- Prefer durable typed state over sidecar text.
- Prefer frame replay and borrowed spans over token-list allocation.
- Prefer dense ids and arenas over string keys and linked nodes in hot paths.
- Keep the verifier path separate from the production path.
- Measure the two arXiv examples on every major architectural change.
- Every approximation must have a named unsupported reason or a parity ticket.
