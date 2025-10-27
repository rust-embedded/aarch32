//! IRQ handler for Armv7 and higher

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    .section .text._asm_default_irq_handler

    // Called from the vector table when we have an interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _irq_handler();`
    .global _asm_default_irq_handler
    .type _asm_default_irq_handler, %function
    _asm_default_irq_handler:
        // make sure we jump back to the right place
        sub     lr, lr, 4
        // The hardware has copied CPSR to SPSR_irq and LR to LR_irq for us.
        // Now push SPSR_irq and LR_irq to the SYS stack (because that's the
        // mode we're in when we pop)
        srsfd   sp!, #{sys_mode}
        // switch to system mode so we can handle another interrupt
        // (because if we interrupt irq mode we trash our own shadow registers)
        cps     #{sys_mode}
        // we also need to save LR, so we can be re-entrant
        push    {{lr}}
        // save state to the system stack (adjusting SP for alignment)
    "#,
    crate::save_context!(),
    r#"
        // call C handler
        bl      _irq_handler
        // restore from the system stack
    "#,
    crate::restore_context!(),
    r#"
        // restore LR
        pop     {{lr}}
        // pop CPSR and LR from the stack (which also restores the mode)
        rfefd   sp!
    .size _asm_default_irq_handler, . - _asm_default_irq_handler

    "#,
    sys_mode = const crate::ProcessorMode::Sys as u8,
);
