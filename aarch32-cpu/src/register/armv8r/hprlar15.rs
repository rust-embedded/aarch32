//! Code for managing HPRLAR15 (*Hyp Protection Region Limit Address Register 15*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR15 (*Hyp Protection Region Limit Address Register 15*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar15(pub u32);

impl SysReg for Hprlar15 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 15;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Hprlar15 {}

impl Hprlar15 {
    #[inline]
    /// Reads HPRLAR15 (*Hyp Protection Region Limit Address Register 15*)
    pub fn read() -> Hprlar15 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprlar15 {}

impl Hprlar15 {
    #[inline]
    /// Writes HPRLAR15 (*Hyp Protection Region Limit Address Register 15*)
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
