# Performance status

This is the pre-merge status ledger. Supported commands and measurement policy
live in [development instructions](development.md). Historical measurements and
the original reproductions live in the
[October investigation record](archive/performance-investigation-2026-10.md).

## Required gates

- Property tests and deterministic work, capacity and retention bounds.
- Real bundled-engine cache, cancellation and malformed-input fixtures.
- Fixed-DPI page pixels and extracted text against pinned upstream pdfTeX.
- Release-mode paired timing on free GitHub Intel and ARM runners.

The timing gate still requires 128 pairs and 32 balanced crossover units per
case. Its practical regression boundary is 10%, relative median absolute
deviation limit is 10%, and order-bias limit is 5%. Exit code 2 is blocking.
Diagnostic profiles and same-binary controls cannot replace a primary result.
The final comparison must use the actual `main` commit as baseline.

## Release blockers

The reproduced failures now have implementations and local regression tests.
Full remote verification of these changes is pending. Every fixture is a
required assertion, with no known-failure exemption.

| Failure | Change |
| --- | --- |
| Input changes during a build become a fresh cache entry | Capture a generation before compilation and validate it before publishing cache state. |
| New higher-priority inputs do not invalidate a cache hit | Retain bounded lookup-directory and alias identities as dependencies. Warm checks inspect those identities without enumerating subfolders. |
| Textual end markers hide active bytes | Hash complete source bytes rather than guessing execution from textual markers. |
| Replaced compressed formats reuse stale raw companions | Materialize into a private content-addressed cache, retain source dependencies and recover corrupt cache entries. Never write adjacent installation companions. |
| Parent cancellation leaves child processes running | Own subprocess groups, forward CLI cancellation, escalate stubborn groups and reap direct children. |

Auxiliary commands share an eight-child ceiling. Its CI fixture checks completed
outputs and maximum overlap, rather than treating completion as a scaling claim.
SIGKILL cannot execute parent cleanup. Library callers retain their signal policy
and must not independently reap children owned by the build scheduler.

## Rust migration

Supported macOS and Linux dependency builds pass with C and C++ compilers
disabled. BibTeX uses a Rust-only bridge, and compression uses Rust backends.
The engine stays a Rust library. Release builds retain fat LTO.

The deeper engine migration is unfinished. The exact engine's translated Rust
still uses C-shaped allocation, streams, formatting and internal ABI connections.
Removing these requires matching producer and consumer changes, shared Rust
types, and invocation-owned state. Necessary OS interfaces remain.

## Last completed remote result

[Run 38108239876](https://github.com/NicoNekoru/tekai/actions/runs/38108239876)
tested `d305dab`, before these fixes. Intel passed all four timing cases. ARM
image timing was inconclusive because candidate relative MAD was 10.734%.
The estimated ratio was 0.9951, so the result did not demonstrate a regression.
Both binaries slowed during the run. Missing CPU, memory and swap telemetry
prevented a reliable explanation. Stable parent RSS and thread counts did not
establish leak freedom in engine children.

Both architectures passed their core tests and 99-page parity checks, but each
reported seven reproduced known failures. Those historical results do not
validate the new fixes. Build times varied with runner conditions, so the
observed rlib-only build improvement is not a causal speedup estimate.

## Remaining performance work

Completion checks do not establish good asymptotic behavior. These costs remain
open unless a dedicated operation-count test proves otherwise.

- Repeated prefix scans and string edits in large formatting requests.
- Repeated column and escape scans in long lint inputs.
- Loading unrelated sidecars from settled auxiliary caches.
- Cloning shared PDF resources per page.
- Cloning expansion state for mutable local groups.

The active experimental renderer remains research code. Its general page parity
is not established. Retired proposal-only modules and the redundant manual
attribution route are recoverable from Git history.

No merge or release is authorized during this pass.
