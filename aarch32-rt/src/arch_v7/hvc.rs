//! HVC handler for Armv7 and higher

#[cfg(target_arch = "arm")]
#[cfg(arm_architecture = "v8-r")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

    .pushsection .text._asm_default_hvc_handler

    // Called from the vector table when we have an hypervisor call.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _hvc_handler(hsr: u32, frame: &Frame) -> u32;`
    .global _asm_default_hvc_handler
    .type _asm_default_hvc_handler, %function
    _asm_default_hvc_handler:
        push    {{ r12, lr }}             // Push preserved registers R12 and LR (1)
        push    {{ r0-r5 }}               // Push HVC frame to stack (2)
        mov     r12, sp                   // R12 = pointer to Frame
    "#,
    crate::fpu_context!("save"),
    r#"
        mrc     p15, 4, r0, c5, c2, 0     // R0 = HSR value
        mov     r1, r12                   // R1 = frame pointer
        bl      _hvc_handler
        mov     r12, r0
    "#,
    crate::fpu_context!("restore"),
    r#"
        pop     {{ r0-r5 }}               // Pop HVC frame (2)
        mov     r0, r12                   // Replace return value
        pop     {{ r12, lr }}             // Pop R12 and LR from stack to undo (1)
        eret                              // Return from the asm handler
    .size _asm_default_hvc_handler, . - _asm_default_hvc_handler
    .popsection
    "#,
);

#[cfg(target_arch = "arm")]
#[cfg(not(arm_architecture = "v8-r"))]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2


    // Never called but makes the linker happy
    .pushsection .text._asm_default_hvc_handler
    .arm
    .global _asm_default_hvc_handler
    .type _asm_default_hvc_handler, %function
    _asm_default_hvc_handler:
        b       .
    .size _asm_default_hvc_handler, . - _asm_default_hvc_handler
    .popsection
    "#,
);
