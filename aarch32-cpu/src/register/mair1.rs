//! Code for managing MAIR1 (*Memory Attribute Indirection Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// MAIR1 (*Memory Attribute Indirection Register 1*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mair1(pub u32);

impl SysReg for Mair1 {
    const CP: u32 = 15;
    const CRN: u32 = 10;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Mair1 {}

impl Mair1 {
    #[inline]
    /// Reads MAIR1 (*Memory Attribute Indirection Register 1*)
    pub fn read() -> Mair1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Mair1 {}

impl Mair1 {
    #[inline]
    /// Writes MAIR1 (*Memory Attribute Indirection Register 1*)
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
