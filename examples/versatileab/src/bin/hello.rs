//! Semihosting hello-world.

#![no_std]
#![no_main]

use aarch32_rt::entry;
use semihosting::println;
use versatileab as _;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn my_main() -> ! {
    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    versatileab::want_panic();
    panic!("I am an example panic");
}
