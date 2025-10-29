# Rust on Arm AArch32

This repository provides support for:

* Legacy Arm Processors, like the ARM7TDMI and ARM926
* Armv7-R Processors, like the Arm Cortex-R5
* Armv8-R AArch32 Processors, like the Arm Cortex-R52
* Armv7-A Processors, like the Arm Cortex-A5
* Armv8-A AArch32 Processors, like the Arm Cortex-A53 running in 32-bit mode

It does not support any M-Profile Processors (like the Arm Cortex-M3) as they
have a fundamentally different interrupt vector table.

It also does not support processors running in AArch64 mode - A64 machine code
uses different instructions for reading/writing system registers.

These libraries were originally written by Ferrous Systems, and are based on the
[`cortex-m` libraries] from the [Rust Embedded Devices Working Group].

[`cortex-m` libraries]: https://github.com/rust-embedded/cortex-m
[Rust Embedded Devices Working Group]: https://github.com/rust-embedded

There are currently five libraries here:

* [aarch32](./aarch32/) - support library for AArch32 CPUs (like the [cortex-m] crate)
* [aarch32-rt](./aarch32-rt/) - run-time library for AArch32 CPUs (like the [cortex-m-rt] crate)
* [arm-targets](./arm-targets/) - a helper library for your build.rs that sets various `--cfg` flags according to the current target
* [aarch32-rt-macros](./aarch32-rt-macros/) - macros for `aarch32-rt` (this is an implementation detail - do not use this crate directly)

There are also example programs for QEMU in the [examples](./examples/) folder.

[cortex-m]: https://crates.io/crates/cortex-m
[cortex-m-rt]: https://crates.io/crates/cortex-m-rt

## Licence

* Copyright (c) Ferrous Systems
* Copyright (c) The Rust Embedded Devices Working Group developers

Licensed under either [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
