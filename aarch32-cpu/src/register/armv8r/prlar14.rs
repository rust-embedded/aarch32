//! Code for managing PRLAR14 (*Protection Region Limit Address Register 14*)

use crate::register::{Prlar, SysReg, SysRegRead, SysRegWrite};

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

impl SysRegRead for Prlar14 {}

impl Prlar14 {
    #[inline]
    /// Reads PRLAR14 (*Protection Region Limit Address Register 14*)
    pub fn read() -> Prlar {
        unsafe { Prlar::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for Prlar14 {}

impl Prlar14 {
    #[inline]
    /// Writes PRLAR14 (*Protection Region Limit Address Register 14*)
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
