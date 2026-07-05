//! Start-up code for Armv8-R to stay in EL2.
//!
//! We boot into EL2, set up a HYP stack pointer, and run `kmain` in EL2.

core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

    .section .text.default_start
    .global _default_start
    .type _default_start, %function
    _default_start:
        // Init .data and .bss
        bl      _init_segments
        // Set stack pointer
        ldr     sp, =_hyp_stack_high_end
        // Set the HVBAR (for EL2) to _vector_table
        ldr     r1, =_vector_table
        mcr     p15, 4, r1, c12, c0, 0
        // Mask IRQ and FIQ
        mrs     r0, CPSR
        orr     r0, {irq_fiq}
        msr     CPSR, r0
        // Clear Thumb Exception bit
        mrc     p15, 0, r0, c1, c0, 0
        bic     r0, #0x40000000
        mcr     p15, 0, r0, c1, c0, 0
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
    .size _default_start, . - _default_start
    "#,
    irq_fiq = const aarch32_cpu::register::Cpsr::new_with_raw_value(0).with_i(true).with_f(true).raw_value()
);
