//! Code for managing PMINTENSET (*Performance Monitors Interrupt Enable Set Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMINTENSET (*Performance Monitors Interrupt Enable Set Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmintenset(pub u32);
impl SysReg for Pmintenset {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Pmintenset {}
impl Pmintenset {
    #[inline]
    /// Reads PMINTENSET (*Performance Monitors Interrupt Enable Set Register*)
    pub fn read() -> Pmintenset {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Pmintenset {}
impl Pmintenset {
    #[inline]
    /// Writes PMINTENSET (*Performance Monitors Interrupt Enable Set Register*)
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
