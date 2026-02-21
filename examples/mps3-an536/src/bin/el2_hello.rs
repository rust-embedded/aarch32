//! Semihosting hello-world for Arm Cortex-R52 running in EL2 (Hyp Mode)

#![no_std]
#![no_main]

use aarch32_cpu::register::Hactlr;
use aarch32_rt::entry;
use mps3_an536 as _;
use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code at the bottom of this file.
#[entry]
fn main() -> ! {
    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);

    let mut mpu = unsafe { aarch32_cpu::pmsav8::El2Mpu::new() };
    for idx in 0..mpu.num_regions() {
        if let Some(region) = mpu.get_region(idx) {
            println!("Region {}: {:?}", idx, region);
        }
    }

    mps3_an536::want_panic();
    panic!("I am an example panic");
}

// Provide a custom `_start` function that sets us up in EL2 mode, with a
// stack.
//
// Unlike the default routine, it does not initialise any other stacks, or
// switch to EL1 mode.
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3-d16

    .section .text.start

    .global _start
    .type _start, %function
    _start:
        // Set stack pointer
        ldr     sp, =_hyp_stack_high_end
        // Set the HVBAR (for EL2) to _vector_table
        ldr     r1, =_vector_table
        mcr     p15, 4, r1, c12, c0, 0
        // Configure HACTLR to let us enter EL1
        mrc     p15, 4, r1, c1, c0, 1
        mov     r2, {hactlr_bits}
        orr     r1, r1, r2
        mcr     p15, 4, r1, c1, c0, 1
        // Init .data and .bss
        bl      _init_segments
        // Allow VFP coprocessor access
        mrc     p15, 0, r0, c1, c0, 2
        orr     r0, r0, #0xF00000
        mcr     p15, 0, r0, c1, c0, 2
        // Enable VFP
        mov     r0, #0x40000000
        vmsr    fpexc, r0
        // Zero all registers before calling kmain
        mov     r0, 0
        mov     r1, 0
        mov     r2, 0
        mov     r3, 0
        mov     r4, 0
        mov     r5, 0
        mov     r6, 0
        mov     r7, 0
        mov     r8, 0
        mov     r9, 0
        mov     r10, 0
        mov     r11, 0
        mov     r12, 0
        // Jump to application
        bl      kmain
        // In case the application returns, loop forever
        b       .
    .size _start, . - _start
    "#,
    hactlr_bits = const {
        Hactlr::new_with_raw_value(0)
            .with_cpuactlr(true)
            .with_cdbgdci(true)
            .with_flashifregionr(true)
            .with_periphpregionr(true)
            .with_qosr(true)
            .with_bustimeoutr(true)
            .with_intmonr(true)
            .with_err(true)
            .with_testr1(true)
            .raw_value()
    },
);
