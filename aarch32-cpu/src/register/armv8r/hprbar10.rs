//! Code for managing HPRBAR10 (*Hyp Protection Region Base Address Register 10*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR10 (*Hyp Protection Region Base Address Register 10*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar10(pub u32);

impl SysReg for Hprbar10 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 13;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Hprbar10 {}

impl Hprbar10 {
    #[inline]
    /// Reads HPRBAR10 (*Hyp Protection Region Base Address Register 10*)
    pub fn read() -> Hprbar10 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprbar10 {}

impl Hprbar10 {
    #[inline]
    /// Writes HPRBAR10 (*Hyp Protection Region Base Address Register 10*)
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
