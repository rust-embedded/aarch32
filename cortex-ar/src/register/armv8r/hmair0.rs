//! Code for managing HMAIR0 (*Hyp Memory Attribute Indirection Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HMAIR0 (*Hyp Memory Attribute Indirection Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hmair0(pub u32);
impl SysReg for Hmair0 {
    const CP: u32 = 15;
    const CRN: u32 = 10;
    const OP1: u32 = 4;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Hmair0 {}
impl Hmair0 {
    #[inline]
    /// Reads HMAIR0 (*Hyp Memory Attribute Indirection Register 0*)
    pub fn read() -> Hmair0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hmair0 {}
impl Hmair0 {
    #[inline]
    /// Writes HMAIR0 (*Hyp Memory Attribute Indirection Register 0*)
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
