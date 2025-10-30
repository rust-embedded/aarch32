//! Code for managing PMEVTYPER2 (*Performance Monitors Event Type Register 2*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMEVTYPER2 (*Performance Monitors Event Type Register 2*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmevtyper2(pub u32);

impl SysReg for Pmevtyper2 {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 2;
}

impl crate::register::SysRegRead for Pmevtyper2 {}

impl Pmevtyper2 {
    #[inline]
    /// Reads PMEVTYPER2 (*Performance Monitors Event Type Register 2*)
    pub fn read() -> Pmevtyper2 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmevtyper2 {}

impl Pmevtyper2 {
    #[inline]
    /// Writes PMEVTYPER2 (*Performance Monitors Event Type Register 2*)
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
