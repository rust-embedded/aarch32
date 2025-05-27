# Rust on Arm Cortex-R and Cortex-A

This repository provides support for:

* Armv7-R Processors, like the Arm Cortex-R5
* Armv8-R AArch32 Processors, like the Arm Cortex-R52
* Armv7-A Processors, like the Arm Cortex-A5
* Armv8-A AArch32 Processors, like the Arm Cortex-A53 running in 32-bit mode

These libraries were originally written by Ferrous Systems, and are based on the
[`cortex-m` libraries] from the [Rust Embedded Devices Working Group].

[`cortex-m` libraries]: https://github.com/rust-embedded/cortex-m
[Rust Embedded Devices Working Group]: https://github.com/rust-embedded

There are currently five libraries here:

* [cortex-ar](./cortex-ar/) - support library for Cortex-R and Cortex-A CPUs (like [cortex-m])
* [cortex-r-rt](./cortex-r-rt/) - run-time library for Cortex-R CPUs (like [cortex-m-rt])
* [cortex-a-rt](./cortex-a-rt/) - run-time library for Cortex-A CPUs (like [cortex-m-rt])
* [arm-targets](./arm-targets/) - a helper library for your build.rs that sets various `--cfg` flags according to the current target
* [cortex-ar-rt-macros](./cortex-ar-rt-macros/) - macros for `cortex-a-rt` and `cortex-r-rt` (this is an implementation detail - do not use this crate directly)

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
