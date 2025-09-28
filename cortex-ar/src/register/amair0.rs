//! Code for managing AMAIR0 (*Auxiliary Memory Attribute Indirection Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// AMAIR0 (*Auxiliary Memory Attribute Indirection Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Amair0(pub u32);
impl SysReg for Amair0 {
    const CP: u32 = 15;
    const CRN: u32 = 10;
    const OP1: u32 = 0;
    const CRM: u32 = 3;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Amair0 {}
impl Amair0 {
    #[inline]
    /// Reads AMAIR0 (*Auxiliary Memory Attribute Indirection Register 0*)
    pub fn read() -> Amair0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Amair0 {}
impl Amair0 {
    #[inline]
    /// Writes AMAIR0 (*Auxiliary Memory Attribute Indirection Register 0*)
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
