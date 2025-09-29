//! Code for managing PRLAR15 (*Protection Region Limit Address Register 15*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR15 (*Protection Region Limit Address Register 15*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar15(pub u32);
impl SysReg for Prlar15 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 15;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Prlar15 {}
impl Prlar15 {
    #[inline]
    /// Reads PRLAR15 (*Protection Region Limit Address Register 15*)
    pub fn read() -> Prlar15 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prlar15 {}
impl Prlar15 {
    #[inline]
    /// Writes PRLAR15 (*Protection Region Limit Address Register 15*)
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
