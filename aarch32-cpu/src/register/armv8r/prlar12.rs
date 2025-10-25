//! Code for managing PRLAR12 (*Protection Region Limit Address Register 12*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR12 (*Protection Region Limit Address Register 12*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar12(pub u32);
impl SysReg for Prlar12 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Prlar12 {}
impl Prlar12 {
    #[inline]
    /// Reads PRLAR12 (*Protection Region Limit Address Register 12*)
    pub fn read() -> Prlar12 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prlar12 {}
impl Prlar12 {
    #[inline]
    /// Writes PRLAR12 (*Protection Region Limit Address Register 12*)
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
