//! Code and types for Generic Timer support at EL0 on Armv8-R.

use crate::register;

/// Represents our Generic Timer when we are running at EL0.
///
/// Note that for most of these APIs to work, EL0 needs to have been granted
/// access using methods like
/// [El1PhysicalTimer::el0_access_physical_counter](crate::generic_timer::El1PhysicalTimer::el0_access_physical_counter).
pub struct El0PhysicalTimer();

impl El0PhysicalTimer {
    /// Create an EL0 Timer handle for the Physical Timer.
    ///
    /// EL2/EL1 has to grant permission for EL0 to use the Physical Timer, so
    /// check they did that.
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that
    /// state.
    pub unsafe fn new() -> El0PhysicalTimer {
        El0PhysicalTimer()
    }
}

impl super::GenericTimer for El0PhysicalTimer {
    fn frequency_hz(&self) -> u32 {
        register::Cntfrq::read().0
    }

    fn counter(&self) -> u64 {
        register::CntPct::read().0
    }

    fn counter_compare(&self) -> u64 {
        register::CntpCval::read().0
    }

    fn counter_compare_set(&mut self, value: u64) {
        register::CntpCval::write(register::CntpCval(value))
    }

    fn countdown(&self) -> u32 {
        register::CntpTval::read().0
    }

    fn countdown_set(&mut self, duration_ticks: u32) {
        register::CntpTval::write(register::CntpTval(duration_ticks))
    }

    fn enabled(&self) -> bool {
        register::CntpCtl::read().enable()
    }

    fn enable(&self, enabled: bool) {
        register::CntpCtl::modify(|r| {
            r.set_enable(enabled);
        });
    }

    fn interrupt_masked(&self) -> bool {
        register::CntpCtl::read().imask()
    }

    fn interrupt_mask(&mut self, mask: bool) {
        register::CntpCtl::modify(|r| {
            r.set_imask(mask);
        });
    }

    fn interrupt_status(&self) -> bool {
        register::CntpCtl::read().istatus()
    }
}

pub struct El0VirtualTimer();

impl El0VirtualTimer {
    /// Create an EL0 Timer handle for the Virtual Timer.
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El0VirtualTimer {
        El0VirtualTimer()
    }
}

impl super::GenericTimer for El0VirtualTimer {
    fn frequency_hz(&self) -> u32 {
        register::Cntfrq::read().0
    }

    fn counter(&self) -> u64 {
        register::CntVct::read().0
    }

    fn counter_compare(&self) -> u64 {
        register::CntvCval::read().0
    }

    fn counter_compare_set(&mut self, value: u64) {
        register::CntvCval::write(register::CntvCval(value))
    }

    fn countdown(&self) -> u32 {
        register::CntvTval::read().0
    }

    fn countdown_set(&mut self, duration_ticks: u32) {
        register::CntvTval::write(register::CntvTval(duration_ticks))
    }

    fn enabled(&self) -> bool {
        register::CntvCtl::read().enable()
    }

    fn enable(&self, enabled: bool) {
        register::CntvCtl::modify(|r| {
            r.set_enable(enabled);
        });
    }

    fn interrupt_masked(&self) -> bool {
        register::CntvCtl::read().imask()
    }

    fn interrupt_mask(&mut self, mask: bool) {
        register::CntvCtl::modify(|r| {
            r.set_imask(mask);
        });
    }

    fn interrupt_status(&self) -> bool {
        register::CntvCtl::read().istatus()
    }
}
