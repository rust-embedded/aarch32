//! Simple assembly routines

use core::sync::atomic::{compiler_fence, Ordering};

/// Data Memory Barrier
///
/// Ensures that all explicit memory accesses that appear in program order before the `DMB`
/// instruction are observed before any explicit memory accesses that appear in program order
/// after the `DMB` instruction.
#[inline]
pub fn dmb() {
    compiler_fence(Ordering::SeqCst);
    unsafe {
        core::arch::asm!("dmb", options(nostack, preserves_flags));
    }
    compiler_fence(Ordering::SeqCst);
}

/// Data Synchronization Barrier
///
/// Acts as a special kind of memory barrier. No instruction in program order after this instruction
/// can execute until this instruction completes. This instruction completes only when both:
///
///  * any explicit memory access made before this instruction is complete
///  * all cache and branch predictor maintenance operations before this instruction complete
#[inline]
pub fn dsb() {
    compiler_fence(Ordering::SeqCst);
    unsafe {
        core::arch::asm!("dsb", options(nostack, preserves_flags));
    }
    compiler_fence(Ordering::SeqCst);
}

/// Instruction Synchronization Barrier
///
/// Flushes the pipeline in the processor, so that all instructions following the `ISB` are fetched
/// from cache or memory, after the instruction has been completed.
#[inline]
pub fn isb() {
    compiler_fence(Ordering::SeqCst);
    unsafe {
        core::arch::asm!("isb", options(nostack, preserves_flags));
    }
    compiler_fence(Ordering::SeqCst);
}

/// Emit an NOP instruction
#[inline]
pub fn nop() {
    unsafe { core::arch::asm!("nop", options(nomem, nostack, preserves_flags)) }
}

/// Emit an WFI instruction
#[inline]
pub fn wfi() {
    unsafe { core::arch::asm!("wfi", options(nomem, nostack, preserves_flags)) }
}

/// Emit an WFE instruction
#[inline]
pub fn wfe() {
    unsafe { core::arch::asm!("wfe", options(nomem, nostack, preserves_flags)) }
}

/// Emit an SEV instruction
#[inline]
pub fn sev() {
    unsafe {
        core::arch::asm!("sev");
    }
}

/// Which core are we?
///
/// Return the bottom 24-bits of the MPIDR
#[inline]
pub fn core_id() -> u32 {
    let r: u32;
    unsafe {
        core::arch::asm!("MRC p15, 0, {}, c0, c0, 5", out(reg) r, options(nomem, nostack, preserves_flags));
    }
    r & 0x00FF_FFFF
}
