//! HVC handler for Armv8-R at EL2

#[cfg(target_arch = "arm")]
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
        push    {{ r12, lr }}             // Push preserved registers R12 and LR (1)
        mrs     lr, elr_hyp               // Grab ELR (2)
        mrs     r12, spsr_hyp             // Grab SPSR (3)
        push    {{ r12, lr }}             // Push them to stack (4)
        and     r12, sp, 7                // Align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here (5)
        push    {{ r0-r6, r12 }}          // Push HVC frame and alignment amount to stack (6)
        mov     r12, sp                   // R12 = pointer to Frame
    "#,
    crate::fpu_context!("save"),
    r#"
        mrc     p15, 4, r0, c5, c2, 0     // R0 = HSR value
        mov     r1, r12                   // R1 = frame pointer
        bl      _hvc_handler
        mov     lr, r0                    // Copy return value into LR, because we're about to use r0 in the FPU restore
    "#,
    crate::fpu_context!("restore"),
    r#"
        pop     {{ r0-r6, r12 }}          // Pop HVC frame and alignment to undo (6)
        mov     r0, lr                    // Copy return value from LR back to R0, overwriting saved R0
        add     sp, r12                   // Restore SP alignment using R12 to undo (5)
        pop     {{ r12, lr }}             // Pop ELR and SPSR values from stack to undo (4)
        msr     spsr_hyp, r12             // Restore SPSR to undo (3)
        msr     elr_hyp, lr               // Restore ELR to undo (2)
        pop     {{ r12, lr }}             // Pop R12 and LR from stack
        eret                              // Return from the asm handler
    .size _asm_default_hvc_handler, . - _asm_default_hvc_handler
    "#,
);
