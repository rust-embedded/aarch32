# Macros for `cortex-a-rt` and `cortex-r-rt`

This crate contains proc-macros that are re-exported through the following crates:

* [`cortex-a-rt`]
* [`cortex-r-rt`]

[cortex-a-rt]: https://crates.io/crates/cortex-a-rt
[cortex-r-rt]: https://crates.io/crates/cortex-r-rt

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.82.0 and up, as recorded
by the `package.rust-version` property in `Cargo.toml`.

Increasing the MSRV is not considered a breaking change and may occur in a
minor version release (e.g. from `0.3.0` to `0.3.1`, because this is still a
`0.x` release).

## Licence

* Copyright (c) Ferrous Systems
* Copyright (c) The Rust Embedded Devices Working Group developers

Licensed under either [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
