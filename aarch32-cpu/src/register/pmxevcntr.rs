//! Code for managing PMXEVCNTR (*Performance Monitors Selected Event Count Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMXEVCNTR (*Performance Monitors Selected Event Count Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmxevcntr(pub u32);

impl SysReg for Pmxevcntr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 13;
    const OP2: u32 = 2;
}

impl crate::register::SysRegRead for Pmxevcntr {}

impl Pmxevcntr {
    #[inline]
    /// Reads PMXEVCNTR (*Performance Monitors Selected Event Count Register*)
    pub fn read() -> Pmxevcntr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmxevcntr {}

impl Pmxevcntr {
    #[inline]
    /// Writes PMXEVCNTR (*Performance Monitors Selected Event Count Register*)
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
