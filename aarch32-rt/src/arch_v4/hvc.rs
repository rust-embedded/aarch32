//! Dummy hypervisor handler for architectures without HYP mode

// # _asm_default_hvc_handler
//
// Never called but makes the linker happy
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2
    .pushsection .text._asm_default_hvc_handler
    .arm
    .global _asm_default_hvc_handler
    .type _asm_default_hvc_handler, %function
    _asm_default_hvc_handler:
        b       .
    .size _asm_default_hvc_handler, . - _asm_default_hvc_handler
    .popsection
    "#,
);
