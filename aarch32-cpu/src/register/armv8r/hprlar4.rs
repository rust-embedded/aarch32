//! Code for managing HPRLAR4 (*Hyp Protection Region Limit Address Register 4*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR4 (*Hyp Protection Region Limit Address Register 4*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar4(pub u32);
impl SysReg for Hprlar4 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 10;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Hprlar4 {}
impl Hprlar4 {
    #[inline]
    /// Reads HPRLAR4 (*Hyp Protection Region Limit Address Register 4*)
    pub fn read() -> Hprlar4 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprlar4 {}
impl Hprlar4 {
    #[inline]
    /// Writes HPRLAR4 (*Hyp Protection Region Limit Address Register 4*)
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
