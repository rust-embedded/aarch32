//! Semihosting hello-world.

#![no_std]
#![no_main]

// pull in our start-up code
use versatileab as _;

use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in the runtime crate.
#[no_mangle]
pub extern "C" fn boot_core(cpu_id: u32) {
    match cpu_id {
        0 => {
            main();
        }
        _ => panic!("unexpected CPU ID {}", cpu_id),
    }
}

/// The main function of our Rust application.
#[export_name = "main"]
fn main() -> ! {
    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    panic!("I am an example panic");
}
