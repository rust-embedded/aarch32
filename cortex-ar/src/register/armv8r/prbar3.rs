//! Code for managing PRBAR3 (*Protection Region Base Address Register 3*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR3 (*Protection Region Base Address Register 3*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar3(pub u32);
impl SysReg for Prbar3 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 9;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Prbar3 {}
impl Prbar3 {
    #[inline]
    /// Reads PRBAR3 (*Protection Region Base Address Register 3*)
    pub fn read() -> Prbar3 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar3 {}
impl Prbar3 {
    #[inline]
    /// Writes PRBAR3 (*Protection Region Base Address Register 3*)
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
