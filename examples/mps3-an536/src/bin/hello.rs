//! Semihosting hello-world for Arm Cortex-R

#![no_std]
#![no_main]

use aarch32_rt::entry;
use mps3_an536 as _;
use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-r-rt`.
#[entry]
fn main() -> ! {
    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    panic!("I am an example panic");
}
