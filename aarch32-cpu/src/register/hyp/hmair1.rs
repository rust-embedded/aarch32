//! Code for managing HMAIR1 (*Hyp Memory Attribute Indirection Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HMAIR1 (*Hyp Memory Attribute Indirection Register 1*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hmair1(pub u32);

impl SysReg for Hmair1 {
    const CP: u32 = 15;
    const CRN: u32 = 10;
    const OP1: u32 = 4;
    const CRM: u32 = 2;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Hmair1 {}

impl Hmair1 {
    #[inline]
    /// Reads HMAIR1 (*Hyp Memory Attribute Indirection Register 1*)
    pub fn read() -> Hmair1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hmair1 {}

impl Hmair1 {
    #[inline]
    /// Writes HMAIR1 (*Hyp Memory Attribute Indirection Register 1*)
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
