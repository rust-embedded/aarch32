//! Code for managing HPRLAR2 (*Hyp Protection Region Limit Address Register 2*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR2 (*Hyp Protection Region Limit Address Register 2*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar2(pub u32);
impl SysReg for Hprlar2 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 9;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Hprlar2 {}
impl Hprlar2 {
    #[inline]
    /// Reads HPRLAR2 (*Hyp Protection Region Limit Address Register 2*)
    pub fn read() -> Hprlar2 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprlar2 {}
impl Hprlar2 {
    #[inline]
    /// Writes HPRLAR2 (*Hyp Protection Region Limit Address Register 2*)
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
