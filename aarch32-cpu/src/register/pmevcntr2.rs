//! Code for managing PMEVCNTR2 (*Performance Monitors Event Count Register 2 *)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMEVCNTR2 (*Performance Monitors Event Count Register 2 *)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmevcntr2(pub u32);

impl SysReg for Pmevcntr2 {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 8;
    const OP2: u32 = 2;
}

impl crate::register::SysRegRead for Pmevcntr2 {}

impl Pmevcntr2 {
    #[inline]
    /// Reads PMEVCNTR2 (*Performance Monitors Event Count Register 2 *)
    pub fn read() -> Pmevcntr2 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmevcntr2 {}

impl Pmevcntr2 {
    #[inline]
    /// Writes PMEVCNTR2 (*Performance Monitors Event Count Register 2 *)
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
