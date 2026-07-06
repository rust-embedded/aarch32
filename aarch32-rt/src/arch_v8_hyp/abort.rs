//! Data and Prefetch Abort handlers for Armv8-R at EL2

core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

    .section .text._asm_default_data_abort_handler

    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _data_abort_handler(addr: usize);`
    .global _asm_default_data_abort_handler
    .type _asm_default_data_abort_handler, %function
    _asm_default_data_abort_handler:
        push    {{ r0-r3, r12, lr }}      // Push preserved registers (1)
        mrs     r0, spsr_hyp              // Grab SPSR (2)
        and     r12, sp, 7                // Align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here (3)
        push    {{ r0, r12 }}             // Push SPSR and alignment amount (4)
    "#,
    crate::fpu_context!("save"),
    r#"
        mrs     r0, elr_hyp               // Pass the faulting instruction address to the handler.
        bl      _data_abort_handler       // Call C handler
        msr     elr_hyp, r0               // If we get back here, assume they returned a new LR in r0
    "#,
    crate::fpu_context!("restore"),
    r#"
        pop     {{ r0, r12 }}             // Pop SPSR and alignment amount to undo (4)
        add     sp, r12                   // Restore SP alignment using R12 to undo (3)
        msr     spsr_hyp, r0              // Restore SPSR to undo (2)
        pop     {{ r0-r3, r12, lr }}      // Pop state that C function didn't save to undo (1)
        eret                              // Return from the asm handler
    .size _asm_default_data_abort_handler, . - _asm_default_data_abort_handler
    "#,
);

core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

    .section .text._asm_default_prefetch_abort_handler

    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _prefetch_abort_handler(addr: usize);`
    .global _asm_default_prefetch_abort_handler
    .type _asm_default_prefetch_abort_handler, %function
    _asm_default_prefetch_abort_handler:
        push    {{ r0-r3, r12, lr }}      // Push preserved registers (1)
        mrs     r0, spsr_hyp              // Grab SPSR (2)
        and     r12, sp, 7                // Align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here (3)
        push    {{ r0, r12 }}             // Push SPSR and alignment amount (4)
    "#,
    crate::fpu_context!("save"),
    r#"
        mrs     r0, elr_hyp               // Pass the faulting instruction address to the handler.
        bl      _prefetch_abort_handler   // Call C handler
        msr     elr_hyp, r0               // If we get back here, assume they returned a new LR in r0
    "#,
    crate::fpu_context!("restore"),
    r#"
        pop     {{ r0, r12 }}             // Pop SPSR and alignment amount to undo (4)
        add     sp, r12                   // Restore SP alignment using R12 to undo (3)
        msr     spsr_hyp, r0              // Restore SPSR to undo (2)
        pop     {{ r0-r3, r12, lr }}      // Pop state that C function didn't save to undo (1)
        eret                              // Return from the asm handler
    .size _asm_default_prefetch_abort_handler, . - _asm_default_prefetch_abort_handler
    "#,
);
