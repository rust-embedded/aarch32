//! IRQ handler for for Armv4 to Armv6

use crate::{Cpsr, ProcessorMode};

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
        // save our LR
        stmfd   sp!, {{ lr }}
        // The hardware has copied the interrupted task's CPSR to SPSR_irq
        mrs     lr, spsr
        stmfd   sp!, {{ lr }}
        // switch to system mode so we can handle another interrupt
        // (because if we interrupt irq mode we trash our own shadow registers)
        msr     cpsr_c, {sys_mode}
        // save state to the system stack (adjusting SP for alignment)
    "#,
    crate::save_context!(),
    r#"
        // call C handler (they may choose to re-enable interrupts)
        bl      _irq_handler
        // restore from the system stack
    "#,
    crate::restore_context!(),
    r#"
        // switch back to IRQ mode (with IRQ masked)
        msr     cpsr_c, {irq_mode}
        // load and restore SPSR
        ldmia   sp!, {{ lr }}
        msr     spsr, lr
        // return
        ldmfd   sp!, {{ pc }}^
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
