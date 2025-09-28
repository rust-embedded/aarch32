//! Code for managing PRBAR0 (*Protection Region Base Address Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR0 (*Protection Region Base Address Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar0(pub u32);
impl SysReg for Prbar0 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 8;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Prbar0 {}
impl Prbar0 {
    #[inline]
    /// Reads PRBAR0 (*Protection Region Base Address Register 0*)
    pub fn read() -> Prbar0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar0 {}
impl Prbar0 {
    #[inline]
    /// Writes PRBAR0 (*Protection Region Base Address Register 0*)
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
