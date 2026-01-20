//! Interrupts on Arm AArch32

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
    // Safety: as per outer function
    unsafe {
        crate::asm::irq_enable();
    }
}

/// Disable IRQ
///
/// * Doesn't work in User mode.
/// * Doesn't disable FIQ.
#[inline]
pub fn disable() {
    crate::asm::irq_disable();
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
