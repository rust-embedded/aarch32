//! Semihosting hello-world.

#![no_std]
#![no_main]

// pull in our start-up code
use versatileab::rt::entry;

use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn my_main() -> ! {
    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    panic!("I am an example panic");
}
