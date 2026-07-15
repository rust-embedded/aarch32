//! # Multi-core hello-world for the Arm Cortex-A9 on a Xilinx Zynq-7000
//!
//! Boots the second core and checks that atomics and critical sections work
//! across both cores.

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use aarch32_rt::entry;
use semihosting::println;
use xilinx_zynq_a9 as _;

/// Set by Core 1 once it has booted.
static CORE1_BOOTED: AtomicBool = AtomicBool::new(false);

/// Incremented by both cores using an atomic read-modify-write.
static SHARED_VARIABLE: AtomicU32 = AtomicU32::new(0);

/// Incremented by both cores from inside a critical section.
static SHARED_VARIABLE_2: critical_section::Mutex<RefCell<u32>> =
    critical_section::Mutex::new(RefCell::new(0));

/// How long Core 0 waits for Core 1.
const CORE0_WILL_WAIT: usize = 1_000_000;

/// How many atomic-add loops each core runs.
const CAS_LOOPS: u32 = 1000;

/// How many critical-section loops each core runs.
const CS_MUTEX_LOOPS: u32 = 1000;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch32-rt` on Core 0.
#[entry]
fn main() -> ! {
    xilinx_zynq_a9::init();

    println!(
        "I am core 0 - {:08x?}",
        aarch32_cpu::register::Mpidr::read()
    );

    xilinx_zynq_a9::start_core1();

    // Wait some time for Core 1 to start.
    for counter in 0..=CORE0_WILL_WAIT {
        if CORE1_BOOTED.load(Ordering::SeqCst) {
            break;
        }
        if counter == CORE0_WILL_WAIT {
            println!("CPU 1 is missing?!");
            xilinx_zynq_a9::exit(0);
        }
    }

    for _ in 0..CAS_LOOPS {
        SHARED_VARIABLE.fetch_add(1, Ordering::Relaxed);
    }

    for _ in 0..CS_MUTEX_LOOPS {
        critical_section::with(|cs| {
            let mut value_ref = SHARED_VARIABLE_2.borrow_ref_mut(cs);
            *value_ref += 1;
        })
    }

    // Let the other core finish.
    for _ in 0..CORE0_WILL_WAIT {
        aarch32_cpu::asm::nop();
    }

    let mut code = 0;
    let total_a = SHARED_VARIABLE.load(Ordering::Relaxed);
    if total_a == CAS_LOOPS * 2 {
        println!("CAS test passed");
    } else {
        println!("CAS test failed, got {} not 2000", total_a);
        code = 1;
    }

    let total_b = critical_section::with(|cs| {
        let value_ref = SHARED_VARIABLE_2.borrow_ref(cs);
        *value_ref
    });

    if total_b == CS_MUTEX_LOOPS * 2 {
        println!("CS Mutex test passed");
    } else {
        println!("CS Mutex test failed, got {} not 2000", total_b);
        code = 1;
    }

    xilinx_zynq_a9::exit(code);
}

/// The entry-point to the Rust application on Core 1.
///
/// Called by the start-up code once Core 0 has released this core.
#[unsafe(no_mangle)]
pub extern "C" fn kmain_secondary() {
    // Each core enables its own MMU before it does any atomics: the exclusive
    // monitor used by `fetch_add` and the critical section needs normal,
    // shareable memory, which the MMU provides.
    xilinx_zynq_a9::init();

    println!(
        "I am core 1 - {:08x?}",
        aarch32_cpu::register::Mpidr::read()
    );
    CORE1_BOOTED.store(true, Ordering::SeqCst);

    for _ in 0..CAS_LOOPS {
        SHARED_VARIABLE.fetch_add(1, Ordering::Relaxed);
    }

    for _ in 0..CS_MUTEX_LOOPS {
        critical_section::with(|cs| {
            let mut value_ref = SHARED_VARIABLE_2.borrow_ref_mut(cs);
            *value_ref += 1;
        })
    }

    loop {
        aarch32_cpu::asm::wfi();
    }
}
