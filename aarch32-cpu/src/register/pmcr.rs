//! Code for managing PMCR (*Performance Monitors Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMCR (*Performance Monitors Control Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmcr(pub u32);

impl SysReg for Pmcr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Pmcr {}

impl Pmcr {
    #[inline]
    /// Reads PMCR (*Performance Monitors Control Register*)
    pub fn read() -> Pmcr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmcr {}

impl Pmcr {
    #[inline]
    /// Writes PMCR (*Performance Monitors Control Register*)
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
