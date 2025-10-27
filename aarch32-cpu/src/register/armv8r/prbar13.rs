//! Code for managing PRBAR13 (*Protection Region Base Address Register 13*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR13 (*Protection Region Base Address Register 13*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar13(pub u32);

impl SysReg for Prbar13 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Prbar13 {}

impl Prbar13 {
    #[inline]
    /// Reads PRBAR13 (*Protection Region Base Address Register 13*)
    pub fn read() -> Prbar13 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prbar13 {}

impl Prbar13 {
    #[inline]
    /// Writes PRBAR13 (*Protection Region Base Address Register 13*)
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
