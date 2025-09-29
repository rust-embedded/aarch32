//! Code for managing PRLAR10 (*Protection Region Limit Address Register 10*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR10 (*Protection Region Limit Address Register 10*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar10(pub u32);
impl SysReg for Prlar10 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 13;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Prlar10 {}
impl Prlar10 {
    #[inline]
    /// Reads PRLAR10 (*Protection Region Limit Address Register 10*)
    pub fn read() -> Prlar10 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prlar10 {}
impl Prlar10 {
    #[inline]
    /// Writes PRLAR10 (*Protection Region Limit Address Register 10*)
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
