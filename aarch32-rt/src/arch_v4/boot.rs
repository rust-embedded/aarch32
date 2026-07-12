//! Start-up code for ARMv4 - ARMv6 CPUs that always boot into EL1

// # _default_start
//
// Reset function for ARMv4 to ARMv6
//
// Initialises global memory and sets up stacks. Only supports one CPU core.
//
// This function must produce A32 machine code, because it's called by the Vector Table
// with a raw PC load and the Vector Table is always in A32 machine code.
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2
    .pushsection .text.default_start
    .arm
    .global _default_start
    .type _default_start, %function
    _default_start:
        // Init .data and .bss on primary core
        bl      _asm_init_segments
        // Do standard core init - only one core supported
        mov     r0, #0
        b       _asm_core_start
    .size _default_start, . - _default_start
    .popsection
    "#
);
