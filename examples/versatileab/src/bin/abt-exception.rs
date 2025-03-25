//! Example triggering an data abort exception.

#![no_std]
#![no_main]

use core::sync::atomic::AtomicU32;

use cortex_ar::register::{Dfar, Dfsr, Sctlr};
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
#[allow(unreachable_code)]
fn main() -> ! {
    // Enable alignment check for Armv7-R. Was not required
    // on Cortex-A for some reason, even though the bit was not set.
    enable_alignment_check();

    println!("Hello, this is an data abort exception example");
    // Unaligned read
    unsafe {
        let addr: *const u32 = 0x1001 as *const u32; // Unaligned address (not 4-byte aligned)
        core::arch::asm!(
            "ldr r0, [{addr}]",  // Attempt unaligned load (should trigger Data Abort)
            addr = in(reg) addr, // Pass unaligned pointer
            options(nostack, preserves_flags) // No stack usage, preserves flags
        );
    }

    unreachable!("should never be here!");
}

fn enable_alignment_check() {
    let mut sctrl = Sctlr::read();
    sctrl.set_a(true);
    Sctlr::write(sctrl);
}

fn disable_alignment_check() {
    let mut sctrl = Sctlr::read();
    sctrl.set_a(false);
    Sctlr::write(sctrl);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _undefined_handler(_faulting_instruction: u32) {
    panic!("unexpected undefined exception");
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _prefetch_handler(_faulting_instruction: u32) {
    panic!("unexpected prefetch exception");
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _abort_handler(_faulting_instruction: u32) {
    println!("data abort occurred");
    let dfsr = Dfsr::read();
    println!("DFSR (Fault Status Register): {:?}", dfsr);
    println!("DFSR Status: {:?}", dfsr.status());
    // If this is not disabled, reading DFAR will trigger an alignment fault on Armv8-R, leading
    // to a loop.
    disable_alignment_check();
    let dfar = Dfar::read();
    println!("DFAR (Faulting Address Register): {:?}", dfar);
    enable_alignment_check();
    // For the first iteration, we do a regular exception return, which should
    // trigger the exception again.
    let counter_val = COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed) + 1;
    if counter_val == 2 {
        semihosting::process::exit(0);
    }
}
