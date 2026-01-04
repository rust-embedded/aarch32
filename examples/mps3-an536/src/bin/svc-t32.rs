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
    unsafe {
        svc12_from_t32();
    }
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    semihosting::process::exit(0);
}

/// This is our SVC exception handler
#[exception(SupervisorCall)]
fn svc_handler(arg: u32) {
    println!("In svc_handler, with arg=0x{:06x}", arg);
    if arg == 0x12 {
        // test nested SVC calls
        unsafe {
            svc34_from_t32();
        }
    }
}

// These functions are written in assembly
extern "C" {
    fn svc12_from_t32();
    fn svc34_from_t32();
}

core::arch::global_asm!(
    r#"
    // fn svc12_from_t32();
    .thumb
    .global svc12_from_t32
    .type svc12_from_t32, %function
    svc12_from_t32:
        push    {{ r7, lr }}
        svc     0x12
        pop     {{ r7, pc }}
    .size svc12_from_t32, . - svc12_from_t32

    // fn svc34_from_t32();
    .thumb
    .global svc34_from_t32
    .type svc34_from_t32, %function
    svc34_from_t32:
        push    {{ r7, lr }}
        svc     0x34
        pop     {{ r7, pc }}
    .size svc34_from_t32, . - svc34_from_t32
"#
);
