//! IRQ handler for for Armv7 and higher

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

    .section .text._asm_default_irq_handler

    // Called from the vector table when we have an interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _irq_handler();`
    .global _asm_default_irq_handler
    .type _asm_default_irq_handler, %function
    _asm_default_irq_handler:
        sub     lr, lr, 4                 // make sure we jump back to the right place
        srsfd   sp!, #{sys_mode}          // store return state to SYS stack
        cps     #{sys_mode}               // switch to system mode (because if we interrupt in IRQ mode we trash IRQ mode's LR)
        push    {{ lr }}                  // save adjusted LR to SYS stack
        mov     lr, sp                    // align SP down to eight byte boundary using LR
        and     lr, lr, 7                 //
        sub     sp, lr                    // SP now aligned - only push 64-bit values from here
        push    {{ r0-r3, r12, lr }}      // push alignment amount (in LR) and preserved registers
     "#,
    crate::save_fpu_context!(),
    r#"
        bl      _irq_handler              // call C handler (they may choose to re-enable interrupts)
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r3, r12, lr }}      // restore alignment amount (in LR) and preserved registers
        add     sp, lr                    // restore SP alignment using LR
        pop     {{ lr }}                  // restore adjusted LR
        rfefd   sp!                       // return from exception
    .size _asm_default_irq_handler, . - _asm_default_irq_handler
    "#,
    sys_mode = const crate::ProcessorMode::Sys as u8,
);
