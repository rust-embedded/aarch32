//! Code for managing HPRLAR13 (*Hyp Protection Region Limit Address Register 13*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR13 (*Hyp Protection Region Limit Address Register 13*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar13(pub u32);
impl SysReg for Hprlar13 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 14;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Hprlar13 {}
impl Hprlar13 {
    #[inline]
    /// Reads HPRLAR13 (*Hyp Protection Region Limit Address Register 13*)
    pub fn read() -> Hprlar13 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprlar13 {}
impl Hprlar13 {
    #[inline]
    /// Writes HPRLAR13 (*Hyp Protection Region Limit Address Register 13*)
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
