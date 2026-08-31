//! Boot code for Armv7-A and Armv8-R

#[cfg(any(
    arm_architecture = "v7-a",
    all(arm_architecture = "v8-r", not(feature = "el2-mode")),
))]
use aarch32_cpu::register::{cpsr::ProcessorMode, Cpsr, Hactlr};

// # _default_start
//
// Reset function for ARMv7-A and ARMv8-R
//
// Cores will first set the HYP stack and then leave EL2 and enter EL1.
// Core 0 will initialise global memory and call `_asm_core_start`. Other cores
// call `_asm_secondary_core_park` and then `_asm_core_start`.
//
// This function must produce A32 machine code, because it's called by the Vector Table
// with a raw PC load and the Vector Table is always in A32 machine code.
#[cfg(any(
    arm_architecture = "v7-a",
    all(arm_architecture = "v8-r", not(feature = "el2-mode")),
))]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2
    .cpu cortex-r52
    .pushsection .text.default_start
    .arm
    .global _default_start
    .type _default_start, %function
    .p2align 2
    _default_start:
        // Read MPIDR into R0
        mrc     p15, 0, r0, c0, c0, 5
        // Core ID in bottom 8 bits
        and     r0, r0, 0xFF
        // Are we in EL2? If not, skip the EL2 setup portion
        mrs     r1, cpsr
        and     r1, r1, 0x1F
        cmp     r1, {cpsr_mode_hyp}
        bne     1f
        // Set up the Hyp stack for this core
        ldr	    sp, =_hyp_stack_high_end
        ldr	    r1, =_hyp_stack_size
        muls    r1, r1, r0
        subs    sp, sp, r1
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
        // Set the VBAR (for EL1) to _vector_table.
        ldr     r1, =_vector_table
        mcr     p15, 0, r1, c12, c0, 0
        // Check if core ID is zero
        cmp     r0, 0
        bne     2f
        // Primary core (core 0) can do normal start-up
        mov     r4, r0
        bl      _asm_init_segments
        mov     r0, r4
        b       _asm_core_start
    2:
        // Secondary core needs to spin until some magic flag is set
        mov     r4, r0
        bl      _asm_secondary_core_park
        mov     r0, r4
        b       _asm_core_start
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

#[unsafe(naked)]
#[unsafe(no_mangle)]
extern "C" fn _asm_default_secondary_core_park() {
    core::arch::naked_asm!(
        // just spin
        "b       ."
    )
}
