//! Code for managing AMAIR1 (*Auxiliary Memory Attribute Indirection Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// AMAIR1 (*Auxiliary Memory Attribute Indirection Register 1*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Amair1(pub u32);
impl SysReg for Amair1 {
    const CP: u32 = 15;
    const CRN: u32 = 10;
    const OP1: u32 = 0;
    const CRM: u32 = 3;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Amair1 {}
impl Amair1 {
    #[inline]
    /// Reads AMAIR1 (*Auxiliary Memory Attribute Indirection Register 1*)
    pub fn read() -> Amair1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Amair1 {}
impl Amair1 {
    #[inline]
    /// Writes AMAIR1 (*Auxiliary Memory Attribute Indirection Register 1*)
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
