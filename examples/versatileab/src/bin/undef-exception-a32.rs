//! Example triggering a undef exception.

#![no_std]
#![no_main]

use portable_atomic::{AtomicU32, Ordering};

use aarch32_rt::{entry, exception};
use semihosting::println;
use versatileab as _;

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    versatileab::init();
    println!("Hello, this is a undef exception example");

    // trigger an Undefined exception, from A32 (Arm) mode
    udf_from_a32();

    println!("Recovered from fault OK!");

    versatileab::exit(0);
}

#[unsafe(naked)]
#[instruction_set(arm::a32)]
extern "C" fn udf_from_a32() {
    core::arch::naked_asm!(
        // Do a UDF
        "udf     #0",
        // Return
        "bx      lr",
    );
}

#[exception(PrefetchAbort)]
fn prefetch_abort_handler(_addr: usize) -> ! {
    panic!("unexpected undefined exception");
}

#[exception(Undefined)]
unsafe fn undefined_handler(addr: usize) -> usize {
    println!("undefined abort occurred");

    if addr == udf_from_a32 as extern "C" fn() as usize {
        println!("caught udf_from_a32");
    } else {
        panic!(
            "Bad fault address {:08x} is not {:08x}",
            addr, udf_from_a32 as extern "C" fn() as usize
        );
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
            addr + 4
        }
        _ => {
            // we've faulted thrice - time to quit
            panic!("_undefined_handler called too often");
        }
    }
}

#[exception(DataAbort)]
fn data_abort_handler(_addr: usize) -> ! {
    panic!("unexpected data abort exception");
}
