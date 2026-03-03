//! Code and types for Generic Timer support at EL2 on Armv8-R.

use crate::register;

use super::{El1PhysicalTimer, El1VirtualTimer, GenericTimer};

/// Represents our Physical Timer when we are running at EL2.
pub struct El2PhysicalTimer(El1PhysicalTimer);

impl El2PhysicalTimer {
    /// Create an EL2 Physical Timer handle
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El2PhysicalTimer {
        unsafe { El2PhysicalTimer(El1PhysicalTimer::new()) }
    }

    /// Set frequency
    ///
    /// Sets the frequency, in Hz, that the counters are incrementing at. You
    /// might need to call this if your system doesn't initialise the frequency
    /// value to something appropriate, or if you change the clock speed of the
    /// timer.
    pub fn frequency_hz_set(&mut self, new_frequency_hz: u32) {
        register::Cntfrq::write(register::Cntfrq(new_frequency_hz))
    }
}

impl GenericTimer for El2PhysicalTimer {
    fn frequency_hz(&self) -> u32 {
        self.0.frequency_hz()
    }

    fn counter(&self) -> u64 {
        self.0.counter()
    }

    fn counter_compare(&self) -> u64 {
        self.0.counter_compare()
    }

    fn counter_compare_set(&mut self, value: u64) {
        self.0.counter_compare_set(value)
    }

    fn countdown(&self) -> u32 {
        self.0.countdown()
    }

    fn countdown_set(&mut self, duration_ticks: u32) {
        self.0.countdown_set(duration_ticks)
    }

    fn enabled(&self) -> bool {
        self.0.enabled()
    }

    fn enable(&self, enabled: bool) {
        self.0.enable(enabled)
    }

    fn interrupt_masked(&self) -> bool {
        self.0.interrupt_masked()
    }

    fn interrupt_mask(&mut self, mask: bool) {
        self.0.interrupt_mask(mask)
    }

    fn interrupt_status(&self) -> bool {
        self.0.interrupt_status()
    }
}

/// Represents our Virtual Timer when we are running at EL1.
pub struct El2VirtualTimer(El1VirtualTimer);

impl El2VirtualTimer {
    /// Create an EL2 Generic Timer handle
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El2VirtualTimer {
        unsafe { El2VirtualTimer(El1VirtualTimer::new()) }
    }

    /// Set frequency
    ///
    /// Sets the frequency, in Hz, that the counters are incrementing at. You
    /// might need to call this if your system doesn't initialise the frequency
    /// value to something appropriate, or if you change the clock speed of the
    /// timer.
    pub fn frequency_hz_set(&mut self, new_frequency_hz: u32) {
        register::Cntfrq::write(register::Cntfrq(new_frequency_hz))
    }
}

impl GenericTimer for El2VirtualTimer {
    fn frequency_hz(&self) -> u32 {
        self.0.frequency_hz()
    }

    fn counter(&self) -> u64 {
        self.0.counter()
    }

    fn counter_compare(&self) -> u64 {
        self.0.counter_compare()
    }

    fn counter_compare_set(&mut self, value: u64) {
        self.0.counter_compare_set(value)
    }

    fn countdown(&self) -> u32 {
        self.0.countdown()
    }

    fn countdown_set(&mut self, duration_ticks: u32) {
        self.0.countdown_set(duration_ticks)
    }

    fn enabled(&self) -> bool {
        self.0.enabled()
    }

    fn enable(&self, enabled: bool) {
        self.0.enable(enabled)
    }

    fn interrupt_masked(&self) -> bool {
        self.0.interrupt_masked()
    }

    fn interrupt_mask(&mut self, mask: bool) {
        self.0.interrupt_mask(mask)
    }

    fn interrupt_status(&self) -> bool {
        self.0.interrupt_status()
    }
}

/// Represents our Hypervisor-specific Physical Timer when we are running at EL1.
pub struct El2HypPhysicalTimer();

impl El2HypPhysicalTimer {
    /// Create a Timer handle for the EL2-specific Hyp Physical Timer.
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El2HypPhysicalTimer {
        El2HypPhysicalTimer()
    }
}

impl super::GenericTimer for El2HypPhysicalTimer {
    fn frequency_hz(&self) -> u32 {
        register::Cntfrq::read().0
    }

    fn counter(&self) -> u64 {
        register::CntPct::read().0
    }

    fn counter_compare(&self) -> u64 {
        register::CnthpCval::read().0
    }

    fn counter_compare_set(&mut self, value: u64) {
        register::CnthpCval::write(register::CnthpCval(value))
    }

    fn countdown(&self) -> u32 {
        register::CnthpTval::read().0
    }

    fn countdown_set(&mut self, duration_ticks: u32) {
        register::CnthpTval::write(register::CnthpTval(duration_ticks))
    }

    fn enabled(&self) -> bool {
        register::CnthpCtl::read().enable()
    }

    fn enable(&self, enabled: bool) {
        register::CnthpCtl::modify(|r| {
            r.set_enable(enabled);
        });
    }

    fn interrupt_masked(&self) -> bool {
        register::CnthpCtl::read().imask()
    }

    fn interrupt_mask(&mut self, mask: bool) {
        register::CnthpCtl::modify(|r| {
            r.set_imask(mask);
        });
    }

    fn interrupt_status(&self) -> bool {
        register::CnthpCtl::read().istatus()
    }
}
