# Examples for the Xilinx Zynq-7000

This package contains example binaries for the Xilinx Zynq-7000 SoC, featuring a
dual-core Arm Cortex-A9 processor. This crate is tested on the following
targets:

- `armv7a-none-eabi` - ARMv7-A, soft-float, Arm mode
- `armv7a-none-eabihf` - ARMv7-A, hard-float, Arm mode
- `thumbv7a-none-eabi` - ARMv7-A, soft-float, Thumb mode
- `thumbv7a-none-eabihf` - ARMv7-A, hard-float, Thumb mode

The [`.cargo/config.toml`] in this folder will ensure the code runs on the
appropriate QEMU configuration (the `xilinx-zynq-a9` machine with a `cortex-a9`
CPU).

This folder contains a [`rust-toolchain.toml`] which pins us to a specific
release of nightly that is known to work.

We have only tested this crate on `qemu-system-arm` emulating the Xilinx
Zynq-7000, not the real thing.

[`.cargo/config.toml`]: ./.cargo/config.toml
[`rust-toolchain.toml`]: ./rust-toolchain.toml

## Examples

- `hello` - prints a message over semihosting and then panics
- `gic` - sets up the memory-mapped GIC (the GICv2 programming model), sends a
  Software Generated Interrupt to this core, and handles it
- `smp` - boots the second core and checks that atomics and critical sections
  work across both cores

The Zynq-7000 is a dual-core device, so QEMU is run with `-smp 2`. The
single-core examples simply leave the second core parked.

## Running

Run these examples as follows:

```console
$ cargo run --bin hello
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
     Running `qemu-system-arm -machine xilinx-zynq-a9 -cpu cortex-a9 -semihosting -nographic -audio none -smp 2 -kernel target/armv7a-none-eabihf/debug/hello`
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

The `smp` example prints the following:

```console
$ cargo run --bin smp
I am core 0 - Mpidr(80000000)
I am core 1 - Mpidr(80000001)
CAS test passed
CS Mutex test passed
```

## Debugging

You can start a GDB server by adding `-- -s -S` to the end of the `cargo run`
command, and then connect with GDB as follows:

```console
$ cargo run --bin hello -- -s -S
# QEMU runs and hangs waiting for a connection. In another terminal run:
$ arm-none-eabi-gdb target/armv7a-none-eabihf/debug/hello
# Then, at the GDB prompt, connect to QEMU's GDB server:
(gdb) target remote localhost:1234
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
