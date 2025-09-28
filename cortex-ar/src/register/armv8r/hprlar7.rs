//! Code for managing HPRLAR7 (*Hyp Protection Region Limit Address Register 7*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR7 (*Hyp Protection Region Limit Address Register 7*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar7(pub u32);
impl SysReg for Hprlar7 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 11;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Hprlar7 {}
impl Hprlar7 {
    #[inline]
    /// Reads HPRLAR7 (*Hyp Protection Region Limit Address Register 7*)
    pub fn read() -> Hprlar7 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprlar7 {}
impl Hprlar7 {
    #[inline]
    /// Writes HPRLAR7 (*Hyp Protection Region Limit Address Register 7*)
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
