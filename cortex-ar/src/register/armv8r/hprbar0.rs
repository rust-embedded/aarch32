//! Code for managing HPRBAR0 (*Hyp Protection Region Base Address Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR0 (*Hyp Protection Region Base Address Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar0(pub u32);
impl SysReg for Hprbar0 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 8;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Hprbar0 {}
impl Hprbar0 {
    #[inline]
    /// Reads HPRBAR0 (*Hyp Protection Region Base Address Register 0*)
    pub fn read() -> Hprbar0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprbar0 {}
impl Hprbar0 {
    #[inline]
    /// Writes HPRBAR0 (*Hyp Protection Region Base Address Register 0*)
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
