//! Code for managing CNTHCTL (*Hyp Counter-timer Control Register*)

use arbitrary_int::u4;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTHCTL (*Hyp Counter-timer Control Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cnthctl {
    /// Selects which bit of CNTPCT, as seen from EL2, is the trigger for the
    /// event stream generated from that counter when that stream is enabled.
    #[bits(4..=7, rw)]
    evnti: u4,
    /// Controls which transition of the CNTPCT trigger bit, as seen from EL2
    /// and defined by EVNTI, generates an event when the event stream is
    /// enabled.
    #[bits(3..=3, rw)]
    evntdir: bool,
    /// Enables the generation of an event stream from CNTPCT as seen from EL2.
    #[bits(2..=2, rw)]
    evnten: bool,
    /// Traps Non-secure EL0 and EL1 MRC or MCR accesses, reported using EC
    /// syndrome value 0x03, and MRRC or MCRR accesses, reported using EC
    /// syndrome value 0x04, to the physical timer registers to Hyp mode.
    #[bits(1..=1, rw)]
    pl1pcen: bool,
    /// Traps Non-secure EL0 and EL1 MRRC or MCRR accesses, reported using EC
    /// syndrome value 0x04, to the physical counter register to Hyp mode.
    #[bits(0..=0, rw)]
    pl1pcten: bool,
}

impl SysReg for Cnthctl {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}

impl SysRegRead for Cnthctl {}

impl Cnthctl {
    #[inline]
    /// Reads CNTHCTL (*Hyp Counter-timer Control Register*)
    pub fn read() -> Cnthctl {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl SysRegWrite for Cnthctl {}

impl Cnthctl {
    #[inline]
    /// Writes CNTHCTL (*Hyp Counter-timer Control Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
