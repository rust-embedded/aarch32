//! Code for managing PRBAR1 (*Protection Region Base Address Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR1 (*Protection Region Base Address Register 1*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar1(pub u32);
impl SysReg for Prbar1 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 8;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Prbar1 {}
impl Prbar1 {
    #[inline]
    /// Reads PRBAR1 (*Protection Region Base Address Register 1*)
    pub fn read() -> Prbar1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar1 {}
impl Prbar1 {
    #[inline]
    /// Writes PRBAR1 (*Protection Region Base Address Register 1*)
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
