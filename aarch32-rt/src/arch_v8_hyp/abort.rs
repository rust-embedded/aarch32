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
        push    {{ r0-r3, r12, lr }}      // preserve state that C function won't save
        mrs     r0, elr_hyp               // grab ELR_hyp
        mrs     r1, spsr_hyp              // grab SPSR_hyp
        mov     r12, sp                   // align SP down to eight byte boundary using R12
        and     r12, r12, 7               //
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here
        push    {{ r0-r2, r12 }}          // save ELR, SPSR, padding and alignment amount
    "#,
    crate::save_fpu_context!(),
    r#"
        mrs     r0, elr_hyp               // Pass the faulting instruction address to the handler.
        bl      _data_abort_handler       // call C handler
        msr     elr_hyp, r0               // if we get back here, assume they returned a new LR in r0
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r2, r12 }}          // restore ELR, SPSR, padding and alignment amount
        add     sp, r12                   // restore SP alignment
        msr     spsr_hyp, r1              // restore SPSR
        pop     {{ r0-r3, r12, lr }}      // restore state that C function didn't save
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
        push    {{ r0-r3, r12, lr }}      // preserve state that C function won't save
        mrs     r0, elr_hyp               // grab ELR_hyp
        mrs     r1, spsr_hyp              // grab SPSR_hyp
        mov     r12, sp                   // align SP down to eight byte boundary using R12
        and     r12, r12, 7               //
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here
        push    {{ r0-r2, r12 }}          // save ELR, SPSR, padding and alignment amount
    "#,
    crate::save_fpu_context!(),
    r#"
        mrs     r0, elr_hyp               // Pass the faulting instruction address to the handler.
        bl      _prefetch_abort_handler   // call C handler
        msr     elr_hyp, r0               // if we get back here, assume they returned a new LR in r0
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r2, r12 }}          // restore ELR, SPSR, padding and alignment amount
        add     sp, r12                   // restore SP alignment
        msr     spsr_hyp, r1              // restore SPSR
        pop     {{ r0-r3, r12, lr }}      // restore state that C function didn't save
        eret                              // Return from the asm handler
    .size _asm_default_prefetch_abort_handler, . - _asm_default_prefetch_abort_handler
    "#,
);
