//! Code for managing PRBAR11 (*Protection Region Base Address Register 11*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR11 (*Protection Region Base Address Register 11*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar11(pub u32);
impl SysReg for Prbar11 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 13;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Prbar11 {}
impl Prbar11 {
    #[inline]
    /// Reads PRBAR11 (*Protection Region Base Address Register 11*)
    pub fn read() -> Prbar11 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar11 {}
impl Prbar11 {
    #[inline]
    /// Writes PRBAR11 (*Protection Region Base Address Register 11*)
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
