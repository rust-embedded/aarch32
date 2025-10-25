//! Code for managing PMEVTYPER0 (*Performance Monitors Event Type Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMEVTYPER0 (*Performance Monitors Event Type Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmevtyper0(pub u32);
impl SysReg for Pmevtyper0 {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Pmevtyper0 {}
impl Pmevtyper0 {
    #[inline]
    /// Reads PMEVTYPER0 (*Performance Monitors Event Type Register 0*)
    pub fn read() -> Pmevtyper0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Pmevtyper0 {}
impl Pmevtyper0 {
    #[inline]
    /// Writes PMEVTYPER0 (*Performance Monitors Event Type Register 0*)
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
