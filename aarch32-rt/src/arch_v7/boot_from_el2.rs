//! Start-up code for CPUs that *might* boot into EL2 but that we want in EL1.

use aarch32_cpu::register::{cpsr::ProcessorMode, Cpsr, Hactlr};

core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2
    .cpu cortex-r52

    .pushsection .text.default_start
    .arm
    .global _default_start
    .type _default_start, %function
    _default_start:
        // Are we in EL2? If not, skip the EL2 setup portion
        mrs     r0, cpsr
        and     r0, r0, 0x1F
        cmp     r0, {cpsr_mode_hyp}
        bne     1f
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
        // Program the SPSR - enter system mode (0x1F) in Arm mode with IRQ, FIQ masked
        mov		r1, {sys_mode}
        msr		spsr_hyp, r1
        adr		r1, 1f
        msr		elr_hyp, r1
        dsb
        isb
        eret
    1:
        // Set the VBAR (for EL1) to _vector_table. NB: This isn't required on
        // Armv7-R because that only supports 'low' (default) or 'high'.
        ldr     r0, =_vector_table
        mcr     p15, 0, r0, c12, c0, 0
        // Init .data and .bss
        bl      _init_segments
        // Set up stacks.
        mov     r0, #0
        bl      _stack_setup_preallocated
        // Clear Thumb Exception bit
        mrc     p15, 0, r0, c1, c0, 0
        bic     r0, #0x40000000
        mcr     p15, 0, r0, c1, c0, 0
    "#,
    #[cfg(any(target_abi = "eabihf", feature = "eabi-fpu"))]
    r#"
        // Allow VFP coprocessor access
        mrc     p15, 0, r0, c1, c0, 2
        orr     r0, r0, #0xF00000
        mcr     p15, 0, r0, c1, c0, 2
        // Enable VFP
        mov     r0, #0x40000000
        vmsr    fpexc, r0
    "#,
    r#"
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
    .popsection
    "#,
    cpsr_mode_hyp = const ProcessorMode::Hyp as u8,
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
    sys_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Sys)
            .with_i(true)
            .with_f(true)
            .raw_value()
    }
);
