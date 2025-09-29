//! Code for managing PRLAR3 (*Protection Region Limit Address Register 3*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR3 (*Protection Region Limit Address Register 3*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar3(pub u32);
impl SysReg for Prlar3 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 9;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Prlar3 {}
impl Prlar3 {
    #[inline]
    /// Reads PRLAR3 (*Protection Region Limit Address Register 3*)
    pub fn read() -> Prlar3 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prlar3 {}
impl Prlar3 {
    #[inline]
    /// Writes PRLAR3 (*Protection Region Limit Address Register 3*)
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
