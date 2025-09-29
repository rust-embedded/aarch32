//! Code for managing PRLAR13 (*Protection Region Limit Address Register 13*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR13 (*Protection Region Limit Address Register 13*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar13(pub u32);
impl SysReg for Prlar13 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Prlar13 {}
impl Prlar13 {
    #[inline]
    /// Reads PRLAR13 (*Protection Region Limit Address Register 13*)
    pub fn read() -> Prlar13 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prlar13 {}
impl Prlar13 {
    #[inline]
    /// Writes PRLAR13 (*Protection Region Limit Address Register 13*)
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
