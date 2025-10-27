//! Code for managing HPRLAR0 (*Hyp Protection Region Limit Address Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR0 (*Hyp Protection Region Limit Address Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar0(pub u32);

impl SysReg for Hprlar0 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 8;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Hprlar0 {}

impl Hprlar0 {
    #[inline]
    /// Reads HPRLAR0 (*Hyp Protection Region Limit Address Register 0*)
    pub fn read() -> Hprlar0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprlar0 {}

impl Hprlar0 {
    #[inline]
    /// Writes HPRLAR0 (*Hyp Protection Region Limit Address Register 0*)
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
