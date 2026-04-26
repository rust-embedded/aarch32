//! Code for managing MAIR0 (*Memory Attribute Indirection Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// MAIR (*Memory Attribute Indirection Register*) value for MAIR0 or MAIR1
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mair {
    /// The four MAIR attributes held in a 32-bit MAIR register
    #[bits(0..=7, rw)]
    attrs: [u8; 4],
}

/// MAIR0 (*Memory Attribute Indirection Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mair0;

impl SysReg for Mair0 {
    const CP: u32 = 15;
    const CRN: u32 = 10;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}

impl SysRegRead for Mair0 {}

impl Mair0 {
    #[inline]
    /// Reads MAIR0 (*Memory Attribute Indirection Register 0*)
    pub fn read() -> Mair {
        unsafe { Mair::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for Mair0 {}

impl Mair0 {
    #[inline]
    /// Writes MAIR0 (*Memory Attribute Indirection Register 0*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Mair) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
