//! SVC (software interrupt) example

#![no_std]
#![no_main]

use aarch32_rt::{entry, exception};
use semihosting::println;
use versatileab as _;

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

    panic!("I am an example panic");
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

#[cfg_attr(
    any(arm_architecture = "v4t", arm_architecture = "v5te"),
    instruction_set(arm::a32)
)]
fn do_svc1() {
    aarch32_cpu::svc!(0xABCDEF);
}

#[cfg_attr(
    any(arm_architecture = "v4t", arm_architecture = "v5te"),
    instruction_set(arm::a32)
)]
fn do_svc2() {
    aarch32_cpu::svc!(0x456789);
}
