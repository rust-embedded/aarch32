//! PL190 soft interrupt hello-world.

#![no_std]
#![no_main]

use portable_atomic::{AtomicU32, Ordering::SeqCst};

use aarch32_rt::{entry, exception};
use pl190_vic::{InterruptId, Pl190Driver, VectorId};
use semihosting::println;

static MARKER: AtomicU32 = AtomicU32::new(0);

static PL190: Pl190Driver = unsafe { Pl190Driver::new_static(versatileab::PL190_BASE_ADDRESS) };

// We can pick any three interrupt ID values here
const TEST_INTERRUPT1: InterruptId = InterruptId::new(1);
const TEST_INTERRUPT2: InterruptId = InterruptId::new(2);
const TEST_INTERRUPT3: InterruptId = InterruptId::new(3);

// We can pick any two vector ID values here, as long as TEST_VECTOR1 is
// higher than TEST_VECTOR2 (i.e. TEST_VECTOR2 is higher priority than
// TEST_VECTOR1).
const TEST_VECTOR1: VectorId = VectorId::new(7);
const TEST_VECTOR2: VectorId = VectorId::new(4);

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn my_main() -> ! {
    // Safety: Not in a critical-section
    unsafe {
        aarch32_cpu::interrupt::enable();
    }

    println!("Setting up interrupts...");
    PL190.set_handler(TEST_INTERRUPT1, TEST_VECTOR1, Some(soft_handler1));
    PL190.set_handler(TEST_INTERRUPT2, TEST_VECTOR2, Some(soft_handler2));
    PL190.set_default_handler(catchall_handler);
    PL190.enable_interrupt(TEST_INTERRUPT1);
    PL190.enable_interrupt(TEST_INTERRUPT2);
    PL190.enable_interrupt(TEST_INTERRUPT3);

    println!("Firing interrupt...");
    PL190.pend_sw_interrupt(TEST_INTERRUPT1);

    // wait for it
    for _ in 0..1_000 {
        if MARKER.load(SeqCst) == 1 {
            println!("Got interrupted :)");
            // this one has no handler
            PL190.pend_sw_interrupt(TEST_INTERRUPT3);
            break;
        }
    }

    for _ in 0..1_000 {
        if MARKER.load(SeqCst) == 2 {
            println!("catch all works. All done!");
            semihosting::process::exit(0);
        }
    }

    println!("Not interrupted!?");
    semihosting::process::exit(1);
}

/// Our low-prio handler re-enables interrupts and triggers a second,
/// higher-priority, handler.
fn soft_handler1() {
    unsafe {
        aarch32_cpu::interrupt::enable();
    }
    println!("> soft_handler1()");
    PL190.clear_sw_interrupt(TEST_INTERRUPT1);
    PL190.pend_sw_interrupt(TEST_INTERRUPT2);
    println!("< soft_handler1()");
}

/// Our high-prio handler sets a global flag
fn soft_handler2() {
    println!("> soft_handler2()");
    PL190.clear_sw_interrupt(TEST_INTERRUPT2);
    MARKER.store(1, SeqCst);
    println!("< soft_handler2()");
}

/// Our catch-all handler sets a global flag
fn catchall_handler() {
    println!("catchall_handler() fired");
    PL190.clear_sw_interrupt(TEST_INTERRUPT3);
    MARKER.store(2, SeqCst);
}

/// Our IRQ handler asks the PL190 what to do, and does it.
#[exception(Irq)]
unsafe fn interrupt_handler() {
    println!("> interrupt_handler()");
    PL190.irq_process();
    println!("< interrupt_handler()");
}
