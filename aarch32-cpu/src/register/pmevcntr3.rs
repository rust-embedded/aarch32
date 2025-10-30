//! Code for managing PMEVCNTR3 (*Performance Monitors Event Count Register 3*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMEVCNTR3 (*Performance Monitors Event Count Register 3*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmevcntr3(pub u32);

impl SysReg for Pmevcntr3 {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 8;
    const OP2: u32 = 3;
}

impl crate::register::SysRegRead for Pmevcntr3 {}

impl Pmevcntr3 {
    #[inline]
    /// Reads PMEVCNTR3 (*Performance Monitors Event Count Register 3*)
    pub fn read() -> Pmevcntr3 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmevcntr3 {}

impl Pmevcntr3 {
    #[inline]
    /// Writes PMEVCNTR3 (*Performance Monitors Event Count Register 3*)
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
