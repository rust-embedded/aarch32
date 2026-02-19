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
$ cargo run --bin hello
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -audio none -kernel target/armv8r-none-eabihf/debug/hello`
Hello, this is semihosting! x = 1.000, y = 2.000
PANIC: PanicInfo {
    message: I am an example panic,
    location: Location {
        file: "src/bin/hello.rs",
        line: 20,
        column: 5,
    },
    can_unwind: true,
    force_no_backtrace: false,
}
$ cargo run --bin hello --target thumbv8r-none-eabihf -Zbuild-std=core
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running `qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -audio none -kernel target/thumbv8r-none-eabihf/debug/hello`
Hello, this is semihosting! x = 1.000, y = 2.000
PANIC: PanicInfo {
    message: I am an example panic,
    location: Location {
        file: "src/bin/hello.rs",
        line: 20,
        column: 5,
    },
    can_unwind: true,
    force_no_backtrace: false,
}
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
