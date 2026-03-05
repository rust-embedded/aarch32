//! Code for managing HAMAIR1 (*Hyp Auxiliary Memory Attribute Indirection Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HAMAIR1 (*Hyp Auxiliary Memory Attribute Indirection Register 1*)
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hamair1(pub u32);

impl SysReg for Hamair1 {
    const CP: u32 = 15;
    const CRN: u32 = 10;
    const OP1: u32 = 4;
    const CRM: u32 = 3;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Hamair1 {}

impl Hamair1 {
    #[inline]
    /// Reads HAMAIR1 (*Hyp Auxiliary Memory Attribute Indirection Register 1*)
    pub fn read() -> Hamair1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hamair1 {}

impl Hamair1 {
    #[inline]
    /// Writes HAMAIR1 (*Hyp Auxiliary Memory Attribute Indirection Register 1*)
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
