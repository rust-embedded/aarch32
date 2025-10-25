//! Code for managing HPRLAR3 (*Hyp Protection Region Limit Address Register 3*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR3 (*Hyp Protection Region Limit Address Register 3*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar3(pub u32);
impl SysReg for Hprlar3 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 9;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Hprlar3 {}
impl Hprlar3 {
    #[inline]
    /// Reads HPRLAR3 (*Hyp Protection Region Limit Address Register 3*)
    pub fn read() -> Hprlar3 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprlar3 {}
impl Hprlar3 {
    #[inline]
    /// Writes HPRLAR3 (*Hyp Protection Region Limit Address Register 3*)
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
