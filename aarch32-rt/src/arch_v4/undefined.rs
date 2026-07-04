//! Undefined handler for Armv4 to Armv6

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2
        
    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _undefined_handler(addr: usize) -> usize;`
    // or
    // `extern "C" fn _undefined_handler(addr: usize) -> !;`
    .pushsection .text._asm_default_undefined_handler
    .arm
    .global _asm_default_undefined_handler
    .type _asm_default_undefined_handler, %function
    _asm_default_undefined_handler:
        push    {{ r12 }}                 // Push preserved register R12 (1)
        mrs     r12, spsr                 // Grab SPSR (2)
        push    {{ r12 }}                 // Push SPSR value (3)
        tst     r12, {t_bit}              // Was the code that triggered the exception in Thumb state?
        ite     eq                        // Adjust LR to point to faulting instruction - see p.1206 of the ARMv7-A architecture manual.
        subeq   lr, lr, #4                // Subtract 4 in Arm Mode
        subne   lr, lr, #2                // Subtract 2 in Thumb Mode
        and     r12, sp, 7                // Align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here (4)
        push    {{ r0-r4, r12 }}          // Push alignment amount, and preserved registers (R4 is just padding) (5)
    "#,
    crate::fpu_context!("save"),
    r#"
        mov     r0, lr                    // Pass the faulting instruction address to the handler.
        bl      _undefined_handler        // Call C handler
        mov     lr, r0                    // If we get back here, assume they returned a new LR in r0
    "#,
    crate::fpu_context!("restore"),
    r#"
        pop     {{ r0-r4, r12 }}          // Pop preserved registers, dummy value, and alignment amount to undo (5)
        add     sp, r12                   // Restore SP alignment using R12 to undo (4)
        pop     {{ r12 }}                 // Grab saved SPSR to undo (3)
        msr     spsr, r12                 // Restore SPSR using LR to undo (2)
        pop     {{ r12 }}                 // Pop R12 to undo (1)
        movs    pc, lr                    // Return from exception (movs => restore SPSR to CPSR)
    .size _asm_default_undefined_handler, . - _asm_default_undefined_handler
    .popsection
    "#,
    t_bit = const { crate::Cpsr::new_with_raw_value(0).with_t(true).raw_value() },
);
