# Examples for Arm MPS3-AN536

This package contains example binaries for the Arm MPS3-AN536 evaluation system,
featuring one or two Arm Cortex-R52 processor cores. This crate is tested on the
following targets:

- `armv8r-none-eabihf` - ARMv8-R AArch32, hard-float, Arm mode
- `thumbv8r-none-eabihf` - ARMv8-R AArch32, hard-float, Thumb mode

The repo-level [`.cargo/config.toml`] will ensure the code runs on the
appropriate QEMU configuration.

As of Rust 1.92, `armv8r-none-eabihf` is a Tier 2 target and so any stable
release from 1.92 or newer should work for that target. However,
`thumbv8r-none-eabihf` is still a Tier 3 target, which means Nightly Rust is
required. This folder contains a [`rust-toolchain.toml`] which pins us to a
specific release of nightly that is known to work.

We have only tested this crate on `qemu-system-arm` emulating the Arm
MPS3-AN536, not the real thing.

[`.cargo/config.toml`]: ../../.cargo/config.toml
[`rust-toolchain.toml`]: ./rust-toolchain.toml

## Running

Run these examples as follows:

```console
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -audio none -smp 2 -kernel target/armv8r-none-eabihf/debug/smp-test`
I am core 0 - Mpidr(80000000)
I am core 1 - Mpidr(80000001)
CAS test passed
CS Mutex test passed
Stack usage report:
SYS0 Stack =   2680 used of  16384 bytes (016%) @ 0x1006bf80..0x1006ff80
FIQ0 Stack =      0 used of     64 bytes (000%) @ 0x1006ff80..0x1006ffc0
IRQ0 Stack =      0 used of     64 bytes (000%) @ 0x1006ffc0..0x10070000
ABT0 Stack =      0 used of  16384 bytes (000%) @ 0x10070000..0x10074000
SVC0 Stack =      0 used of  16384 bytes (000%) @ 0x10074000..0x10078000
UND0 Stack =      0 used of  16384 bytes (000%) @ 0x10078000..0x1007c000
HYP0 Stack =      0 used of  16384 bytes (000%) @ 0x1007c000..0x10080000
SYS1 Stack =    680 used of  16384 bytes (004%) @ 0x10000018..0x10004018
FIQ1 Stack =      0 used of     64 bytes (000%) @ 0x10004018..0x10004058
IRQ1 Stack =      0 used of     64 bytes (000%) @ 0x10004058..0x10004098
ABT1 Stack =      0 used of  16384 bytes (000%) @ 0x10004098..0x10008098
SVC1 Stack =      0 used of  16384 bytes (000%) @ 0x10008098..0x1000c098
UND1 Stack =      0 used of  16384 bytes (000%) @ 0x1000c098..0x10010098
HYP1 Stack =      0 used of  16384 bytes (000%) @ 0x10010098..0x10014098
$ cargo run --target thumbv8r-none-eabihf -Zbuild-std=core
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -audio none -smp 2 -kernel target/thumbv8r-none-eabihf/debug/smp-test`
I am core 0 - Mpidr(80000000)
I am core 1 - Mpidr(80000001)
CAS test passed
CS Mutex test passed
Stack usage report:
SYS0 Stack =   4840 used of  16384 bytes (029%) @ 0x1006bf80..0x1006ff80
FIQ0 Stack =      0 used of     64 bytes (000%) @ 0x1006ff80..0x1006ffc0
IRQ0 Stack =      0 used of     64 bytes (000%) @ 0x1006ffc0..0x10070000
ABT0 Stack =      0 used of  16384 bytes (000%) @ 0x10070000..0x10074000
SVC0 Stack =      0 used of  16384 bytes (000%) @ 0x10074000..0x10078000
UND0 Stack =      0 used of  16384 bytes (000%) @ 0x10078000..0x1007c000
HYP0 Stack =      0 used of  16384 bytes (000%) @ 0x1007c000..0x10080000
SYS1 Stack =   1568 used of  16384 bytes (009%) @ 0x10000018..0x10004018
FIQ1 Stack =      0 used of     64 bytes (000%) @ 0x10004018..0x10004058
IRQ1 Stack =      0 used of     64 bytes (000%) @ 0x10004058..0x10004098
ABT1 Stack =      0 used of  16384 bytes (000%) @ 0x10004098..0x10008098
SVC1 Stack =      0 used of  16384 bytes (000%) @ 0x10008098..0x1000c098
UND1 Stack =      0 used of  16384 bytes (000%) @ 0x1000c098..0x10010098
HYP1 Stack =      0 used of  16384 bytes (000%) @ 0x10010098..0x10014098
```

## Debugging

You can start a GDB server by adding `-- -s -S` to the end of the `cargo run`
command, and the connect with GDB as follows:

```console
$ cargo run --bin hello -- -s -S
# QEMU runs and hangs waiting for a connection. In another terminal run:
$ arm-none-eabi-gdb -x commands.gdb target/armv8r-none-eabihf/debug/hello
# GDB will start and connect to QEMU's GDB server. The commands.gdb file sets up some useful defaults.
```

## Minimum Supported Rust Version (MSRV)

These examples are guaranteed to compile on the version of Rust given in the
[`rust-toolchain.toml`] file. These examples are not version controlled and we
may change the MSRV at any time.

## Licence

- Copyright (c) Ferrous Systems
- Copyright (c) The Rust Embedded Devices Working Group developers

Licensed under either [MIT](../LICENSE-MIT) or [Apache-2.0](../LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
