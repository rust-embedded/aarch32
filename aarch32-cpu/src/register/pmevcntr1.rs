//! Code for managing PMEVCNTR1 (*Performance Monitors Event Count Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMEVCNTR1 (*Performance Monitors Event Count Register 1*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmevcntr1(pub u32);
impl SysReg for Pmevcntr1 {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 8;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Pmevcntr1 {}
impl Pmevcntr1 {
    #[inline]
    /// Reads PMEVCNTR1 (*Performance Monitors Event Count Register 1*)
    pub fn read() -> Pmevcntr1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Pmevcntr1 {}
impl Pmevcntr1 {
    #[inline]
    /// Writes PMEVCNTR1 (*Performance Monitors Event Count Register 1*)
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
