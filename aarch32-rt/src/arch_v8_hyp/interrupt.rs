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
        push    {{ r0-r3, r12, lr }}      // Push preserved registers (1)
        mrs     r0, elr_hyp               // Grab ELR (2)
        mrs     r1, spsr_hyp              // Grab SPSR (3)
        and     r12, sp, 7                // Align SP down to eight byte boundary using R12 
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here (4)
        push    {{ r0-r2, r12 }}          // Push ELR, SPSR, padding and alignment amount (5)
    "#,
    crate::fpu_context!("save"),
    r#"
        bl      _irq_handler              // Call C handler (they may choose to re-enable interrupts)
    "#,
    crate::fpu_context!("restore"),
    r#"
        pop     {{ r0-r2, r12 }}          // Pop ELR, SPSR, padding and alignment amount to undo (5)
        add     sp, r12                   // Restore SP alignment to undo (4)
        msr     spsr_hyp, r1              // Restore SPSR to undo (3)
        msr     elr_hyp, r0               // Restore ELR to undo (2)
        pop     {{ r0-r3, r12, lr }}      // Pop preserved registers (1)
        eret                              // Return from the asm handler
    .size _asm_default_irq_handler, . - _asm_default_irq_handler
    "#,
);
