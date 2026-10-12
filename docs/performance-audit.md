# Release verification status

This ledger records the verified implementation through `bd9bf45` on
12 October 2026. Supported commands and measurement policy live in
[development instructions](development.md). Earlier reproductions, ownership
checkpoints and remote comparisons remain in the
[October investigation record](archive/performance-investigation-2026-10.md).

## Current evidence

[Run 38210192247](https://github.com/NicoNekoru/tekai/actions/runs/38210192247)
tested `bd9bf45` against actual `main` at `0b6e92e`. Both macOS ARM64 and
Intel core jobs passed, including the bundled large-paper gate, real
installed-TeX lookup, Rust checks and compiler integrations. Both architectures
also passed release library tests, required runtime regressions and all
99 reference pages with matching pixels and extracted text. The portable
Python runner tests passed on Python 3.10 and 3.13.

The hosted paired comparisons retained their original MAD-blocking policy and
returned inconclusive. They did not demonstrate a regression.

| Case | Intel | ARM64 |
| --- | --- | --- |
| CLI startup | Inconclusive | Pass |
| Nested lookup | Inconclusive | Inconclusive |
| Image compilation | Pass | Pass |
| Warm build cache | Pass | Pass |

A separate complete local ARM64 comparison used the same baseline and candidate,
immutable source snapshots, the same canonical source and fresh target paths,
Rust 1.99.0 and matching release settings. Candidate compilation disabled C and
C++ compilers. All four cases passed under the original MAD-blocking policy,
with 128 pairs and 32 complete crossover units per case. It completed in
3524.9 seconds without errors.

| Case | Candidate / baseline | Median interval |
| --- | ---: | --- |
| CLI startup | 0.9971 | 0.9844 to 1.0116 |
| Nested lookup | 0.1468 | 0.1458 to 0.1473 |
| Image compilation | 0.9717 | 0.9663 to 0.9760 |
| Warm build cache | 0.9913 | 0.9849 to 0.9981 |

Executable and fixture hashes stayed unchanged. Warm-cache output, all
1024 referenced inputs and state content and identity remained verified.
The complete local report, raw command journal and retained-artifact hashes
remain under `/tmp/tekai-local-timing.bd9bf45.IRZYnD/`. This result applies to
these workloads on that Mac, not hosted-runner stability, universal speed,
asymptotic complexity or leak freedom.

The local bundled PGF regression covers explicit and forced list-and-make modes,
warm figure reuse, missing-figure regeneration, picture edits and child-only
dependencies. It verifies that child-only edits regenerate the affected figure
while the parent source, recorder and picture hash stay unchanged. System-TeX
PGF tests skipped on the local machine but ran in the hosted core jobs.
Optional tests that return early can still appear as harness passes. They must
not be counted as exercised compatibility workflows.

## Final-candidate gates

The results above precede the timing-policy update. Before release, the final
prepared candidate must repeat property tests and deterministic work bounds,
bundled-engine cache and
cancellation fixtures, malformed-input regressions, reference PDF pixels and
text, and paired release timing on both supported macOS architectures.
Required runtime assertions have no known-failure exemption.

The approved prospective timing policy makes relative MAD of pooled
per-artifact absolute batch times diagnostic rather than an independent veto.
It retains 128 pairs, 32 balanced crossover units, the 10% practical relative
slowdown boundary and multiplicity-adjusted median-ratio intervals. Fixed minimum
batch durations, the 5% order-bias guard, complete matched samples, output
oracles, executable and fixture integrity, cache-state checks and owned-process
deadlines remain blocking. Insufficient interval precision remains inconclusive,
with exit 2.

This change applies to newly collected final-candidate evidence. Historical
reports keep their original policy and verdicts. No samples are discarded or
rescored to make the hosted results green. The final comparison must record the
actual `main` baseline and the prepared candidate, not substitute the local
result or diagnostic profiles.

## Scoped correctness guarantees

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

Subprocess ownership forwards CLI cancellation, stops descendants and reaps
direct children. Auxiliary commands share an eight-child ceiling. SIGKILL
cannot run parent cleanup, and library callers retain their own signal policy.

Equal and nested output folders use sorted, deduplicated descriptor locks.
Cleanup holds an exclusive lease while builds share it. Persistent lock
retention is bounded by 769 files. These guarantees require cooperating upgraded
processes. Hash collisions can serialize unrelated folders, continuous builds
can postpone cleanup, and custom caches inside another build's output folder
still need separate placement constraints.

## Rust migration and remaining limits

Candidate builds pass with C and C++ compiler commands disabled. BibTeX uses a
Rust-only bridge and compression uses Rust backends. Rust owns allocation
wrappers, integer formatting, stream backends, argument storage and checked
integer/backend parsers. Shared core types and layout tests preserve format
compatibility. Checked font parsers cover the implemented TrueType post and
OpenType name paths, SFD fields, encoding-copy intervals and ToUnicode updates.
These checks do not make every font reader a complete hostile-input parser.

Disabling native compilers does not remove native runtime calls. Three
floating-point scans, C string utilities, option parsing and the native regex
compiler/matcher remain production dependencies. Regex result storage is
bounded, but a Rust replacement still needs implementation and verification.
Backend ABI connections, raw-pointer callers and mutable global ownership also
remain unfinished. Necessary operating-system interfaces are separate from
those replaceable utilities.

The exact engine accepts one dedicated-process invocation. Argument backing
stays alive for globals that borrow it, and normal completion flushes streams
and returns a typed result. Fatal exits can bypass Rust drops, and several
destructors leave stale global roots. Repeated in-process execution, complete
invocation-owned state and leak freedom are not established. Rust panics must
not be caught across existing non-unwinding internal calls.

The active experimental renderer remains research code without general page
parity. Neither its architectural goals nor complete native-utility replacement
should be reported as completed release work. Release support remains macOS,
and the exact engine remains a Rust library with fat-LTO release builds.

## Follow-up performance work

Dedicated operation-count tests are still needed for repeated formatting and
lint scans, unrelated settled-cache sidecar loading, shared PDF resource copies,
expansion-state copies and repeated auxiliary-phase recorder reads. Streaming
removes complete file copies and duplicate retained paths, but does not cap the
longest recorder row or every unique dependency set.

Full-byte hashing uses 64 KiB of scratch per active call. Output snapshots have
at most eight hash workers, bounding their hash scratch at 512 KiB. This does
not bound total process memory or concurrent independent builds. Remaining font
readers need ownership and bounds, and the compact name-layout planner is not a
globally optimal packing solver.
