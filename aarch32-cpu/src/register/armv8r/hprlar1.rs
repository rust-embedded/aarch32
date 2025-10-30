//! Code for managing HPRLAR1 (*Hyp Protection Region Limit Address Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR1 (*Hyp Protection Region Limit Address Register 1*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprlar1(pub u32);

impl SysReg for Hprlar1 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 8;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Hprlar1 {}

impl Hprlar1 {
    #[inline]
    /// Reads HPRLAR1 (*Hyp Protection Region Limit Address Register 1*)
    pub fn read() -> Hprlar1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprlar1 {}

impl Hprlar1 {
    #[inline]
    /// Writes HPRLAR1 (*Hyp Protection Region Limit Address Register 1*)
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
