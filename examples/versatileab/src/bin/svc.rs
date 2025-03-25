//! SVC (software interrupt) example

#![no_std]
#![no_main]

// pull in our start-up code
use versatileab as _;

use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code.
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
    let x = 1;
    let y = x + 1;
    let z = (y as f64) * 1.5;
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    cortex_ar::svc!(0xABCDEF);
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    panic!("I am an example panic");
}

/// This is our SVC exception handler
#[no_mangle]
unsafe extern "C" fn _svc_handler(arg: u32) {
    println!("In _svc_handler, with arg={:#06x}", arg);
    if arg == 0xABCDEF {
        // test nested SVC calls
        cortex_ar::svc!(0x456789);
    }
}
