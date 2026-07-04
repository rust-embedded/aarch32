//! Data and Prefetch Abort handlers for Armv7 and higher

core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _data_abort_handler(addr: usize);`
    .pushsection .text._asm_default_data_abort_handler
    .arm
    .global _asm_default_data_abort_handler
    .type _asm_default_data_abort_handler, %function
    _asm_default_data_abort_handler:
        sub     lr, lr, #8                // Make sure we jump back to the right place
        srsfd   sp!, #{abt_mode}          // Store return state to ABT stack (1)
        push    {{ r12 }}                 // Push preserved register R12 (2)
        and     r12, sp, 7                // Align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here (3)
        push    {{ r0-r4, r12 }}          // Push alignment amount, and preserved registers (R4 is just padding) (4)
    "#,
    crate::fpu_context!("save"),
    r#"
        mov     r0, lr                    // Pass the faulting instruction address to the handler.
        bl      _data_abort_handler       // Call C handler
        mov     lr, r0                    // If we get back here, assume they returned a new LR in r0
    "#,
    crate::fpu_context!("restore"),
    r#"
        pop     {{ r0-r4, r12 }}          // Pop preserved registers, dummy value, and alignment amount to undo (4)
        add     sp, r12                   // Restore SP alignment using R12 to undo (3)
        pop     {{ r12 }}                 // Pop R12 to undo (2)
        str     lr, [sp]                  // Overwrite the saved LR with the one from the C handler
        rfefd   sp!                       // Return from exception to undo (1)
    .size _asm_default_data_abort_handler, . - _asm_default_data_abort_handler
    .popsection
    "#,
    abt_mode = const crate::ProcessorMode::Abt as u8,
);

core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3
  
    // Called from the vector table when we have a prefetch abort.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _prefetch_abort_handler(addr: usize);`
    .pushsection .text._asm_default_prefetch_abort_handler
    .arm
    .global _asm_default_prefetch_abort_handler
    .type _asm_default_prefetch_abort_handler, %function
    _asm_default_prefetch_abort_handler:
        sub     lr, lr, #4                // Make sure we jump back to the right place
        srsfd   sp!, #{abt_mode}          // Store return state to ABT stack (1)
        push    {{ r12 }}                 // Push preserved register R12 (2)
        and     r12, sp, 7                // Align SP down to eight byte boundary using R12
        sub     sp, r12                   // SP now aligned - only push 64-bit values from here (3)
        push    {{ r0-r4, r12 }}          // Push reserved registers and alignment amount (R4 is just padding) (4)
    "#,
    crate::fpu_context!("save"),
    r#"
        mov     r0, lr                    // Pass the faulting instruction address to the handler.
        bl      _prefetch_abort_handler   // Call C handler
        mov     lr, r0                    // If we get back here, assume they returned a new LR in r0
    "#,
    crate::fpu_context!("restore"),
    r#"
        pop     {{ r0-r4, r12 }}          // Pop preserved registers, dummy value, and alignment amount to undo (4)
        add     sp, r12                   // Restore SP alignment using R12 to undo (3)
        pop     {{ r12 }}                 // Pop R12 to undo (2)
        str     lr, [sp]                  // Overwrite the saved LR with the one from the C handler to undo (1)
        rfefd   sp!                       // Return from exception
    .size _asm_default_prefetch_abort_handler, . - _asm_default_prefetch_abort_handler
    .popsection
   "#,
    abt_mode = const crate::ProcessorMode::Abt as u8,
);
