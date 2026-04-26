//! Code for managing PRLAR8 (*Protection Region Limit Address Register 8*)

use crate::register::{Prlar, SysReg, SysRegRead, SysRegWrite};

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

impl SysRegRead for Prlar8 {}

impl Prlar8 {
    #[inline]
    /// Reads PRLAR8 (*Protection Region Limit Address Register 8*)
    pub fn read() -> Prlar {
        unsafe { Prlar::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for Prlar8 {}

impl Prlar8 {
    #[inline]
    /// Writes PRLAR8 (*Protection Region Limit Address Register 8*)
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
