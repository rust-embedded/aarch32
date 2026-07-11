//! Start-up code for CPUs that always boot into EL1

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
