//! Code for managing PRLAR11 (*Protection Region Limit Address Register 11*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR11 (*Protection Region Limit Address Register 11*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar11(pub u32);

impl SysReg for Prlar11 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 13;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Prlar11 {}

impl Prlar11 {
    #[inline]
    /// Reads PRLAR11 (*Protection Region Limit Address Register 11*)
    pub fn read() -> Prlar11 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prlar11 {}

impl Prlar11 {
    #[inline]
    /// Writes PRLAR11 (*Protection Region Limit Address Register 11*)
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
