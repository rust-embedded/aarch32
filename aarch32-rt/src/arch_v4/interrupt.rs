//! IRQ handler for for Armv4 to Armv6

use crate::{Cpsr, ProcessorMode};

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    // Called from the vector table when we have an interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _irq_handler();`
    .section .text._asm_default_irq_handler
    .arm
    .global _asm_default_irq_handler
    .type _asm_default_irq_handler, %function
    _asm_default_irq_handler:
        sub     lr, lr, 4                 // make sure we jump back to the right place
        push    {{ lr }}                  // save adjusted LR to IRQ stack
        mrs     lr, spsr                  // The hardware has copied the interrupted task's CPSR to SPSR_irq - grab it and
        push    {{ lr }}                  //   save it to IRQ stack using LR
        msr     cpsr_c, {sys_mode}        // switch to system mode (because if we interrupt in IRQ mode we trash IRQ mode's LR)
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
        msr     cpsr_c, {irq_mode}        // switch back to IRQ mode (with IRQ masked)
        pop     {{ lr }}                  // load and restore SPSR using LR
        msr     spsr, lr                  //
        ldmfd   sp!, {{ pc }}^            // return from exception (^ => restore SPSR to CPSR)
    .size _asm_default_irq_handler, . - _asm_default_irq_handler
    "#,
    // sys mode with IRQ masked
    sys_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Sys)
            .with_i(true)
            .raw_value()
    },
    irq_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Irq)
            .with_i(true)
            .raw_value()
    }
);
