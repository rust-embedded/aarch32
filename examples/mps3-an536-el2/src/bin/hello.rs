//! Semihosting hello-world for Arm Cortex-R52 running in EL2 (Hyp Mode)

#![no_std]
#![no_main]

use aarch32_rt::entry;
use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code at the bottom of this file.
#[entry]
fn main() -> ! {
    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    println!("{:?}", aarch32_cpu::register::Sctlr::read());
    println!("{:?}", aarch32_cpu::register::Cpsr::read());

    let mut mpu = unsafe { aarch32_cpu::pmsav8::El2Mpu::new() };
    for idx in 0..mpu.num_regions() {
        if let Some(region) = mpu.get_region(idx) {
            println!("Region {}: {:?}", idx, region);
        }
    }

    mps3_an536_el2::exit(0);
}
