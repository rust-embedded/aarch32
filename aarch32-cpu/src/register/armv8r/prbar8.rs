//! Code for managing PRBAR8 (*Protection Region Base Address Register 8*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR8 (*Protection Region Base Address Register 8*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar8(pub u32);

impl SysReg for Prbar8 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Prbar8 {}

impl Prbar8 {
    #[inline]
    /// Reads PRBAR8 (*Protection Region Base Address Register 8*)
    pub fn read() -> Prbar8 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prbar8 {}

impl Prbar8 {
    #[inline]
    /// Writes PRBAR8 (*Protection Region Base Address Register 8*)
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
