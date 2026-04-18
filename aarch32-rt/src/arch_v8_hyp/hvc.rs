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
        push    {{ r12, lr }}             // give us R12 and LR to work with
        mrs     lr, elr_hyp               // grab elr
        mrs     r12, spsr_hyp             // grab spsr
        push    {{ r12, lr }}             // push them to stack
        mov     r12, sp                   // align SP down to eight byte boundary using R12
        and     r12, r12, 7               //
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here
        push    {{ r0-r6, r12 }}          // push frame and alignment amount to stack
        mov     r12, sp                   // r12 = pointer to Frame
    "#,
    crate::save_fpu_context!(),
    r#"
        mrc     p15, 4, r0, c5, c2, 0     // r0 = HSR value
        mov     r1, r12                   // r1 = frame pointer
        bl      _hvc_handler
        mov     lr, r0                    // copy return value into LR, because we're about to use r0 in the FPU restore
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r6, r12 }}          // restore frame and alignment
        mov     r0, lr                    // copy return value from lr back to r0, overwriting saved r0
        add     sp, r12                   // restore SP alignment using R12
        pop     {{ r12, lr }}             // pop elr and spsr from stack
        msr     elr_hyp, lr               // restore elr
        msr     spsr_hyp, r12             // restore spsr
        pop     {{ r12, lr }}             // pop R12 and LR from stack
        eret                              // Return from the asm handler
    .size _asm_default_hvc_handler, . - _asm_default_hvc_handler
    "#,
);
