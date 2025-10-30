//! Code for managing PRBAR7 (*Protection Region Base Address Register 7*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR7 (*Protection Region Base Address Register 7*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar7(pub u32);

impl SysReg for Prbar7 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 11;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Prbar7 {}

impl Prbar7 {
    #[inline]
    /// Reads PRBAR7 (*Protection Region Base Address Register 7*)
    pub fn read() -> Prbar7 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prbar7 {}

impl Prbar7 {
    #[inline]
    /// Writes PRBAR7 (*Protection Region Base Address Register 7*)
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
