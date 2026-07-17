# Contributing

Bug reports and focused pull requests are welcome. Start with an issue when a
change affects engine fidelity, cache semantics, or the public CLI contract.

Before opening a pull request:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --locked
```

Output-affecting changes must also pass the fixed-DPI rendered comparison on the
two large fixtures. Performance claims should use release binaries, real paper
roots, repeated matched trials, and an explicit correctness gate. See
[Development](docs/development.md) for the complete procedure.

Keep commit messages to one printable ASCII line. Keep public help, usage docs,
config parsing, and tests synchronized.
