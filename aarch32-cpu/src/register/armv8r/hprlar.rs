//! Code for managing HPRLAR (*Hyp Protection Region Limit Address Register*)

use arbitrary_int::{u26, u3};

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR (*Hyp Protection Region Limit Address Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct Hprlar {
    /// Length of region
    #[bits(6..=31, rw)]
    limit: u26,
    /// Which HMAIR attribute to use
    #[bits(1..=3, rw)]
    mair: u3,
    /// Is region enabled?
    #[bits(0..=0, rw)]
    enabled: bool,
}

impl SysReg for Hprlar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 3;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Hprlar {}

impl Hprlar {
    #[inline]
    /// Reads HPRLAR (*Hyp Protection Region Limit Address Register*)
    pub fn read() -> Hprlar {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprlar {}

impl Hprlar {
    #[inline]
    /// Writes HPRLAR (*Hyp Protection Region Limit Address Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
