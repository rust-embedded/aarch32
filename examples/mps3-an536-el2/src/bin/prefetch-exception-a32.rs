//! Example triggering a prefetch abort exception.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use aarch32_rt::{entry, exception};
use semihosting::println;

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
        // trigger an prefetch abort exception, from A32 (Arm) mode
        bkpt_from_a32();
    }

    println!("Recovered from fault OK!");

    mps3_an536_el2::exit(0);
}

// These functions are written in assembly
unsafe extern "C" {
    fn bkpt_from_a32();
}

core::arch::global_asm!(
    r#"
    // fn bkpt_from_a32();
    .arm
    .global bkpt_from_a32
    .type bkpt_from_a32, %function
    bkpt_from_a32:
        bkpt    #0
        bx      lr
    .size bkpt_from_a32, . - bkpt_from_a32
"#
);

// Custom link sections are allowed as well.
#[exception(Undefined)]
fn undefined_handler(_addr: usize) -> ! {
    panic!("unexpected undefined exception");
}

#[exception(PrefetchAbort)]
unsafe fn prefetch_abort_handler(addr: usize) -> usize {
    let hsr = aarch32_cpu::register::Hsr::read();
    println!("prefetch abort occurred {:08x?}, {:x?}", hsr, hsr.get_iss());

    if addr == bkpt_from_a32 as unsafe extern "C" fn() as usize {
        println!("caught bkpt_from_a32");
    } else {
        println!(
            "Bad fault address {:08x} is not {:08x}",
            addr, bkpt_from_a32 as unsafe extern "C" fn() as usize
        );
    }

    match COUNTER.fetch_add(1, Ordering::Relaxed) {
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
            addr + 4
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
