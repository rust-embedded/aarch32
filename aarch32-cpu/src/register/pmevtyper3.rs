//! Code for managing PMEVTYPER3 (*Performance Monitors Event Type Register 3*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMEVTYPER3 (*Performance Monitors Event Type Register 3*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmevtyper3(pub u32);

impl SysReg for Pmevtyper3 {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 3;
}

impl crate::register::SysRegRead for Pmevtyper3 {}

impl Pmevtyper3 {
    #[inline]
    /// Reads PMEVTYPER3 (*Performance Monitors Event Type Register 3*)
    pub fn read() -> Pmevtyper3 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmevtyper3 {}

impl Pmevtyper3 {
    #[inline]
    /// Writes PMEVTYPER3 (*Performance Monitors Event Type Register 3*)
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
