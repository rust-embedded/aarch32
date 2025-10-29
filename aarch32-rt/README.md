[![crates.io](https://img.shields.io/crates/v/aarch32-rt)](https://crates.io/crates/aarch32-rt)
[![docs.rs](https://img.shields.io/docsrs/aarch32-rt)](https://docs.rs/aarch32-rt)

# Run-time support for Arm AArch32 Processors

This library implements a simple Arm vector table, suitable for getting into a
Rust application running in System Mode. It also provides a reference start up
method. Some Arm AArch32 based systems will require chip specific start-up
code, so the start-up method can be overridden. It should be suitable for:

* Legacy Arm Processors, like the ARM7TDMI and ARM926
* Armv7-R Processors, like the Arm Cortex-R5
* Armv8-R AArch32 Processors, like the Arm Cortex-R52
* Armv7-A Processors, like the Arm Cortex-A5
* Armv8-A AArch32 Processors, like the Arm Cortex-A53 running in 32-bit mode

It does not support any M-Profile Processors (like the Arm Cortex-M3) as they
have a fundamentally different interrupt vector table.

It also does not support processors running in AArch64 mode - A64 machine code
uses different instructions for reading/writing system registers.

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
