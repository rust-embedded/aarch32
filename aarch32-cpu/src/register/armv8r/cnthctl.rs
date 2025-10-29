//! Code for managing CNTHCTL (*Hyp Counter-timer Control Register*)

use arbitrary_int::u4;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTHCTL (*Hyp Counter-timer Control Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cnthctl {
    #[bits(19..=19, rw)]
    cntpmask: bool,
    #[bits(18..=18, rw)]
    cntvmask: bool,
    #[bits(17..=17, rw)]
    evntis: bool,
    #[bits(16..=16, rw)]
    el1nvvct: bool,
    #[bits(15..=15, rw)]
    el1nvpct: bool,
    #[bits(14..=14, rw)]
    el1tvct: bool,
    #[bits(13..=13, rw)]
    el1tvt: bool,
    #[bits(12..=12, rw)]
    ecv: bool,
    #[bits(11..=11, rw)]
    el1pten: bool,
    #[bits(10..=10, rw)]
    el1pcten: bool,
    #[bits(9..=9, rw)]
    el0pten: bool,
    #[bits(8..=8, rw)]
    el0vten: bool,
    #[bits(4..=7, rw)]
    evnti: u4,
    #[bits(3..=3, rw)]
    evntdir: bool,
    #[bits(2..=2, rw)]
    evnten: bool,
    #[bits(1..=1, rw)]
    el0vcten: bool,
    #[bits(0..=0, rw)]
    el0pcten: bool,
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
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
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
