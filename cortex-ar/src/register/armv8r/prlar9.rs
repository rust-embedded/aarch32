//! Code for managing PRLAR9 (*Protection Region Limit Address Register 9*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR9 (*Protection Region Limit Address Register 9*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar9(pub u32);
impl SysReg for Prlar9 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Prlar9 {}
impl Prlar9 {
    #[inline]
    /// Reads PRLAR9 (*Protection Region Limit Address Register 9*)
    pub fn read() -> Prlar9 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prlar9 {}
impl Prlar9 {
    #[inline]
    /// Writes PRLAR9 (*Protection Region Limit Address Register 9*)
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
