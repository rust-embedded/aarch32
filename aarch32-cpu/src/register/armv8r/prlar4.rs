//! Code for managing PRLAR4 (*Protection Region Limit Address Register 4*)

use crate::register::{Prlar, SysReg, SysRegRead, SysRegWrite};

/// PRLAR4 (*Protection Region Limit Address Register 4*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prlar4(pub u32);

impl SysReg for Prlar4 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 10;
    const OP2: u32 = 1;
}

impl SysRegRead for Prlar4 {}

impl Prlar4 {
    #[inline]
    /// Reads PRLAR4 (*Protection Region Limit Address Register 4*)
    pub fn read() -> Prlar {
        unsafe { Prlar::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for Prlar4 {}

impl Prlar4 {
    #[inline]
    /// Writes PRLAR4 (*Protection Region Limit Address Register 4*)
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
