//! Code for managing HPRBAR7 (*Hyp Protection Region Base Address Register 7*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR7 (*Hyp Protection Region Base Address Register 7*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar7(pub u32);

impl SysReg for Hprbar7 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 11;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Hprbar7 {}

impl Hprbar7 {
    #[inline]
    /// Reads HPRBAR7 (*Hyp Protection Region Base Address Register 7*)
    pub fn read() -> Hprbar7 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprbar7 {}

impl Hprbar7 {
    #[inline]
    /// Writes HPRBAR7 (*Hyp Protection Region Base Address Register 7*)
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
