//! Code for managing HPRBAR8 (*Hyp Protection Region Base Address Register 8*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR8 (*Hyp Protection Region Base Address Register 8*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar8(pub u32);

impl SysReg for Hprbar8 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 12;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Hprbar8 {}

impl Hprbar8 {
    #[inline]
    /// Reads HPRBAR8 (*Hyp Protection Region Base Address Register 8*)
    pub fn read() -> Hprbar8 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprbar8 {}

impl Hprbar8 {
    #[inline]
    /// Writes HPRBAR8 (*Hyp Protection Region Base Address Register 8*)
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
