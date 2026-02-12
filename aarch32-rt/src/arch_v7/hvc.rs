//! HVC handler for Armv7 and higher

#[cfg(target_arch = "arm")]
#[cfg(arm_architecture = "v8-r")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

    .section .text._asm_default_hvc_handler

    // Called from the vector table when we have an hypervisor call.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _hvc_handler(hsr: u32, frame: &Frame) -> u32;`
    .global _asm_default_hvc_handler
    .type _asm_default_hvc_handler, %function
    _asm_default_hvc_handler:
        push    {{ r12, lr }}             // push state to stack
        push    {{ r0-r5 }}               // push frame to stack
        mov     r12, sp                   // r12 = pointer to Frame
    "#,
    crate::save_fpu_context!(),
    r#"
        mrc     p15, 4, r0, c5, c2, 0     // r0 = HSR value
        mov     r1, r12                   // r1 = frame pointer
        bl      _hvc_handler
        mov     r12, r0
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r5 }}               // restore frame
        mov     r0, r12                   // replace return value
        pop     {{ r12, lr }}             // pop state from stack
        eret                              // Return from the asm handler
    .size _asm_default_hvc_handler, . - _asm_default_hvc_handler
    "#,
);

#[cfg(target_arch = "arm")]
#[cfg(not(arm_architecture = "v8-r"))]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2


    // Never called but makes the linker happy
    .section .text._asm_default_hvc_handler
    .arm
    .global _asm_default_hvc_handler
    .type _asm_default_hvc_handler, %function
    _asm_default_hvc_handler:
        b       .
    .size _asm_default_hvc_handler, . - _asm_default_hvc_handler
    "#,
);
