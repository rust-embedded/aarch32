//! Data and Prefetch Abort handlers for Armv4 to Armv6

core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    .section .text._asm_default_data_abort_handler

    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _data_abort_handler(addr: usize);`
    .global _asm_default_data_abort_handler
    .type _asm_default_data_abort_handler, %function
    _asm_default_data_abort_handler:
        // Subtract 8 from the stored LR, see p.1214 of the ARMv7-A architecture manual.
        subs    lr, lr, #8
        // state save from compiled code
        stmfd   sp!, {{ r0 }}
        mrs     r0, spsr
        stmfd   sp!, {{ r0 }}
    "#,
    crate::save_context!(),
    r#"
        // Pass the faulting instruction address to the handler.
        mov     r0, lr
        // call C handler
        bl      _data_abort_handler
        // if we get back here, assume they returned a new LR in r0
        mov     lr, r0
    "#,
    crate::restore_context!(),
    r#"
        // Return from the asm handler
        ldmia   sp!, {{ r0 }}
        msr     spsr, r0
        ldmia   sp!, {{ r0 }}
        movs    pc, lr
    .size _asm_default_data_abort_handler, . - _asm_default_data_abort_handler

    .section .text._asm_default_prefetch_abort_handler

    // Called from the vector table when we have a prefetch abort.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _prefetch_abort_handler(addr: usize);`
    .global _asm_default_prefetch_abort_handler
    .type _asm_default_prefetch_abort_handler, %function
    _asm_default_prefetch_abort_handler:
        // Subtract 4 from the stored LR, see p.1212 of the ARMv7-A architecture manual.
        subs    lr, lr, #4
        // state save from compiled code
        stmfd   sp!, {{ r0 }}
        mrs     r0, spsr
        stmfd   sp!, {{ r0 }}
    "#,
    crate::save_context!(),
    r#"
        // Pass the faulting instruction address to the handler.
        mov     r0, lr
        // call C handler
        bl      _prefetch_abort_handler
        // if we get back here, assume they returned a new LR in r0
        mov     lr, r0
    "#,
    crate::restore_context!(),
    r#"
        // Return from the asm handler
        ldmia   sp!, {{ r0 }}
        msr     spsr, r0
        ldmia   sp!, {{ r0 }}
        movs    pc, lr
    .size _asm_default_prefetch_abort_handler, . - _asm_default_prefetch_abort_handler
    "#,
);
