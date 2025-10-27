//! Code for managing PMUSERENR (*Performance Monitors User Enable Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMUSERENR (*Performance Monitors User Enable Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmuserenr(pub u32);

impl SysReg for Pmuserenr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Pmuserenr {}

impl Pmuserenr {
    #[inline]
    /// Reads PMUSERENR (*Performance Monitors User Enable Register*)
    pub fn read() -> Pmuserenr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmuserenr {}

impl Pmuserenr {
    #[inline]
    /// Writes PMUSERENR (*Performance Monitors User Enable Register*)
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
