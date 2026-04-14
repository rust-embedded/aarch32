//! SVC handler for Armv4 to Armv6

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    // Called from the vector table when we have an software interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _svc_handler(arg: u32, frame: &Frame) -> u32;`
    .pushsection .text._asm_default_svc_handler
    .arm
    .global _asm_default_svc_handler
    .type _asm_default_svc_handler, %function
    _asm_default_svc_handler:
        push    {{ r12, lr }}             // save LR and R12 - can now use R12 (but leave LR alone for SVC code lookup)
        mrs     r12, spsr                 // grab SPSR using R12
        push    {{ r12 }}                 // save SPSR value
        and     r12, sp, 7                // align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here
        push    {{ r0-r6, r12 }}          // push alignment amount, and stacked SVC argument registers (must be even number of regs for alignment)
        mov     r12, sp                   // save SP for integer frame
    "#,
    crate::save_fpu_context!(),
    r#"
        mrs     r0, spsr                  // Load processor status that was banked on entry
        tst     r0, {t_bit}               // SVC occurred from Thumb state?
        beq     1f
        ldrb    r0, [lr,#-2]              // Yes: Load 1-byte immediate
        b       2f
    1:
        ldr     r0, [lr,#-4]              // No: Load word and...
        bic     r0, r0, #0xFF000000       // ...extract 3-byte immediate
    2:
        mov     r1, r12                   // pass the stacked integer registers in r1
        bl      _svc_handler
        mov     lr, r0                    // move r0 out of the way - restore_fpu_context will trash it
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r6, r12 }}          // restore stacked registers and alignment amount
        mov     r0, lr                    // replace R0 with return value from _svc_handler
        add     sp, r12                   // restore SP alignment using R12
        pop     {{ lr }}                  // restore SPSR using LR
        msr     spsr, lr                  //
        ldmfd   sp!, {{ r12, pc }}^       // restore R12 and return from exception (^ => restore SPSR to CPSR)
    .size _asm_default_svc_handler, . - _asm_default_svc_handler
    .popsection
    "#,
    t_bit = const { crate::Cpsr::new_with_raw_value(0).with_t(true).raw_value() },
);
