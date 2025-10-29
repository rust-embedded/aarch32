//! Simple assembly routines for ARMv4

/// Emit an NOP instruction
#[cfg_attr(not(feature = "check-asm"), inline)]
pub fn nop() {
    unsafe { core::arch::asm!("nop", options(nomem, nostack, preserves_flags)) }
}

/// Mask IRQ
#[cfg_attr(not(feature = "check-asm"), inline)]
#[instruction_set(arm::a32)]
pub fn irq_disable() {
    unsafe {
        core::arch::asm!(r#"
            mrs {0}, cpsr 
            orr {0}, {flag}
            msr cpsr, {0}
        "#,
        in(reg) 0,
        flag = const {
            crate::register::Cpsr::new_with_raw_value(0)
                .with_i(true)
                .raw_value()
        },
        options(nomem, nostack, preserves_flags));
    };
}

/// Unmask IRQ
#[cfg_attr(not(feature = "check-asm"), inline)]
#[instruction_set(arm::a32)]
pub fn irq_enable() {
    unsafe {
        core::arch::asm!(r#"
            mrs {0}, cpsr 
            bic {0}, #{flag}
            msr cpsr, {0}
        "#,
        in(reg) 0,
        flag = const {
            crate::register::Cpsr::new_with_raw_value(0)
                .with_i(true)
                .raw_value()
        },
        options(nomem, nostack, preserves_flags));
    };
}

/// Which core are we?
///
/// Return the bottom 24-bits of the MPIDR
#[cfg_attr(not(feature = "check-asm"), inline)]
#[instruction_set(arm::a32)]
pub fn core_id() -> u32 {
    let r: u32;
    unsafe {
        core::arch::asm!("MRC p15, 0, {}, c0, c0, 5", out(reg) r, options(nomem, nostack, preserves_flags));
    }
    r & 0x00FF_FFFF
}

#[no_mangle]
pub extern "C" fn __sync_synchronize() {
    // we don't have a barrier instruction - the linux kernel just uses an empty inline asm block
    unsafe {
        core::arch::asm!("");
    }
}
