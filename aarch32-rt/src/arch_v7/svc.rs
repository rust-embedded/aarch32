//! SVC handler for Armv7 and higher

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

    .section .text._asm_default_svc_handler

    // Called from the vector table when we have an software interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _svc_handler(svc: u32);`
    .global _asm_default_svc_handler
    .type _asm_default_svc_handler, %function
    _asm_default_svc_handler:
        // state save from compiled code
        srsfd   sp!, #{svc_mode}
    "#,
    crate::save_context!(),
    r#"
        mrs     r0, spsr                 // Load processor status that was banked on entry
        tst     r0, {t_bit}              // SVC occurred from Thumb state?
        beq     1f
        ldrh    r0, [lr,#-2]             // Yes: Load halfword and...
        bic     r0, r0, #0xFF00          // ...extract comment field
        b       2f
    1:
        ldr     r0, [lr,#-4]             // No: Load word and...
        bic     r0, r0, #0xFF000000      // ...extract comment field
    2:
        // r0 now contains SVC number
        bl      _svc_handler
    "#,
    crate::restore_context!(),
    r#"
        // Return from the asm handler
        rfefd   sp!
    .size _asm_default_svc_handler, . - _asm_default_svc_handler
    "#,
    svc_mode = const crate::ProcessorMode::Svc as u8,
    t_bit = const {
        crate::Cpsr::new_with_raw_value(0)
            .with_t(true)
            .raw_value()
    },
);
