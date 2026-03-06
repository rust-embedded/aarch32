//! Code for managing PRLAR13 (*Protection Region Limit Address Register 13*)

use crate::register::{Prlar, SysReg, SysRegRead, SysRegWrite};

/// PRLAR13 (*Protection Region Limit Address Register 13*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar13(pub u32);

impl SysReg for Prlar13 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Prlar13 {}

impl Prlar13 {
    #[inline]
    /// Reads PRLAR13 (*Protection Region Limit Address Register 13*)
    pub fn read() -> Prlar {
        unsafe { Prlar::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prlar13 {}

impl Prlar13 {
    #[inline]
    /// Writes PRLAR13 (*Protection Region Limit Address Register 13*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Prlar) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
