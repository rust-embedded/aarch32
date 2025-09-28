//! Code for managing PRBAR12 (*Protection Region Base Address Register 12*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR12 (*Protection Region Base Address Register 12*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar12(pub u32);
impl SysReg for Prbar12 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Prbar12 {}
impl Prbar12 {
    #[inline]
    /// Reads PRBAR12 (*Protection Region Base Address Register 12*)
    pub fn read() -> Prbar12 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar12 {}
impl Prbar12 {
    #[inline]
    /// Writes PRBAR12 (*Protection Region Base Address Register 12*)
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
