//! Code for managing CSSELR (*Cache Size Selection Register*)
use arbitrary_int::u3;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CacheType {
    DataOrUnified = 0,
    Instruction = 1,
}

/// CSSELR (*Cache Size Selection Register*)
#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq)]
pub struct Csselr {
    /// 0 for L1 cache, 1 for L2, etc.
    #[bits(1..=3, rw)]
    level: u3,
    #[bit(0, rw)]
    cache_type: CacheType,
}

impl SysReg for Csselr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 2;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Csselr {}

impl Csselr {
    #[inline]
    /// Reads CSSELR (*Cache Size Selection Register*)
    pub fn read() -> Csselr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Csselr {}

impl Csselr {
    #[inline]
    /// Writes CSSELR (*Cache Size Selection Register*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
