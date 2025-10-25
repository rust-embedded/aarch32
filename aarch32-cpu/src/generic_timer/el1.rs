//! Code and types for Generic Timer support at EL1 on Armv8-R.

use crate::register;

use super::{El0PhysicalTimer, El0VirtualTimer, GenericTimer};

/// Represents our Physical Timer when we are running at EL1.
pub struct El1PhysicalTimer(pub(crate) El0PhysicalTimer);

impl El1PhysicalTimer {
    /// Create an EL1 Generic Timer handle
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El1PhysicalTimer {
        unsafe { El1PhysicalTimer(El0PhysicalTimer::new()) }
    }

    /// Control whether user code at EL0 can access the physical counter.
    pub fn el0_access_physical_counter(&mut self, access: bool) {
        register::Cntkctl::modify(|r| {
            r.set_el0pcten(access);
        });
    }

    /// Control whether user code at EL0 can access the physical timer.
    pub fn el0_access_physical_timer(&mut self, access: bool) {
        register::Cntkctl::modify(|r| {
            r.set_el0pten(access);
        });
    }
}

impl GenericTimer for El1PhysicalTimer {
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
pub struct El1VirtualTimer(El0VirtualTimer);

impl El1VirtualTimer {
    /// Create an EL1 Generic Timer handle
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El1VirtualTimer {
        unsafe { El1VirtualTimer(El0VirtualTimer::new()) }
    }

    /// Control whether user code at EL0 can access the virtual counter.
    pub fn el0_access_virtual_counter(&mut self, access: bool) {
        register::Cntkctl::modify(|r| {
            r.set_el0vcten(access);
        });
    }

    /// Control whether user code at EL0 can access the virtual timer.
    pub fn el0_access_virtual_timer(&mut self, access: bool) {
        register::Cntkctl::modify(|r| {
            r.set_el0vten(access);
        });
    }

    /// Configure an event stream from the virtual counter.
    ///
    /// The event stream is tied to one of the bottom 16 bits of the virtual
    /// counter. If you select the bottom (0th) bit, the event fires every
    /// counter tick. If you select the 3rd bit, the event fires every 2^3 = 8
    /// counter ticks.
    ///
    /// This is useful if you want to ensure that a WFE instruction can never
    /// wait forever; effectively it allows you to put a timeout on a WFE.
    ///
    /// Pass None to disable.
    pub fn virtual_event_stream_configure(&mut self, event_config: Option<&super::EventConfig>) {
        if let Some(event_config) = event_config {
            register::Cntkctl::modify(|r| {
                r.set_evnti(arbitrary_int::u4::from_u8(event_config.rate as u8));
                r.set_evntdir(event_config.evntdir == super::EventDir::HighLow);
                r.set_evnten(true);
            });
        } else {
            register::Cntkctl::modify(|r| {
                r.set_evnten(false);
            });
        }
    }
}

impl GenericTimer for El1VirtualTimer {
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
