//! Code for managing PRLAR1 (*Protection Region Limit Address Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR1 (*Protection Region Limit Address Register 1*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar1(pub u32);

impl SysReg for Prlar1 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 8;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Prlar1 {}

impl Prlar1 {
    #[inline]
    /// Reads PRLAR1 (*Protection Region Limit Address Register 1*)
    pub fn read() -> Prlar1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prlar1 {}

impl Prlar1 {
    #[inline]
    /// Writes PRLAR1 (*Protection Region Limit Address Register 1*)
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
