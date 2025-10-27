//! Code for managing HPRLAR6 (*Hyp Protection Region Limit Address Register 6*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR6 (*Hyp Protection Region Limit Address Register 6*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar6(pub u32);

impl SysReg for Hprlar6 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 11;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Hprlar6 {}

impl Hprlar6 {
    #[inline]
    /// Reads HPRLAR6 (*Hyp Protection Region Limit Address Register 6*)
    pub fn read() -> Hprlar6 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprlar6 {}

impl Hprlar6 {
    #[inline]
    /// Writes HPRLAR6 (*Hyp Protection Region Limit Address Register 6*)
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
