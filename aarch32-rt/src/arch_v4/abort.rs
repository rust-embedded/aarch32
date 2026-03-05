//! Data and Prefetch Abort handlers for Armv4 to Armv6

core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _data_abort_handler(addr: usize);`
    .section .text._asm_default_data_abort_handler
    .arm
    .global _asm_default_data_abort_handler
    .type _asm_default_data_abort_handler, %function
    _asm_default_data_abort_handler:
        sub     lr, lr, #8                // Subtract 8 from LR, see p.1214 of the ARMv7-A architecture manual.
        push    {{ r12 }}                 // Save preserved register R12 - can now use it
        mrs     r12, spsr                 // grab SPSR
        push    {{ r12 }}                 // save SPSR value
        and     r12, sp, 7                // align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here
        push    {{ r0-r4, r12 }}          // push alignment amount, and preserved registers - can now use R0-R3 (R4 is just padding)
    "#,
    crate::save_fpu_context!(),
    r#"
        mov     r0, lr                    // Pass the faulting instruction address to the handler.
        bl      _data_abort_handler       // call C handler
        mov     lr, r0                    // if we get back here, assume they returned a new LR in r0
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r4, r12 }}          // restore preserved registers, dummy value, and alignment amount
        add     sp, r12                   // restore SP alignment using R12
        pop     {{ r12 }}                 // restore SPSR using R12
        msr     spsr, r12                 //
        pop     {{ r12 }}                 // restore R12
        movs    pc, lr                    // return from exception
    .size _asm_default_data_abort_handler, . - _asm_default_data_abort_handler
    "#
);

core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    // Called from the vector table when we have a prefetch abort.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _prefetch_abort_handler(addr: usize);`
    .section .text._asm_default_prefetch_abort_handler
    .arm
    .global _asm_default_prefetch_abort_handler
    .type _asm_default_prefetch_abort_handler, %function
    _asm_default_prefetch_abort_handler:
        sub     lr, lr, #4                // Subtract 4 from LR, see p.1212 of the ARMv7-A architecture manual.
        push    {{ r12 }}                 // Save preserved register R12 - can now use it
        mrs     r12, spsr                 // grab SPSR
        push    {{ r12 }}                 // save SPSR value
        and     r12, sp, 7                // align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here
        push    {{ r0-r4, r12 }}          // push alignment amount, and preserved registers - can now use R0-R3 (R4 is just padding)
    "#,
    crate::save_fpu_context!(),
    r#"
        mov     r0, lr                    // Pass the faulting instruction address to the handler.
        bl      _prefetch_abort_handler   // call C handler
        mov     lr, r0                    // if we get back here, assume they returned a new LR in r0
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r4, r12 }}          // restore preserved registers, dummy value, and alignment amount
        add     sp, r12                   // restore SP alignment using R12
        pop     {{ r12 }}                 // restore SPSR using R12
        msr     spsr, r12                 //
        pop     {{ r12 }}                 // restore R12
        movs    pc, lr                    // return from exception
    .size _asm_default_prefetch_abort_handler, . - _asm_default_prefetch_abort_handler
    "#,
);
