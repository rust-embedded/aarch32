//! Code for managing PRLAR15 (*Protection Region Limit Address Register 15*)

use crate::register::{Prlar, SysReg, SysRegRead, SysRegWrite};

/// PRLAR15 (*Protection Region Limit Address Register 15*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar15(pub u32);

impl SysReg for Prlar15 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 15;
    const OP2: u32 = 5;
}

impl SysRegRead for Prlar15 {}

impl Prlar15 {
    #[inline]
    /// Reads PRLAR15 (*Protection Region Limit Address Register 15*)
    pub fn read() -> Prlar {
        unsafe { Prlar::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for Prlar15 {}

impl Prlar15 {
    #[inline]
    /// Writes PRLAR15 (*Protection Region Limit Address Register 15*)
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
