//! Simple FPU test

#![no_std]
#![no_main]

// pull in our start-up code
use versatileab::rt::entry;

use semihosting::println;

static BAR: &str = "............................................................";
const MAX_LEN: f32 = BAR.len() as f32;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    const STEPS: u16 = 100;
    const RADIANS_PER_STEP: f32 = (4.0 * core::f32::consts::PI) / (STEPS as f32);
    println!("Sine wave test (f32)...");
    for row in 0..STEPS {
        let angle = (row as f32) * RADIANS_PER_STEP;
        let sine = libm::sinf(angle);
        let bar_len = ((sine + 1.0) * (MAX_LEN / 2.0)) as usize;
        println!("({:7.04}) {:.*}o", sine, bar_len, BAR);
    }

    println!("Sine wave test (f64)...");
    for row in 0..STEPS {
        let angle = (row as f64) * f64::from(RADIANS_PER_STEP);
        let sine = libm::sin(angle);
        let bar_len = ((sine + 1.0) * (f64::from(MAX_LEN) / 2.0)) as usize;
        println!("({:7.04}) {:.*}o", sine, bar_len, BAR);
    }
    semihosting::process::exit(0);
}
