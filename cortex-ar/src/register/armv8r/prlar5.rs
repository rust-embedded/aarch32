//! Code for managing PRLAR5 (*Protection Region Limit Address Register 5*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR5 (*Protection Region Limit Address Register 5*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar5(pub u32);
impl SysReg for Prlar5 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 10;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Prlar5 {}
impl Prlar5 {
    #[inline]
    /// Reads PRLAR5 (*Protection Region Limit Address Register 5*)
    pub fn read() -> Prlar5 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prlar5 {}
impl Prlar5 {
    #[inline]
    /// Writes PRLAR5 (*Protection Region Limit Address Register 5*)
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
