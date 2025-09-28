//! Code for managing HPRLAR10 (*Hyp Protection Region Limit Address Register 10*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR10 (*Hyp Protection Region Limit Address Register 10*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar10(pub u32);
impl SysReg for Hprlar10 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 13;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Hprlar10 {}
impl Hprlar10 {
    #[inline]
    /// Reads HPRLAR10 (*Hyp Protection Region Limit Address Register 10*)
    pub fn read() -> Hprlar10 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprlar10 {}
impl Hprlar10 {
    #[inline]
    /// Writes HPRLAR10 (*Hyp Protection Region Limit Address Register 10*)
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
