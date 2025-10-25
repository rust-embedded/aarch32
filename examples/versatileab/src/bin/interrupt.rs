//! PL190 soft interrupt hello-world.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering::SeqCst};

use aarch32_rt::{entry, exception};
use semihosting::println;
use versatileab::Pl190;

static MARKER: AtomicU32 = AtomicU32::new(0);

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn my_main() -> ! {
    let mut pl190 = Pl190::create();

    // Safety: Not in a critical-section
    unsafe {
        aarch32_cpu::interrupt::enable();
    }

    println!("Firing interrupt...");
    pl190.write_vic_intenable(1);
    pl190.write_vic_softint(1);

    // wait for it
    for _ in 0..1_000 {
        if MARKER.load(SeqCst) == 1 {
            println!("Got interrupted :)");
            semihosting::process::exit(0);
        }
    }

    println!("Not interrupted!?");
    semihosting::process::exit(1);
}

#[exception(Irq)]
unsafe fn interrupt_handler() {
    println!("Clearing interrupt...");
    let mut pl190 = Pl190::create();
    pl190.write_vic_softintclear(1);
    MARKER.store(1, SeqCst);
}
