//! PL190 soft FIQ interrupt hello-world.

#![no_std]
#![no_main]

use portable_atomic::{AtomicU32, Ordering::SeqCst};

use aarch32_rt::entry;
use pl190_vic::{InterruptId, InterruptType, Pl190Driver};
use semihosting::println;

static MARKER: AtomicU32 = AtomicU32::new(0);

static PL190: Pl190Driver = unsafe { Pl190Driver::new_static(versatileab::PL190_BASE_ADDRESS) };

// We can pick any interrupt ID value here
const TEST_INTERRUPT1: InterruptId = InterruptId::new(1);

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn my_main() -> ! {
    versatileab::init();
    aarch32_cpu::asm::fiq_enable();

    println!("Setting up interrupts...");
    PL190.set_interrupt_selection(TEST_INTERRUPT1, InterruptType::Fiq);

    println!("Firing interrupt...");
    PL190.pend_sw_interrupt(TEST_INTERRUPT1);

    // wait for it
    for _ in 0..1_000 {
        if MARKER.load(SeqCst) == 1 {
            println!("Got interrupted :)");
            versatileab::exit(0);
        }
    }

    println!("Not interrupted!?");
    versatileab::exit(1);
}

/// Our assembly language FIQ handler
///
/// We don't bother with VIC interrupt vectoring - we just set MARKER to 1 and
/// clear the soft interrupt bit.
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[instruction_set(arm::a32)]
extern "C" fn _asm_fiq_handler() {
    // FIQ handlers can use R8 to R12 freely because FIQ mode has its own copies
    core::arch::naked_asm!(
        // set MARKER = 1
        "MOVS    R8, #1",
        "LDR     R9, ={marker}",
        "STR     R8, [R9]",
        // push/pop something just to see the watermark move
        "PUSH    {{ R8, R9 }}",
        "POP     {{ R8, R9 }}",
        // clear soft interrupt
        "LDR     R8, ={int_mask}",
        "LDR     R9, ={intclear}",
        "STR     R8, [R9]",
        // return from FIQ
        "SUBS    PC, LR, #4",
        marker = sym MARKER,
        int_mask = const { TEST_INTERRUPT1.to_mask() },
        intclear = const { versatileab::PL190_BASE_ADDRESS + 0x1C }
    );
}
