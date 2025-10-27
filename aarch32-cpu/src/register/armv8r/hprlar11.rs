//! Code for managing HPRLAR11 (*Hyp Protection Region Limit Address Register 11*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR11 (*Hyp Protection Region Limit Address Register 11*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar11(pub u32);

impl SysReg for Hprlar11 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 13;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Hprlar11 {}

impl Hprlar11 {
    #[inline]
    /// Reads HPRLAR11 (*Hyp Protection Region Limit Address Register 11*)
    pub fn read() -> Hprlar11 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprlar11 {}

impl Hprlar11 {
    #[inline]
    /// Writes HPRLAR11 (*Hyp Protection Region Limit Address Register 11*)
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
