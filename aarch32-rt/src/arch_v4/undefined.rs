//! Undefined handler for Armv4 to Armv6

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _undefined_handler(addr: usize) -> usize;`
    // or
    // `extern "C" fn _undefined_handler(addr: usize) -> !;`
    .section .text._asm_default_undefined_handler
    .global _asm_default_undefined_handler
    .type _asm_default_undefined_handler, %function
    _asm_default_undefined_handler:
        // state save from compiled code
        stmfd   sp!, {{ r0 }}
        mrs     r0, spsr
        stmfd   sp!, {{ r0 }}
        // First adjust LR for two purposes: Passing the faulting instruction to the C handler,
        // and to return to the failing instruction after the C handler returns.
        // Load processor status for the calling code
        mrs     r0, spsr
        // Was the code that triggered the exception in Thumb state?
        tst     r0, {t_bit}
        // Subtract 2 in Thumb Mode, 4 in Arm Mode - see p.1206 of the ARMv7-A architecture manual.
        ite     eq
        subeq   lr, lr, #4
        subne   lr, lr, #2
        // now do our standard exception save (which saves the 'wrong' R0)
    "#,
    crate::save_context!(),
    r#"
        // Pass the faulting instruction address to the handler.
        mov     r0, lr
        // call C handler
        bl      _undefined_handler
        // if we get back here, assume they returned a new LR in r0
        mov     lr, r0
        // do our standard restore (with the 'wrong' R0)
    "#,
    crate::restore_context!(),
    r#"
        // Return from the asm handler
        ldmia   sp!, {{ r0 }}
        msr     spsr, r0
        ldmia   sp!, {{ r0 }}
        movs    pc, lr
    .size _asm_default_undefined_handler, . - _asm_default_undefined_handler
    "#,
    t_bit = const { crate::Cpsr::new_with_raw_value(0).with_t(true).raw_value() },
);
