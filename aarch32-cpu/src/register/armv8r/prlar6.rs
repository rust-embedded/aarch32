//! Code for managing PRLAR6 (*Protection Region Limit Address Register 6*)

use crate::register::{Prlar, SysReg, SysRegRead, SysRegWrite};

/// PRLAR6 (*Protection Region Limit Address Register 6*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar6(pub u32);

impl SysReg for Prlar6 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 11;
    const OP2: u32 = 1;
}

impl SysRegRead for Prlar6 {}

impl Prlar6 {
    #[inline]
    /// Reads PRLAR6 (*Protection Region Limit Address Register 6*)
    pub fn read() -> Prlar {
        unsafe { Prlar::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for Prlar6 {}

impl Prlar6 {
    #[inline]
    /// Writes PRLAR6 (*Protection Region Limit Address Register 6*)
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
