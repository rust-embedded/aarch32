//! Example triggering an prefetch exception.

#![no_std]
#![no_main]

use core::sync::atomic::AtomicU32;

use cortex_ar::register::{Ifar, Ifsr};
// pull in our start-up code
use versatileab as _;

use semihosting::println;

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in the runtime crate.
#[no_mangle]
pub extern "C" fn boot_core(cpu_id: u32) {
    match cpu_id {
        0 => {
            main();
        }
        _ => panic!("unexpected CPU ID {}", cpu_id),
    }
}

/// The main function of our Rust application.
#[export_name = "main"]
fn main() -> ! {
    println!("Hello, this is an prefetch exception example");

    // A BKPT instruction triggers a Prefetch Abort except when Halting debug-mode is enabled.
    // See p. 2038 of ARMv7-M Architecture Reference Manual
    unsafe {
        core::arch::asm!("bkpt");
    }

    unreachable!("should never be here!");
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _undefined_handler(_faulting_instruction: u32) {
    panic!("unexpected undefined exception");
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _prefetch_handler(_faulting_instruction: u32) {
    println!("prefetch abort occurred");
    let ifsr = Ifsr::read();
    println!("IFSR (Fault Status Register): {:?}", ifsr);
    println!("IFSR Status: {:?}", ifsr.status());
    let ifar = Ifar::read();
    println!("IFAR (Faulting Address Register): {:?}", ifar);
    // For the first iteration, we do a regular exception return, which should
    // trigger the exception again.
    let counter_val = COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed) + 1;
    if counter_val == 2 {
        semihosting::process::exit(0);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _abort_handler(_faulting_instruction: u32) {
    panic!("unexpected abort exception");
}
