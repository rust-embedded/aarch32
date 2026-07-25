//! Start-up code for Armv8-R to stay in EL2.

// # _default_start
//
// We boot into EL2, set up a HYP stack pointer, and run `kmain` in EL2 on the
// primary core, and `kmain_secondary` in EL2 on any secondary cores.
//
// This function must produce A32 machine code, because it's called by the Vector Table
// with a raw PC load and the Vector Table is always in A32 machine code.
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3
    .pushsection .text.default_start
    .arm
    .global _default_start
    .type _default_start, %function
    _default_start:
        // Read MPIDR into R0
        mrc     p15, 0, r0, c0, c0, 5
        // Check if core ID (bottom 8 bits) is zero
        ands    r0, r0, 0xFF
        bne     1f
        // Primary core (core 0) can do normal start-up
        mov     r4, r0
        bl      _asm_init_segments
        mov     r0, r4
        b       2f
    1:
        // Secondary core needs to spin until some magic flag is set
        mov     r4, r0
        bl      _asm_secondary_core_park
        mov     r0, r4
    2:
        // Set stack pointer
        ldr	    sp, =_hyp_stack_high_end
        ldr	    r1, =_hyp_stack_size
        muls    r1, r1, r0
        subs    sp, sp, r1
        // Set the HVBAR (for EL2) to _vector_table
        ldr     r1, =_vector_table
        mcr     p15, 4, r1, c12, c0, 0
        // Mask IRQ and FIQ
        mrs     r1, CPSR
        orr     r1, {irq_fiq}
        msr     CPSR, r1
        // Clear Thumb Exception bit
        mrc     p15, 0, r1, c1, c0, 0
        bic     r1, #0x40000000
        mcr     p15, 0, r1, c1, c0, 0
        // Allow VFP coprocessor access
        mrc     p15, 0, r1, c1, c0, 2
        orr     r1, r1, #0xF00000
        mcr     p15, 0, r1, c1, c0, 2
        // Enable VFP
        mov     r1, #0x40000000
        vmsr    fpexc, r1
        // Zero all registers before calling kmain
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
        cmp     r0, 0
        bne     3f
        // Jump to application with primary core
        bl      kmain
        // In case the application returns, loop forever
        b       .
    3:
        // Jump to application with secondary core
        bl      kmain_secondary
        // In case the application returns, loop forever
        b       .
    .size _default_start, . - _default_start
    .popsection
    "#,
    irq_fiq = const aarch32_cpu::register::Cpsr::new_with_raw_value(0).with_i(true).with_f(true).raw_value()
);

#[unsafe(naked)]
#[unsafe(no_mangle)]
extern "C" fn _asm_default_secondary_core_park() {
    core::arch::naked_asm!(
        // just spin
        "b       ."
    )
}
