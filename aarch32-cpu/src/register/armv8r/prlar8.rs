//! Code for managing PRLAR8 (*Protection Region Limit Address Register 8*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR8 (*Protection Region Limit Address Register 8*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar8(pub u32);

impl SysReg for Prlar8 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Prlar8 {}

impl Prlar8 {
    #[inline]
    /// Reads PRLAR8 (*Protection Region Limit Address Register 8*)
    pub fn read() -> Prlar8 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prlar8 {}

impl Prlar8 {
    #[inline]
    /// Writes PRLAR8 (*Protection Region Limit Address Register 8*)
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
