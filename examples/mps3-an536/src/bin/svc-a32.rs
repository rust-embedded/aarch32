//! SVC (software interrupt) example

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r_rt::{entry, exception};

use semihosting::println;
use mps3_an536 as _;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    let x = 1;
    let y = x + 1;
    let z = (y as f64) * 1.5;
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    cortex_ar::svc!(0xABCDEF);
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    panic!("I am an example panic");
}

/// This is our SVC exception handler
#[exception(SupervisorCall)]
fn svc_handler(arg: u32) {
    println!("In svc_handler, with arg=0x{:06x}", arg);
    if arg == 0xABCDEF {
        // test nested SVC calls
        cortex_ar::svc!(0x456789);
    }
}
