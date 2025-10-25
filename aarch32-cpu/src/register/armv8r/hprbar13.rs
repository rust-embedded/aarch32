//! Code for managing HPRBAR13 (*Hyp Protection Region Base Address Register 13*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR13 (*Hyp Protection Region Base Address Register 13*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar13(pub u32);
impl SysReg for Hprbar13 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 14;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Hprbar13 {}
impl Hprbar13 {
    #[inline]
    /// Reads HPRBAR13 (*Hyp Protection Region Base Address Register 13*)
    pub fn read() -> Hprbar13 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprbar13 {}
impl Hprbar13 {
    #[inline]
    /// Writes HPRBAR13 (*Hyp Protection Region Base Address Register 13*)
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
