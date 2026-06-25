//! IRQ handler for for Armv4 to Armv6

use crate::{Cpsr, ProcessorMode};

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    // Called from the vector table when we have an interrupt. Saves state and
    // calls a C-compatible handler like `extern "C" fn _irq_handler();` in
    // system mode (or SVC mode if the `svc-stack-interrupt` feature is
    // enabled).
    //
    // We call the C-compatible handler in a different mode because when when an
    // IRQ occurs, the PC is copied to LR_irq immediately. If the C code was
    // running in IRQ mode, then it will be using LR_irq for normal LR things
    // (because that's the LR register when you are in IRQ mode). Instantly
    // trashing the LR register of running code is bad. So, by switching to SYS
    // mode (or SVC mode) we ensure that LR_irq is always unused at the point an
    // IRQ occurs.
    //
    // See [ARM Cortex-R Series (Armv7-R) Programmer's Guide] for more details.
    //
    // [ARM Cortex-R Series (Armv7-R) Programmer's Guide]:
    //     https://developer.arm.com/documentation/den0042/0100/Exceptions-and-Interrupts/External-interrupt-requests/Nested-interrupt-handling
    .pushsection .text._asm_default_irq_handler
    .arm
    .global _asm_default_irq_handler
    .type _asm_default_irq_handler, %function
    _asm_default_irq_handler:
        sub     lr, lr, 4                 // make sure we jump back to the right place
        stmfd   sp!, {{ lr }}             // save adjusted LR to IRQ stack (1)
        mrs     lr, spsr                  // The hardware has copied the interrupted task's CPSR to SPSR_irq - grab it (2) and
        push    {{ lr }}                  //   save it to IRQ stack using LR (3)
        msr     cpsr_c, {handler_mode}    // switch to handler mode (4)
        push    {{ lr }}                  // Save LR of handler mode before using it for stack alignment (5)
        and     lr, sp, 7                 // align SP down to eight byte boundary using LR
        sub     sp, lr                    // SP now aligned - only push 64-bit values from here (6)
        push    {{ r0-r3, r12, lr }}      // push alignment amount (in LR) and preserved registers (7)
     "#,
    crate::save_fpu_context!(),
    r#"
        bl      _irq_handler              // call C handler in the selected handler mode (they may choose to re-enable interrupts)
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r3, r12, lr }}      // restore alignment amount (in LR) and preserved registers to undo (7)
        add     sp, lr                    // restore SP alignment using LR to undo (6)
        pop     {{ lr }}                  // Restore the actual link register of handler mode to undo (5)
        msr     cpsr_c, {irq_mode}        // switch back to IRQ mode (with IRQ masked) to undo (4)
        pop     {{ lr }}                  // load SPSR to undo (3)
        msr     spsr, lr                  // restore SPSR to undo (2)
        ldmfd   sp!, {{ pc }}^            // return from exception (^ => restore SPSR to CPSR) to undo (1)
    .size _asm_default_irq_handler, . - _asm_default_irq_handler
    .popsection
    "#,
    handler_mode = const HANDLER_MODE,
    irq_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Irq)
            .with_i(true)
            .raw_value()
    }
);

#[cfg(feature = "svc-stack-interrupt")]
const HANDLER_MODE: u32 = const {
    Cpsr::new_with_raw_value(0)
        .with_mode(ProcessorMode::Svc)
        .with_i(true)
        .raw_value()
};

#[cfg(not(feature = "svc-stack-interrupt"))]
const HANDLER_MODE: u32 = const {
    Cpsr::new_with_raw_value(0)
        .with_mode(ProcessorMode::Sys)
        .with_i(true)
        .raw_value()
};
