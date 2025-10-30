//! Code for managing PMEVCNTR0 (*Performance Monitors Event Count Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMEVCNTR0 (*Performance Monitors Event Count Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmevcntr0(pub u32);

impl SysReg for Pmevcntr0 {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 8;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Pmevcntr0 {}

impl Pmevcntr0 {
    #[inline]
    /// Reads PMEVCNTR0 (*Performance Monitors Event Count Register 0*)
    pub fn read() -> Pmevcntr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmevcntr0 {}

impl Pmevcntr0 {
    #[inline]
    /// Writes PMEVCNTR0 (*Performance Monitors Event Count Register 0*)
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
