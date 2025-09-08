[![crates.io](https://img.shields.io/crates/v/cortex-a-rt)](https://crates.io/crates/cortex-a-rt)
[![docs.rs](https://img.shields.io/docsrs/cortex-a-rt)](https://docs.rs/cortex-a-rt)

# Run-time support for Arm Cortex-A (AArch32)

This library implements a simple Arm vector table, suitable for getting into a
Rust application running in System Mode. It also provides a reference start
up method. Most Cortex-A based systems will require chip specific start-up
code, so the start-up method can be overridden.

See <https://docs.rs/cortex-a-rt> for detailed documentation.

## Features

- `vfp-dp`: Enables support for the double-precision VFP floating point support. If your target
  CPU has this feature or support for NEON which also implies double-precision support, this
  feature should be activated.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.83.0 and up, as recorded
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
