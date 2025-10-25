//! Code for managing PRBAR6 (*Protection Region Base Address Register 6*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR6 (*Protection Region Base Address Register 6*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar6(pub u32);
impl SysReg for Prbar6 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 11;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Prbar6 {}
impl Prbar6 {
    #[inline]
    /// Reads PRBAR6 (*Protection Region Base Address Register 6*)
    pub fn read() -> Prbar6 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar6 {}
impl Prbar6 {
    #[inline]
    /// Writes PRBAR6 (*Protection Region Base Address Register 6*)
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
