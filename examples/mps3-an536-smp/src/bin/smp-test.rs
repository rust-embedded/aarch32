//! Multi-core hello-world for Arm Cortex-R
//!
//! Runs code on two cores, checking that atomic fetch_add works.

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use aarch32_rt::entry;
use semihosting::println;

static CORE1_BOOTED: AtomicBool = AtomicBool::new(false);

static SHARED_VARIABLE: AtomicU32 = AtomicU32::new(0);

static SHARED_VARIABLE_2: critical_section::Mutex<RefCell<u32>> =
    critical_section::Mutex::new(RefCell::new(0));

/// How long core 0 waits for core 1
const CORE0_WILL_WAIT: usize = 1_000_000;

/// How many CAS loops to run?
const CAS_LOOPS: u32 = 1000;

/// How many CS Mutex loops to run?
const CS_MUTEX_LOOPS: u32 = 1000;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch32-rt`.
#[entry]
fn main() -> ! {
    println!(
        "I am core 0 - {:08x?}",
        aarch32_cpu::register::Mpidr::read()
    );

    mps3_an536_smp::start_core1();

    // wait some time for core 1 to start
    for counter in 0..=CORE0_WILL_WAIT {
        if CORE1_BOOTED.load(Ordering::SeqCst) {
            break;
        }
        if counter == CORE0_WILL_WAIT {
            println!("CPU 1 is missing?!");

            mps3_an536_smp::exit(0);
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

    // let the other core finish
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

    mps3_an536_smp::exit(code);
}

/// The entry-point to the Rust application.
///
/// It is called by the start-up code below, on Core 1.
#[unsafe(no_mangle)]
pub extern "C" fn kmain2() {
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
