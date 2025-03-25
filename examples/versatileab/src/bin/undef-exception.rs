//! Example triggering an undefined exception.

#![no_std]
#![no_main]

use core::sync::atomic::AtomicU32;

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
    println!("Hello, this is an undefined exception example");
    unsafe {
        core::arch::asm!("udf #0");
    }
    unreachable!("should never be here!");
}

#[no_mangle]
unsafe extern "C" fn _undefined_handler(_faulting_instruction: u32) {
    println!("undefined exception occurred");
    // For the first iteration, we do a regular exception return, which should
    // trigger the exception again.
    let counter_val = COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed) + 1;
    if counter_val == 2 {
        semihosting::process::exit(0);
    }
}
