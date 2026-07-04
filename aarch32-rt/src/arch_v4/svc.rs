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
        stmfd   sp!, {{ r12, lr }}        // Save LR and R12 (1)
        mrs     r12, spsr                 // Grab SPSR (2)
        push    {{ r12 }}                 // Push SPSR value (3)
        and     r12, sp, 7                // Align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here (4)
        push    {{ r0-r6, r12 }}          // Push SVC frame registers and alignment amount (5)
        mov     r12, sp                   // Save SP for integer frame
    "#,
    crate::fpu_context!("save"),
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
        mov     r1, r12                   // Pass the stacked integer registers in r1
        bl      _svc_handler              // Call C handler in SVC mode
        mov     lr, r0                    // Move r0 out of the way - restore_fpu_context will trash it
    "#,
    crate::fpu_context!("restore"),
    r#"
        pop     {{ r0-r6, r12 }}          // Pop SVC frame registers and alignment amount to undo (5)
        mov     r0, lr                    // Replace R0 with return value from _svc_handler
        add     sp, r12                   // Restore SP alignment using R12 to undo (4)
        pop     {{ r12 }}                 // Grab saved SPSR to undo (3)
        msr     spsr, r12                 // Restore SPSR using R12 to undo (2)
        ldmfd   sp!, {{ r12, pc }}^       // Restore R12 and return from exception (^ => restore SPSR to CPSR) to undo (1)
    .size _asm_default_svc_handler, . - _asm_default_svc_handler
    .popsection
    "#,
    t_bit = const { crate::Cpsr::new_with_raw_value(0).with_t(true).raw_value() },
);
