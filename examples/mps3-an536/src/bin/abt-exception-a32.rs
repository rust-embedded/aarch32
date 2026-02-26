//! Example triggering an data abort exception.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use aarch32_cpu::register::{Dfar, Dfsr, Sctlr};
use aarch32_rt::{entry, exception};
use mps3_an536 as _;
use semihosting::println;

#[unsafe(no_mangle)]
static COUNTER: AtomicU32 = AtomicU32::new(0);

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    // Enable alignment check for Armv7-R. Was not required
    // on Cortex-A for some reason, even though the bit was not set.
    enable_alignment_check();

    println!("Hello, this is an data abort exception example");

    // Unaligned read
    unaligned_from_a32();

    // turn it off before we do the stack dump on exit, because println! has been
    // observed to do unaligned reads.
    disable_alignment_check();

    println!("Recovered from fault OK!");

    mps3_an536::exit(0);
}

#[unsafe(naked)]
#[instruction_set(arm::a32)]
extern "C" fn unaligned_from_a32() {
    core::arch::naked_asm!(
        "ldr     r0, =COUNTER",
        "adds    r0, r0, 1",
        "ldr     r0, [r0]",
        "bx      lr",
    );
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

#[exception(Undefined)]
fn undefined_handler(_addr: usize) -> ! {
    panic!("unexpected undefined exception");
}

#[exception(PrefetchAbort)]
fn prefetch_abort_handler(_addr: usize) -> ! {
    panic!("unexpected prefetch abort");
}

#[exception(DataAbort)]
unsafe fn data_abort_handler(addr: usize) -> usize {
    println!("data abort occurred");
    // If this is not disabled, reading DFAR will trigger an alignment fault on Armv8-R, leading
    // to a loop.
    disable_alignment_check();
    let dfsr = Dfsr::read();
    println!("DFSR (Fault Status Register): {:?}", dfsr);
    let dfar = Dfar::read();
    enable_alignment_check();

    // note the fault isn't at the start of the function
    let expect_fault_at = unaligned_from_a32 as extern "C" fn() as usize + 8;

    if addr == expect_fault_at {
        println!("caught unaligned_from_a32");
    } else {
        println!(
            "Bad fault address {:08x} is not {:08x}",
            addr, expect_fault_at
        );
        semihosting::process::abort();
    }

    let expect_fault_from = core::ptr::addr_of!(COUNTER) as usize + 1;

    if dfar.0 as usize == expect_fault_from {
        println!("caught fault on COUNTER");
    } else {
        println!(
            "Bad DFAR address {:08x} is not {:08x}",
            dfar.0, expect_fault_from
        );
        semihosting::process::abort();
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
            println!("We triple faulted");
            semihosting::process::abort();
        }
    }
}
