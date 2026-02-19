//! SVC (supervisor call) at EL2 example

#![no_std]
#![no_main]

use aarch32_rt::{entry, exception};
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
    let value = do_svc1();
    println!("Got {:08x}", value);
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    mps3_an536_el2::exit(0);
}

/// This is our HVC exception handler
#[exception(HypervisorCall)]
fn hvc_handler(hsr: u32, frame: &aarch32_rt::Frame) -> u32 {
    let hsr = aarch32_cpu::register::Hsr::new_with_raw_value(hsr);
    println!(
        "In hvc_handler, with {:08x?}, {:x?}, {:08x?}",
        hsr,
        hsr.get_iss(),
        frame
    );
    if hsr.iss().value() == 0x12 {
        do_svc2();
    }
    return 0x12345678;
}

#[instruction_set(arm::t32)]
fn do_svc1() -> u32 {
    aarch32_cpu::svc6!(
        0x12,
        0x1000_0000,
        0x1000_0001,
        0x1000_0002,
        0x1000_0003,
        0x1000_0004,
        0x1000_0005
    )
}

#[instruction_set(arm::t32)]
fn do_svc2() -> u32 {
    aarch32_cpu::svc6!(
        0x32,
        0x2000_0000,
        0x2000_0001,
        0x2000_0002,
        0x2000_0003,
        0x2000_0004,
        0x2000_0005
    )
}
