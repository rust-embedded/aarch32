//! Code for managing HPRLAR5 (*Hyp Protection Region Limit Address Register 5*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR5 (*Hyp Protection Region Limit Address Register 5*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar5(pub u32);

impl SysReg for Hprlar5 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 10;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Hprlar5 {}

impl Hprlar5 {
    #[inline]
    /// Reads HPRLAR5 (*Hyp Protection Region Limit Address Register 5*)
    pub fn read() -> Hprlar5 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprlar5 {}

impl Hprlar5 {
    #[inline]
    /// Writes HPRLAR5 (*Hyp Protection Region Limit Address Register 5*)
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
