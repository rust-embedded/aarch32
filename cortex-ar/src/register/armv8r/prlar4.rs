//! Code for managing PRLAR4 (*Protection Region Limit Address Register 4*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR4 (*Protection Region Limit Address Register 4*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar4(pub u32);
impl SysReg for Prlar4 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 10;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Prlar4 {}
impl Prlar4 {
    #[inline]
    /// Reads PRLAR4 (*Protection Region Limit Address Register 4*)
    pub fn read() -> Prlar4 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prlar4 {}
impl Prlar4 {
    #[inline]
    /// Writes PRLAR4 (*Protection Region Limit Address Register 4*)
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
