//! Example triggering a undef exception.

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
    println!("Hello, this is a undef exception example");

    unsafe {
        // trigger an Undefined exception, from T32 (Thumb) mode
        udf_from_t32();
    }

    println!("Recovered from fault OK!");

    mps3_an536_el2::exit(0);
}

// These functions are written in assembly
unsafe extern "C" {
    fn udf_from_t32();
}

core::arch::global_asm!(
    r#"
    // fn udf_from_t32();
    .thumb
    .global udf_from_t32
    .type udf_from_t32, %function
    udf_from_t32:
        udf     #0
        bx      lr
    .size udf_from_t32, . - udf_from_t32
"#
);

#[exception(PrefetchAbort)]
fn prefetch_abort_handler(_addr: usize) -> ! {
    panic!("unexpected undefined exception");
}

#[exception(Undefined)]
unsafe fn undefined_handler(addr: usize) -> usize {
    let hsr = aarch32_cpu::register::Hsr::read();
    println!(
        "undefined abort occurred {:08x?}, {:x?}",
        hsr,
        hsr.get_iss()
    );

    if (addr + 1) == udf_from_t32 as unsafe extern "C" fn() as usize {
        // note that thumb functions have their LSB set, despite always being a
        // multiple of two - that's how the CPU knows they are written in T32
        // machine code.
        println!("caught udf_from_t32");
    } else {
        println!(
            "Bad fault address {:08x} is not {:08x}",
            addr, udf_from_t32 as unsafe extern "C" fn() as usize
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
            addr + 2
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
