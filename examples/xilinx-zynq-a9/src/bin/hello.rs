//! Semihosting hello-world for the Xilinx Zynq-A9.

#![no_std]
#![no_main]

use aarch32_rt::entry;
use semihosting::println;
use xilinx_zynq_a9 as _;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn my_main() -> ! {
    xilinx_zynq_a9::init();
    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    xilinx_zynq_a9::want_panic();
    panic!("I am an example panic");
}
