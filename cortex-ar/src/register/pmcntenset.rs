//! Code for managing PMCNTENSET (*Performance Monitors Count Enable Set Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMCNTENSET (*Performance Monitors Count Enable Set Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmcntenset(pub u32);
impl SysReg for Pmcntenset {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Pmcntenset {}
impl Pmcntenset {
    #[inline]
    /// Reads PMCNTENSET (*Performance Monitors Count Enable Set Register*)
    pub fn read() -> Pmcntenset {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Pmcntenset {}
impl Pmcntenset {
    #[inline]
    /// Writes PMCNTENSET (*Performance Monitors Count Enable Set Register*)
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
