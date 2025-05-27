# Examples for Arm Versatile Application Board

This package contains example binaries for the Arm Versatile Application
baseboard evaluation system, featuring an Arm Cortex-R5 processor or Arm
Cortex-A8 processor core. This crate should be compiled for the
`armv7r-none-eabihf`, `armv7r-none-eabi`, `armv7a-none-eabi` or
`armv7a-none-eabihf` targets. The repo-level [`.cargo/config.toml`] will
ensure the code runs on the appropriate QEMU configuration.

We have only tested this crate on `qemu-system-arm` emulating the Arm
Versatile Application Board, not the real thing.

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
