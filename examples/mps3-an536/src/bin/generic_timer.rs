//! Generic-timer example for Arm Cortex-R52

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r_rt::entry;

// pull in our library
use mps3_an536 as _;

use cortex_ar::generic_timer::GenericTimer;

use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-r-rt`.
#[entry]
fn main() -> ! {
    let mut board = mps3_an536::Board::new().unwrap();

    let cntfrq = cortex_ar::register::Cntfrq::read().0;
    println!("cntfrq = {:.03} MHz", cntfrq as f32 / 1_000_000.0);

    let delay_ticks = cntfrq / 2;

    let physical_timer_ref: &mut dyn GenericTimer = &mut board.physical_timer;
    let virtual_timer_ref: &mut dyn GenericTimer = &mut board.virtual_timer;

    for (timer, name) in [
        (physical_timer_ref, "physical"),
        (virtual_timer_ref, "virtual"),
    ] {
        println!("Using {} timer ************************", name);

        println!("Print five, every 100ms...");
        for i in 0..5 {
            println!("i = {}", i);
            timer.delay_ms(100);
        }

        let now = timer.counter();
        println!("Waiting for {} {} ticks to count up...", delay_ticks, name);
        timer.counter_compare_set(now + delay_ticks as u64);
        timer.enable(true);
        while !timer.interrupt_status() {
            core::hint::spin_loop();
        }
        println!("Matched! {}", name);

        println!(
            "Waiting for {} {} ticks to count down...",
            delay_ticks, name
        );
        timer.countdown_set(delay_ticks);
        while !timer.interrupt_status() {
            core::hint::spin_loop();
        }
        println!("{} countdown hit zero!", name,);
    }

    semihosting::process::exit(0);
}
