//! Code for managing HPRLAR14 (*Hyp Protection Region Limit Address Register 14*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR14 (*Hyp Protection Region Limit Address Register 14*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar14(pub u32);

impl SysReg for Hprlar14 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 15;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Hprlar14 {}

impl Hprlar14 {
    #[inline]
    /// Reads HPRLAR14 (*Hyp Protection Region Limit Address Register 14*)
    pub fn read() -> Hprlar14 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprlar14 {}

impl Hprlar14 {
    #[inline]
    /// Writes HPRLAR14 (*Hyp Protection Region Limit Address Register 14*)
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
