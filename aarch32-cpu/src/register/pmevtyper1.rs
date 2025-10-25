//! Code for managing PMEVTYPER1 (*Performance Monitors Event Type Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMEVTYPER1 (*Performance Monitors Event Type Register 1*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmevtyper1(pub u32);
impl SysReg for Pmevtyper1 {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Pmevtyper1 {}
impl Pmevtyper1 {
    #[inline]
    /// Reads PMEVTYPER1 (*Performance Monitors Event Type Register 1*)
    pub fn read() -> Pmevtyper1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Pmevtyper1 {}
impl Pmevtyper1 {
    #[inline]
    /// Writes PMEVTYPER1 (*Performance Monitors Event Type Register 1*)
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
