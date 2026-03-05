//! Code for managing CNTKCTL (*Counter-timer Kernel Control Register*)

use arbitrary_int::u4;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTKCTL (*Counter-timer Kernel Control Register*)
#[bitbybit::bitfield(u32, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cntkctl {
    /// Controls whether the physical timer registers are accessible from EL0
    /// modes.
    #[bits(9..=9, rw)]
    el0pten: bool,
    /// Controls whether the virtual timer registers are accessible from EL0
    /// modes.
    #[bits(8..=8, rw)]
    el0vten: bool,
    /// Selects which bit of CNTVCT is the trigger for the event stream
    /// generated from the virtual counter, when that stream is enabled.
    #[bits(4..=7, rw)]
    evnti: u4,
    /// Controls which transition of the CNTVCT trigger bit, defined by EVNTI,
    /// generates an event, when the event stream is enabled.
    ///
    /// * true: a 1-0 transition
    /// * false: a 0-1 transition
    #[bits(3..=3, rw)]
    evntdir: bool,
    /// Enables the generation of an event stream from the virtual counter.
    #[bits(2..=2, rw)]
    evnten: bool,
    /// Controls whether the virtual counter, CNTVCT, and the frequency register
    /// CNTFRQ, are accessible from EL0 modes.
    #[bits(1..=1, rw)]
    el0vcten: bool,
    /// Controls whether the physical counter, CNTPCT, and the frequency
    /// register CNTFRQ, are accessible from EL0 modes.
    #[bits(0..=0, rw)]
    el0pcten: bool,
}

impl core::fmt::Debug for Cntkctl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Cntkctl")
            .field("el0pten", &self.el0pten())
            .field("el0vten", &self.el0vten())
            .field("evnti", &self.evnti())
            .field("evntdir", &self.evntdir())
            .field("evnten", &self.evnten())
            .field("el0vcten", &self.el0vcten())
            .field("el0pcten", &self.el0pcten())
            .finish()
    }
}

impl SysReg for Cntkctl {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}

impl SysRegRead for Cntkctl {}

impl Cntkctl {
    #[inline]
    /// Reads CNTKCTL (*Counter-timer Kernel Control Register*)
    pub fn read() -> Cntkctl {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for Cntkctl {}

impl Cntkctl {
    #[inline]
    /// Writes CNTKCTL (*Counter-timer Kernel Control Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }

    #[inline]
    /// Modifies CNTKCTL (*Counter-timer Kernel Control Register*)
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }
}
