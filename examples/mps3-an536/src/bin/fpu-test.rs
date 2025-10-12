//! Simple FPU test

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r_rt::entry;

// pull in our library
use mps3_an536 as _;

use semihosting::println;

static BAR: &str = "............................................................";
const MAX_LEN: f32 = BAR.len() as f32;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-r-rt`.
#[entry]
fn main() -> ! {
    const STEPS: u32 = 100;
    const RADIANS_PER_STEP: f32 = (2.0 * core::f32::consts::PI) / 50.0;
    println!("Sine wave test (f32)...");
    for row in 0u32..100u32 {
        let angle = (row as f32) * RADIANS_PER_STEP;
        let sine = libm::sinf(angle);
        let bar_len = ((sine + 1.0) * (MAX_LEN / 2.0)) as usize;
        println!("({:7.04}) {:.*}o", sine, bar_len, BAR);
    }

    println!("Sine wave test (f64)...");
    for row in 0u32..100u32 {
        let angle = (row as f64) * f64::from(RADIANS_PER_STEP);
        let sine = libm::sin(angle);
        let bar_len = ((sine + 1.0) * (f64::from(MAX_LEN) / 2.0)) as usize;
        println!("({:7.04}) {:.*}o", sine, bar_len, BAR);
    }
    semihosting::process::exit(0);
}
