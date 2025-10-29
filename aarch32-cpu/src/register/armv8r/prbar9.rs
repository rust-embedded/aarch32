//! Code for managing PRBAR9 (*Protection Region Base Address Register 9*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR9 (*Protection Region Base Address Register 9*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar9(pub u32);
impl SysReg for Prbar9 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Prbar9 {}
impl Prbar9 {
    #[inline]
    /// Reads PRBAR9 (*Protection Region Base Address Register 9*)
    pub fn read() -> Prbar9 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar9 {}
impl Prbar9 {
    #[inline]
    /// Writes PRBAR9 (*Protection Region Base Address Register 9*)
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
