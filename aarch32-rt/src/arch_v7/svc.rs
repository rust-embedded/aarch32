//! SVC handler for Armv7 and higher

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3
        
    // Called from the vector table when we have an software interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _svc_handler(arg: u32, frame: &Frame) -> u32;`
    .pushsection .text._asm_default_svc_handler
    .arm
    .global _asm_default_svc_handler
    .type _asm_default_svc_handler, %function
    _asm_default_svc_handler:
        srsfd   sp!, #{svc_mode}          // Store return state to the SVC stack (1)
        push    {{ r12, lr }}             // Push preserved registers R12 and LR (2)
        and     r12, sp, 7                // Align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here (3)
        push    {{ r0-r6, r12 }}          // Push SVC frame registers and alignment amount (4)
        mov     r12, sp                   // Save SP for integer frame
    "#,
    crate::fpu_context!("save"),
    r#"
        mrs     r0, spsr                  // Load processor status that was banked on entry
        tst     r0, {t_bit}               // SVC occurred from Thumb state?
        ldrbne  r0, [lr,#-2]              // Yes: Load 1-byte immediate
        ldreq   r0, [lr,#-4]              // No: Load word and...
        biceq   r0, r0, #0xFF000000       // ...extract 3-byte immediate
        mov     r1, r12                   // Pass the stacked integer registers in r1
        bl      _svc_handler              // Call C handler in SVC mode
        mov     lr, r0                    // Move r0 out of the way - restore_fpu_context will trash it
    "#,
    crate::fpu_context!("restore"),
    r#"
        pop     {{ r0-r6, r12 }}          // Pop SVC frame registers and alignment amount to undo (4)
        mov     r0, lr                    // Replace R0 with return value from _svc_handler
        add     sp, r12                   // Restore SP alignment using R12 to undo (3)
        pop     {{ r12, lr }}             // Pop R12 and LR to undo (2)
        rfefd   sp!                       // Return from exception to undo (1)
    .size _asm_default_svc_handler, . - _asm_default_svc_handler
    .popsection
    "#,
    svc_mode = const crate::ProcessorMode::Svc as u8,
    t_bit = const { crate::Cpsr::new_with_raw_value(0).with_t(true).raw_value() },
);
