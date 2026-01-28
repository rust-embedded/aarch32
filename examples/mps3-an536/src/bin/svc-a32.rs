//! SVC (software interrupt) example

#![no_std]
#![no_main]

use aarch32_rt::{entry, exception};
use mps3_an536 as _;
use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    let x = 1;
    let y = x + 1;
    let z = (y as f64) * 1.5;
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    do_svc1();
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    semihosting::process::exit(0);
}

/// This is our SVC exception handler
#[exception(SupervisorCall)]
fn svc_handler(arg: u32) {
    println!("In svc_handler, with arg=0x{:06x}", arg);
    if arg == 0xABCDEF {
        // test nested SVC calls
        do_svc2();
    }
}

#[instruction_set(arm::a32)]
fn do_svc1() {
    aarch32_cpu::svc!(0xABCDEF);
}

#[instruction_set(arm::a32)]
fn do_svc2() {
    aarch32_cpu::svc!(0x456789);
}
