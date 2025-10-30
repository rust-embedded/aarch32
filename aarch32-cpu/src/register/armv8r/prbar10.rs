//! Code for managing PRBAR10 (*Protection Region Base Address Register 10*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR10 (*Protection Region Base Address Register 10*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar10(pub u32);

impl SysReg for Prbar10 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 13;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Prbar10 {}

impl Prbar10 {
    #[inline]
    /// Reads PRBAR10 (*Protection Region Base Address Register 10*)
    pub fn read() -> Prbar10 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prbar10 {}

impl Prbar10 {
    #[inline]
    /// Writes PRBAR10 (*Protection Region Base Address Register 10*)
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
