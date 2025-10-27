//! Code for managing HAMAIR0 (*Hyp Auxiliary Memory Attribute Indirection Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HAMAIR0 (*Hyp Auxiliary Memory Attribute Indirection Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hamair0(pub u32);

impl SysReg for Hamair0 {
    const CP: u32 = 15;
    const CRN: u32 = 10;
    const OP1: u32 = 4;
    const CRM: u32 = 3;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Hamair0 {}

impl Hamair0 {
    #[inline]
    /// Reads HAMAIR0 (*Hyp Auxiliary Memory Attribute Indirection Register 0*)
    pub fn read() -> Hamair0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hamair0 {}

impl Hamair0 {
    #[inline]
    /// Writes HAMAIR0 (*Hyp Auxiliary Memory Attribute Indirection Register 0*)
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
