# tekai-engine

`tekai-engine` is the self-contained exact typesetting engine embedded in the
`tekai` CLI. It owns the TeX execution core, format loading, file discovery,
font handling, PDF inclusion and writing, compression, image decoding, and the
runtime interfaces needed by the default `tekai-engine` build path.

The crate produces a Rust library only. Both the CLI and standalone runner
call its Rust entrypoint directly. It does not produce `libtekai_engine.a`
or provide a C `main` entrypoint for external TeX Live linking. Internal core
and backend calls use Rust functions and shared Rust record definitions.
Necessary operating-system interfaces remain separate from the Rust engine.

The engine is built entirely by Cargo and does not link a system TeX engine,
Kpathsea, libpng, or PDF library. Packages, fonts, maps, encodings, and the
filename index come from the pinned embedded data in `runtime/texmf.tar.gz`.
No system TeX distribution or `kpsewhich` is used. Personal `TEXMFHOME` and
site-wide `TEXMFLOCAL` addition trees are shared with installed TeX tools.
`TEKAI_TEXMF_MODE=bundled` disables their automatic lookup. Explicit search
paths support recursive `//`, variable/tilde/brace expansion, default insertion,
and database-only `!!` lookup. See the [runtime reference](../../runtime/README.md).

`build.rs` embeds the checked-in archives and format as read-only object data
on macOS/Linux ARM64 and x86-64, keeping their bytes out of Rust compiler
metadata. Other targets use static byte slices. Asset changes trigger a
rebuild, and tests compare the embedded bytes with their source files. This
reduces build-folder size without stripping debugger information or requiring
an additional compiler, download, or runtime file beside the executable.

The default CLI keeps a dedicated child process for cancellation and fault
containment. The library does not require process isolation for state reuse.
Its unsafe `run_from_args` and `run_with_options` entries own core state,
argument bytes, raw buffers, streams, decoders, compression and lookup caches
for each call. Normal completion, fatal diagnostics and help/version exits
return a typed result without terminating the host. Unexpected Rust panics
propagate after teardown. Calls may repeat or run independently on separate
threads. Nested entries on one thread are rejected.

`InvocationOptions` supplies an owned directory, an environment overlay and an
interrupt token without changing the host directory, environment or signal
handlers. Recovery requires unwind-enabled builds, and the crate rejects
`panic=abort`. Host callers must not independently reap engine pipe children
or configure SIGCHLD to reap them automatically.

The CLI registers its own signal adapters outside the engine. Its supervising
parent owns the worker group until completion and stops residual descendants
before reaping the worker. Nested engine/PGF jobs inherit that managed group.
Standalone execution uses the same whole-job cleanup policy. Reusable-library
calls instead own private pipe groups and stop abandoned commands on teardown.
Explicit `pclose` and `system` preserve POSIX rules for deliberately released
background commands, which remain the library caller's responsibility.
Private commands stopped by SIGTTIN or SIGTTOU report a normal I/O error and
are cleaned up without transferring the host terminal. This also covers
commands reopening `/dev/tty` after standard descriptors were redirected.
`PipeProcessPolicy::CallerManagedProcess` explicitly inherits the host's
terminal job control and makes descendant cleanup the host's responsibility.
SIGKILL of a supervising parent cannot run cleanup, and kernel-uninterruptible
processes cannot be given a hard termination deadline.

Translated pointer operations and format readers still require trusted input.
The library is not a security sandbox, and engine ownership alone does not
establish that every malformed font or format is memory-safe.
Filename search expressions currently require UTF-8. Invalid requests fail
rather than selecting a replacement spelling. Direct file and decoder paths
preserve OS filename bytes.

For focused debugging, build its standalone executable with:

```sh
cargo build \
  -p tekai-engine \
  --bin tekai-engine \
  --no-default-features \
  --features standalone-binary
```

Validation is behavior-first:

```sh
cargo test -p tekai-engine
cargo test --workspace
```

Changes that can affect layout or PDF output must also pass the repository's
fixed-DPI rendered-page comparison on both large paper fixtures. See
[`docs/development.md`](../../docs/development.md).
