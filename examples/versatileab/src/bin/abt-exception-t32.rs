//! Example triggering an data abort exception.

#![no_std]
#![no_main]

use portable_atomic::{AtomicU32, Ordering};

use aarch32_cpu::register::Sctlr;
#[cfg(not(arm_architecture = "v4t"))]
use aarch32_cpu::register::{Dfar, Dfsr};
use aarch32_rt::{entry, exception};
use semihosting::println;
use versatileab as _;

#[unsafe(no_mangle)]
static COUNTER: AtomicU32 = AtomicU32::new(0);

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    versatileab::init();
    // Enable alignment check for Armv7-R. Was not required
    // on Cortex-A for some reason, even though the bit was not set.
    enable_alignment_check();

    println!("Hello, this is an data abort exception example");

    // Unaligned read
    unaligned_from_t32();

    // turn it off before we do the stack dump on exit, because println! has been
    // observed to do unaligned reads.
    disable_alignment_check();

    println!("Recovered from fault OK!");

    versatileab::exit(0);
}

#[unsafe(naked)]
#[instruction_set(arm::t32)]
extern "C" fn unaligned_from_t32() {
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

    #[cfg(not(arm_architecture = "v4t"))]
    {
        // If this is not disabled, reading DFAR will trigger an alignment fault on Armv8-R, leading
        // to a loop.
        disable_alignment_check();
        let dfsr = Dfsr::read();
        println!("DFSR (Fault Status Register): {:?}", dfsr);
        let dfar = Dfar::read();
        enable_alignment_check();

        let expect_fault_from = core::ptr::addr_of!(COUNTER) as usize + 1;

        if dfar.0 as usize == expect_fault_from {
            println!("caught fault on COUNTER");
        } else {
            println!(
                "Bad DFAR address {:08x} is not {:08x}",
                dfar.0, expect_fault_from
            );
        }
    }

    // note the fault isn't at the start of the function
    let expect_fault_at = unaligned_from_t32 as extern "C" fn() as usize + 3;

    if addr == expect_fault_at {
        println!("caught unaligned_from_t32");
    } else {
        println!(
            "Bad fault address {:08x} is not {:08x}",
            addr, expect_fault_at
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
            addr + 2
        }
        _ => {
            // we've faulted thrice - time to quit
            println!("We triple faulted");
            semihosting::process::abort();
        }
    }
}
