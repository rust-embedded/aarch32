//! Code for managing PMINTENCLR (*Performance Monitors Interrupt Enable Clear Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMINTENCLR (*Performance Monitors Interrupt Enable Clear Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmintenclr(pub u32);
impl SysReg for Pmintenclr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for Pmintenclr {}
impl Pmintenclr {
    #[inline]
    /// Reads PMINTENCLR (*Performance Monitors Interrupt Enable Clear Register*)
    pub fn read() -> Pmintenclr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Pmintenclr {}
impl Pmintenclr {
    #[inline]
    /// Writes PMINTENCLR (*Performance Monitors Interrupt Enable Clear Register*)
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
