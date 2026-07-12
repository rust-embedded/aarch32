//! Boot code for Armv7-R

// # _default_start
//
// Reset function for ARMv7-R
//
// Core 0 will initialise global memory and call `_asm_core_start`. Other cores
// call `_asm_secondary_core_park` and then `_asm_core_start`.
//
// This function must produce A32 machine code, because it's called by the Vector Table
// with a raw PC load and the Vector Table is always in A32 machine code.
core::arch::global_asm!(
    r#"
    .pushsection .text._default_start
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
        b       _asm_core_start
    1:
        // Secondary core needs to spin until some magic flag is set
        mov     r4, r0
        bl      _asm_secondary_core_park
        mov     r0, r4
        b       _asm_core_start
    .size _default_start, . - _default_start
    .popsection
    "#
);

#[unsafe(naked)]
#[unsafe(no_mangle)]
extern "C" fn _asm_default_secondary_core_park() {
    core::arch::naked_asm!(
        // just spin
        "b       ."
    )
}
