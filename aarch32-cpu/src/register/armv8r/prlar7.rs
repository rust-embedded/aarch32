//! Code for managing PRLAR7 (*Protection Region Limit Address Register 7*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR7 (*Protection Region Limit Address Register 7*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar7(pub u32);

impl SysReg for Prlar7 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 11;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Prlar7 {}

impl Prlar7 {
    #[inline]
    /// Reads PRLAR7 (*Protection Region Limit Address Register 7*)
    pub fn read() -> Prlar7 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prlar7 {}

impl Prlar7 {
    #[inline]
    /// Writes PRLAR7 (*Protection Region Limit Address Register 7*)
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
