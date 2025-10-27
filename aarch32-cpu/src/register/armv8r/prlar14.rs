//! Code for managing PRLAR14 (*Protection Region Limit Address Register 14*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR14 (*Protection Region Limit Address Register 14*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar14(pub u32);

impl SysReg for Prlar14 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 15;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Prlar14 {}

impl Prlar14 {
    #[inline]
    /// Reads PRLAR14 (*Protection Region Limit Address Register 14*)
    pub fn read() -> Prlar14 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prlar14 {}

impl Prlar14 {
    #[inline]
    /// Writes PRLAR14 (*Protection Region Limit Address Register 14*)
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
