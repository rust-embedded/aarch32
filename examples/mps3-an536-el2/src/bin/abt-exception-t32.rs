//! Example triggering an data abort exception.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use aarch32_cpu::register::Hsctlr;
use aarch32_rt::{entry, exception};
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
    unsafe {
        // Unaligned read
        unaligned_from_t32();
    }

    // turn it off before we do the stack dump on exit, because println! has been
    // observed to do unaligned reads.
    disable_alignment_check();

    println!("Recovered from fault OK!");

    mps3_an536_el2::exit(0);
}

// These functions are written in assembly
unsafe extern "C" {
    fn unaligned_from_t32();
}

core::arch::global_asm!(
    r#"
    // fn unaligned_from_t32();
    .thumb
    .global unaligned_from_t32
    .type unaligned_from_t32, %function
    unaligned_from_t32:
        ldr     r0, =COUNTER
        add     r0, r0, 1
        ldr     r0, [r0]
        bx      lr
    .size unaligned_from_t32, . - unaligned_from_t32
"#
);

fn enable_alignment_check() {
    let mut hsctrl = Hsctlr::read();
    hsctrl.set_a(true);
    Hsctlr::write(hsctrl);
}

fn disable_alignment_check() {
    let mut hsctrl = Hsctlr::read();
    hsctrl.set_a(false);
    Hsctlr::write(hsctrl);
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
    let hsr = aarch32_cpu::register::Hsr::read();
    disable_alignment_check();
    println!("data abort occurred {:?} {:x?}", hsr, hsr.get_iss());
    enable_alignment_check();

    // note the fault isn't at the start of the function
    let expect_fault_at = unaligned_from_t32 as unsafe extern "C" fn() as usize + 5;

    if addr == expect_fault_at {
        println!("caught unaligned_from_t32");
    } else {
        println!(
            "Bad fault address {:08x} is not {:08x}",
            addr, expect_fault_at
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
            addr + 2
        }
        _ => {
            // we've faulted thrice - time to quit
            println!("We triple faulted");
            semihosting::process::abort();
        }
    }
}
