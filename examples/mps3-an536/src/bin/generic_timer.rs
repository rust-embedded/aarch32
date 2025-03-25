//! Generic-timer example for Arm Cortex-R52

#![no_std]
#![no_main]

// pull in our start-up code
use mps3_an536 as _;

use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-m-rt`.
#[no_mangle]
pub extern "C" fn boot_core(cpu_id: u32) -> ! {
    match cpu_id {
        0 => {
            main();
        }
        _ => panic!("unexpected CPU ID {}", cpu_id),
    }
}

/// Let's test some timers!
fn main() -> ! {
    use cortex_ar::generic_timer::{El1PhysicalTimer, El1VirtualTimer, GenericTimer};
    let cntfrq = cortex_ar::register::Cntfrq::read().0;
    println!("cntfrq = {:.03} MHz", cntfrq as f32 / 1_000_000.0);

    let delay_ticks = cntfrq * 2;

    let mut pgt = unsafe { El1PhysicalTimer::new() };
    let mut vgt = unsafe { El1VirtualTimer::new() };

    let pgt_ref: &mut dyn GenericTimer = &mut pgt;
    let vgt_ref: &mut dyn GenericTimer = &mut vgt;

    for (timer, name) in [(pgt_ref, "physical"), (vgt_ref, "virtual")] {
        println!("Using {} timer ************************", name);

        println!("Print five, one per second...");
        for i in 0..5 {
            println!("i = {}", i);
            timer.delay_ms(1000);
        }

        let now = timer.counter();
        println!("{} is now: {}", name, now);
        println!("Waiting for {} {} ticks to count up...", delay_ticks, name);
        timer.counter_compare_set(now + delay_ticks as u64);
        timer.enable(true);
        while !timer.interrupt_status() {
            core::hint::spin_loop();
        }
        println!("Matched! {} count now {}", name, timer.counter());

        println!(
            "Waiting for {} {} ticks to count down...",
            delay_ticks, name
        );
        timer.countdown_set(delay_ticks);
        while !timer.interrupt_status() {
            core::hint::spin_loop();
        }
        println!(
            "{} countdown hit zero! (and is now {})",
            name,
            timer.countdown() as i32
        );
    }
    semihosting::process::exit(0)
}
