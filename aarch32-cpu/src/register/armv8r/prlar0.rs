//! Code for managing PRLAR0 (*Protection Region Limit Address Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR0 (*Protection Region Limit Address Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar0(pub u32);

impl SysReg for Prlar0 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 8;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Prlar0 {}

impl Prlar0 {
    #[inline]
    /// Reads PRLAR0 (*Protection Region Limit Address Register 0*)
    pub fn read() -> Prlar0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prlar0 {}

impl Prlar0 {
    #[inline]
    /// Writes PRLAR0 (*Protection Region Limit Address Register 0*)
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
