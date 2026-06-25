//! IRQ handler for for Armv7 and higher

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

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
    // Because this is ARMv7, we can save state (SPSR_irq and LR_irq) straight
    // to another mode's stack, meaning that we never actually push anything to
    // the IRQ stack. You can therefore run with an IRQ stack size of zero.
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
        srsfd   sp!, #{handler_mode}      // store return state to the handler stack (1)
        cps     #{handler_mode}           // switch to handler mode (2)
        push    {{ lr }}                  // save adjusted LR to handler mode stack (3)
        and     lr, sp, 7                 // align SP down to eight byte boundary using LR
        sub     sp, lr                    // SP now aligned - only push 64-bit values from here (4)
        push    {{ r0-r3, r12, lr }}      // push alignment amount (in LR) and preserved registers (5)
     "#,
    crate::save_fpu_context!(),
    r#"
        bl      _irq_handler              // call C handler (they may choose to re-enable interrupts)
    "#,
    crate::restore_fpu_context!(),
    r#"
        pop     {{ r0-r3, r12, lr }}      // restore alignment amount (in LR) and preserved registers to undo (5)
        add     sp, lr                    // restore SP alignment using LR to undo (4)
        pop     {{ lr }}                  // restore adjusted LR to undo (3)
        rfefd   sp!                       // return from exception to undo (2) and (1) together
    .size _asm_default_irq_handler, . - _asm_default_irq_handler
    .popsection
    "#,
    handler_mode = const HANDLER_MODE,
);

#[cfg(feature = "svc-stack-interrupt")]
const HANDLER_MODE: u8 = crate::ProcessorMode::Svc as u8;

#[cfg(not(feature = "svc-stack-interrupt"))]
const HANDLER_MODE: u8 = crate::ProcessorMode::Sys as u8;
