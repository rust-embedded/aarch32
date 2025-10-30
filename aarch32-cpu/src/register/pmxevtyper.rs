//! Code for managing PMXEVTYPER (*Performance Monitors Selected Event Type Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMXEVTYPER (*Performance Monitors Selected Event Type Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmxevtyper(pub u32);

impl SysReg for Pmxevtyper {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 13;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Pmxevtyper {}

impl Pmxevtyper {
    #[inline]
    /// Reads PMXEVTYPER (*Performance Monitors Selected Event Type Register*)
    pub fn read() -> Pmxevtyper {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmxevtyper {}

impl Pmxevtyper {
    #[inline]
    /// Writes PMXEVTYPER (*Performance Monitors Selected Event Type Register*)
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
