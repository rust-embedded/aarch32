//! Code for managing PMSELR (*Performance Monitors Event Counter Selection Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMSELR (*Performance Monitors Event Counter Selection Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmselr(pub u32);

impl SysReg for Pmselr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Pmselr {}

impl Pmselr {
    #[inline]
    /// Reads PMSELR (*Performance Monitors Event Counter Selection Register*)
    pub fn read() -> Pmselr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmselr {}

impl Pmselr {
    #[inline]
    /// Writes PMSELR (*Performance Monitors Event Counter Selection Register*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
