//! Example triggering a prefetch abort exception.

#![no_std]
#![no_main]

use portable_atomic::{AtomicU32, Ordering};

use aarch32_cpu::register::{Ifar, Ifsr};
use aarch32_rt::{entry, exception};
use semihosting::println;
use versatileab as _;

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    println!("Hello, this is a prefetch abort exception example");

    // A BKPT instruction triggers a Prefetch Abort except when Halting debug-mode is enabled.
    // See p. 2038 of ARMv7-M Architecture Reference Manual
    unsafe {
        // trigger an prefetch abort exception, from T32 (Thumb) mode
        bkpt_from_t32();
    }

    println!("Recovered from fault OK!");

    semihosting::process::exit(0);
}

// These functions are written in assembly
extern "C" {
    fn bkpt_from_t32();
}

core::arch::global_asm!(
    r#"
    // fn bkpt_from_t32();
    .thumb
    .global bkpt_from_t32
    .type bkpt_from_t32, %function
    bkpt_from_t32:
        bkpt    #0
        bx      lr
    .size bkpt_from_t32, . - bkpt_from_t32
"#
);

#[exception(Undefined)]
fn undefined_handler(_addr: usize) -> ! {
    panic!("unexpected undefined exception");
}

#[exception(PrefetchAbort)]
unsafe fn prefetch_abort_handler(addr: usize) -> usize {
    println!("prefetch abort occurred");
    let ifsr = Ifsr::read();
    println!("IFSR (Fault Status Register): {:?}", ifsr);
    println!("IFSR Status: {:?}", ifsr.status());

    if cfg!(not(any(
        arm_architecture = "v4t",
        arm_architecture = "v5te",
        arm_architecture = "v6"
    ))) {
        let ifar = Ifar::read();
        println!("IFAR (Faulting Address Register): {:?}", ifar);

        if (addr + 1) == bkpt_from_t32 as unsafe extern "C" fn() as usize {
            // note that thumb functions have their LSB set, despite always being a
            // multiple of two - that's how the CPU knows they are written in T32
            // machine code.
            println!("caught bkpt_from_t32");
        } else {
            println!(
                "Bad fault address {:08x} is not {:08x}",
                addr, bkpt_from_t32 as unsafe extern "C" fn() as usize
            );
        }
    }

    let counter = COUNTER.load(Ordering::Relaxed);
    COUNTER.store(counter + 1, Ordering::Relaxed);
    match counter {
        0 => {
            // first time, huh?
            // go back and do it again
            println!("Doing it again");
            addr
        }
        1 => {
            // second time, huh?
            // go back but skip the instruction
            println!("Skipping instruction");
            addr + 2
        }
        _ => {
            // we've faulted thrice - time to quit
            panic!("prefetch_handler called too often");
        }
    }
}

#[exception(DataAbort)]
fn data_abort_handler(_addr: usize) -> ! {
    panic!("unexpected data abort exception");
}
