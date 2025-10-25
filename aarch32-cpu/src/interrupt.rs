//! Interrupts on Arm Cortex-R

use core::sync::atomic::{compiler_fence, Ordering};

/// Enable interrupts
///
/// * Doesn't work in User mode.
/// * Doesn't enable FIQ.
///
/// # Safety
///
/// Do not call this function inside an interrupt-based critical section
#[inline]
pub unsafe fn enable() {
    // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
    compiler_fence(Ordering::SeqCst);
    // Safety: We're atomically setting a bit in a special register, and we're
    // in an unsafe function that places restrictions on when you can call it
    #[cfg(target_arch = "arm")]
    unsafe {
        core::arch::asm!("dsb", "cpsie i", options(nomem, nostack, preserves_flags));
    };
}

/// Disable IRQ
///
/// * Doesn't work in User mode.
/// * Doesn't disable FIQ.
#[inline]
pub fn disable() {
    // Safety: We're atomically clearing a bit in a special register
    #[cfg(target_arch = "arm")]
    unsafe {
        core::arch::asm!("cpsid i", "dsb", options(nomem, nostack, preserves_flags));
    };
    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);
}

/// Run with interrupts disabled
///
/// * Doesn't work in User mode.
/// * Doesn't disable FIQ.
#[inline]
pub fn free<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let cpsr = crate::register::Cpsr::read();
    disable();
    let result = f();
    if cpsr.i() {
        // Safety: We're only turning them back on if they were on previously
        unsafe {
            enable();
        }
    }
    result
}
