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
    .pushsection .text._asm_default_undefined_handler
    .arm
    .global _asm_default_undefined_handler
    .type _asm_default_undefined_handler, %function
    _asm_default_undefined_handler:
        push    {{ r12 }}                 // save R12 - can now use it
        mrs     r12, spsr                 // grab SPSR using R12
        push    {{ r12 }}                 // save SPSR value
        tst     r12, {t_bit}              // Was the code that triggered the exception in Thumb state?
        ite     eq                        // Adjust LR to point to faulting instruction - see p.1206 of the ARMv7-A architecture manual.
        subeq   lr, lr, #4                // Subtract 4 in Arm Mode
        subne   lr, lr, #2                // Subtract 2 in Thumb Mode
        and     r12, sp, 7                // align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here
        push    {{ r0-r4, r12 }}          // push alignment amount, and preserved registers - can now use R0-R3 (R4 is just padding)
    "#,
    crate::save_fpu_context!(),
    r#"
        mov     r0, lr                    // Pass the faulting instruction address to the handler.
        bl      _undefined_handler        // call C handler
        mov     lr, r0                    // if we get back here, assume they returned a new LR in r0
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r4, r12 }}          // restore preserved registers, dummy value, and alignment amount
        add     sp, r12                   // restore SP alignment using R12
        pop     {{ r12 }}                 // restore SPSR using R12
        msr     spsr, r12                 //
        pop     {{ r12 }}                 // restore R12
        movs    pc, lr                    // return from exception (movs => restore SPSR to CPSR)
    .size _asm_default_undefined_handler, . - _asm_default_undefined_handler
    .popsection
    "#,
    t_bit = const { crate::Cpsr::new_with_raw_value(0).with_t(true).raw_value() },
);
