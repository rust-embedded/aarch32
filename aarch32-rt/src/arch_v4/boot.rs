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
        // Init .data and .bss
        bl      _init_segments
        // Set up stacks.
        mov     r0, #0
        bl      _stack_setup_preallocated
    "#,
    crate::system_init!(),
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
    "#
);
