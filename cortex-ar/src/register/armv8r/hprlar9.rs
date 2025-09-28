//! Code for managing HPRLAR9 (*Hyp Protection Region Limit Address Register 9*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR9 (*Hyp Protection Region Limit Address Register 9*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar9(pub u32);
impl SysReg for Hprlar9 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 12;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Hprlar9 {}
impl Hprlar9 {
    #[inline]
    /// Reads HPRLAR9 (*Hyp Protection Region Limit Address Register 9*)
    pub fn read() -> Hprlar9 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprlar9 {}
impl Hprlar9 {
    #[inline]
    /// Writes HPRLAR9 (*Hyp Protection Region Limit Address Register 9*)
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
