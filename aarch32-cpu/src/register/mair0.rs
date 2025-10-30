//! Code for managing MAIR0 (*Memory Attribute Indirection Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// MAIR0 (*Memory Attribute Indirection Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mair0(pub u32);

impl SysReg for Mair0 {
    const CP: u32 = 15;
    const CRN: u32 = 10;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Mair0 {}

impl Mair0 {
    #[inline]
    /// Reads MAIR0 (*Memory Attribute Indirection Register 0*)
    pub fn read() -> Mair0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Mair0 {}

impl Mair0 {
    #[inline]
    /// Writes MAIR0 (*Memory Attribute Indirection Register 0*)
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
