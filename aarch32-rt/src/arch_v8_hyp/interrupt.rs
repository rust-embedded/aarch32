//! IRQ handler for Armv8-R at EL2

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

    .section .text._asm_default_irq_handler

    // Called from the vector table when we have an interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _irq_handler();`
    .global _asm_default_irq_handler
    .type _asm_default_irq_handler, %function
    _asm_default_irq_handler:
        push    {{ r0-r3, r12, lr }}      // preserve state that C function won't save (1)
        mrs     r0, elr_hyp               // grab ELR_hyp (2)
        mrs     r1, spsr_hyp              // grab SPSR_hyp (3)
        mov     r12, sp                   // align SP down to eight byte boundary using R12 
        and     r12, r12, 7               //
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here (4)
        push    {{ r0-r2, r12 }}          // save ELR, SPSR, padding and alignment amount (5)
    "#,
    crate::save_fpu_context!(),
    r#"
        bl      _irq_handler              // call C handler (they may choose to re-enable interrupts)
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r2, r12 }}          // restore ELR, SPSR, padding and alignment amount to undo (5)
        add     sp, r12                   // restore SP alignment to undo (4)
        msr     spsr_hyp, r1              // restore SPSR_hyp to undo (3)
        msr     elr_hyp, r0               // restore ELR_hyp to undo (2)
        pop     {{ r0-r3, r12, lr }}      // restore state that C function didn't save to undo (1)
        eret                              // Return from the asm handler
    .size _asm_default_irq_handler, . - _asm_default_irq_handler
    "#,
);
