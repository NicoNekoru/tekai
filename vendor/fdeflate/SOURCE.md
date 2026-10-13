# Pinned upstream source

This directory imports the crates.io package `fdeflate` version `0.3.7`.
The local registry archive was checked against the package checksum in
Cargo.lock before vendoring.

- Registry archive SHA-256 `1e6853b52649d4ac5c0bd02320cddc5ba956bdb407c4b75a2c6b75bf51500f8c`.
- Package-recorded upstream revision `c365c7e6ffa81feb2e1fb762eed7299f05c9b0ca`.
  This revision comes from the package's `.cargo_vcs_info.json`, not a claim
  about the current upstream HEAD.
- Upstream repository [image-rs/fdeflate](https://github.com/image-rs/fdeflate).
- Upstream license `MIT OR Apache-2.0`. Both license files are retained.

The imported `Cargo.toml` came from the package's original `Cargo.toml.orig`,
rather than its registry-generated manifest. The initial import preserves all six Rust source
files, README, licenses and the three existing `.zz` regression assets
byte-for-byte. It omits the upstream lockfile and Cargo-generated metadata.

Local production changes in `src/decompress.rs` replace overlapping 16-byte
copies for short-distance matches with bounds-checked prefix doubling. Both
full-output-room branches retain their original admission checks. Distance-one
filling, longer distances and end-buffer copying remain unchanged.

Fragmented-output property tests also exposed an upstream checksum defect.
Partial queued-match returns skipped their published bytes when updating
Adler32. Both returns now account those bytes when checksum checking is enabled.
Completed drains keep their existing accounting. Valid and corrupt-footer
controls exercise both queued backreferences and run-length matches. Those same
controls also require zero-space reads to preserve an actual pending match and
its checksum, then withhold the final footer byte until the payload has drained.
Valid footers finish without further output space; corrupt footers still fail.
These are checksum-enabled decoder API tests, not a change to PNG checksum policy.

The workspace adds a `proptest` development dependency. Exhaustive copy tests
and bounded, independently encoded streaming properties accompany the changes.
Work and dispatch counters compile only in tests. Existing upstream tests and
their binary regression assets remain enabled. One redundant test iterator
conversion is removed for the workspace's strict Clippy check. Both licenses
are unchanged.

The package include list also retains this provenance file and the persisted
property regression seed. The inherited `rust-version = "1.67.0"` describes
the upstream library, not the workspace's expanded test suite. The added
development dependency requires newer Rust. Tekai supports Rust 1.89 and checks
the vendored suite with that toolchain.
