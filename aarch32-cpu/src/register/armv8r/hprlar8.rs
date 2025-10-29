//! Code for managing HPRLAR8 (*Hyp Protection Region Limit Address Register 8*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR8 (*Hyp Protection Region Limit Address Register 8*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar8(pub u32);
impl SysReg for Hprlar8 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 12;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Hprlar8 {}
impl Hprlar8 {
    #[inline]
    /// Reads HPRLAR8 (*Hyp Protection Region Limit Address Register 8*)
    pub fn read() -> Hprlar8 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprlar8 {}
impl Hprlar8 {
    #[inline]
    /// Writes HPRLAR8 (*Hyp Protection Region Limit Address Register 8*)
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
