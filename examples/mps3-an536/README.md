# Examples for Arm MPS3-AN536

This package contains example binaries for the Arm MPS3-AN536 evaluation
system, featuring one or two Arm Cortex-R52 processor cores. This crate
should be compiled for the `armv8r-none-eabihf` target. The repo-level
[`.cargo/config.toml`] will ensure the code runs on the appropriate QEMU
configuration.

As of Rust 1.87, `armv8r-none-eabihf` is a Tier 3 target and so these examples
require Nightly Rust.

We have only tested this crate on `qemu-system-arm` emulating the Arm
MPS3-AN536, not the real thing.

[`.cargo/config.toml`]: ../../.cargo/config.toml

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.82.0 and up, as recorded
by the `package.rust-version` property in `Cargo.toml`. These examples are
not version controlled and we may change the MSRV at any time.

## Licence

* Copyright (c) Ferrous Systems
* Copyright (c) The Rust Embedded Devices Working Group developers

Licensed under either [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
