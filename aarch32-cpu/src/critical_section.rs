//! Code that implements the `critical-section` traits on Cortex-R or Cortex-A
//!
//! We have single-core and multi-core versions. Select with the
//! `critical-section-single-core` and `critical-section-multi-core` features.

#[cfg(feature = "critical-section-single-core")]
mod single_core {
    struct SingleCoreCriticalSection;

    critical_section::set_impl!(SingleCoreCriticalSection);

    /// Indicates the critical section was entered with interrupts on
    pub const INT_ON: u8 = 0;

    /// Indicates the critical section was entered with interrupts off
    pub const INT_OFF: u8 = 1;

    #[cfg(feature = "critical-section-single-core")]
    unsafe impl critical_section::Impl for SingleCoreCriticalSection {
        unsafe fn acquire() -> critical_section::RawRestoreState {
            use core::sync::atomic;
            // the i bit means "masked"
            let was_active = !crate::register::Cpsr::read().i();
            crate::interrupt::disable();
            atomic::compiler_fence(atomic::Ordering::SeqCst);
            if was_active {
                INT_ON
            } else {
                INT_OFF
            }
        }

        unsafe fn release(was_active: critical_section::RawRestoreState) {
            use core::sync::atomic;
            // Only re-enable interrupts if they were enabled before the critical section.
            if was_active == INT_ON {
                atomic::compiler_fence(atomic::Ordering::SeqCst);
                // Safety: This is OK because we're releasing a lock that was
                // entered with interrupts enabled
                unsafe {
                    crate::interrupt::enable();
                }
            }
        }
    }
}

#[cfg(feature = "critical-section-multi-core")]
mod multi_core {
    struct MultiCoreCriticalSection;

    critical_section::set_impl!(MultiCoreCriticalSection);

    /// The default value for our spin-lock
    pub const UNLOCKED: u32 = 0xFFFF_FFFF;

    /// Indicates the critical section was entered with interrupts on, and the spin-lock unlocked
    pub const INT_ON_UNLOCKED: u8 = 0;

    /// Indicates the critical section was entered with interrupts off, and the spin-lock locked (by us)
    pub const INT_OFF_LOCKED: u8 = 1;

    /// Indicates the critical section was entered with interrupts off, and the spin-lock unlocked
    pub const INT_OFF_UNLOCKED: u8 = 2;

    pub static CORE_SPIN_LOCK: core::sync::atomic::AtomicU32 =
        core::sync::atomic::AtomicU32::new(UNLOCKED);
    unsafe impl critical_section::Impl for MultiCoreCriticalSection {
        unsafe fn acquire() -> critical_section::RawRestoreState {
            use core::sync::atomic;

            // the i bit means "masked"
            let was_active = !crate::register::Cpsr::read().i();
            crate::interrupt::disable();

            let core_id = crate::asm::core_id();

            let locked_already = loop {
                match CORE_SPIN_LOCK.compare_exchange(
                    UNLOCKED,
                    core_id,
                    atomic::Ordering::Acquire,
                    atomic::Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        // we got the lock
                        break false;
                    }
                    Err(n) if n == core_id => {
                        // we already held the lock
                        break true;
                    }
                    Err(_) => {
                        // someone else holds the lock
                        core::hint::spin_loop();
                    }
                }
            };

            atomic::compiler_fence(atomic::Ordering::SeqCst);

            match (was_active, locked_already) {
                (true, true) => {
                    panic!("Invalid CS state?!");
                }
                (true, false) => {
                    // we need to turn interrupts on, and release the lock
                    INT_ON_UNLOCKED
                }
                (false, false) => {
                    // we need release the lock
                    INT_OFF_UNLOCKED
                }
                (false, true) => {
                    // we need to do nothing
                    INT_OFF_LOCKED
                }
            }
        }

        unsafe fn release(was_active: critical_section::RawRestoreState) {
            use core::sync::atomic;

            atomic::compiler_fence(atomic::Ordering::SeqCst);
            match was_active {
                INT_OFF_LOCKED => {
                    // do nothing
                }
                INT_OFF_UNLOCKED => {
                    // the spin-lock was unlocked before, so unlock it
                    CORE_SPIN_LOCK.store(UNLOCKED, atomic::Ordering::Release);
                }
                INT_ON_UNLOCKED => {
                    // the spin-lock was unlocked before, so unlock it
                    CORE_SPIN_LOCK.store(UNLOCKED, atomic::Ordering::Release);
                    // Safety: This is OK because we're releasing a lock that was
                    // entered with interrupts enabled
                    unsafe {
                        crate::interrupt::enable();
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }
}
