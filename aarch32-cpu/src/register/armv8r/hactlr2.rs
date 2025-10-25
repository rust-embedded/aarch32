//! Code for managing HACTLR2 (*Hyp Auxiliary Control Register 2*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HACTLR2 (*Hyp Auxiliary Control Register 2*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hactlr2(pub u32);
impl SysReg for Hactlr2 {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 3;
}
impl crate::register::SysRegRead for Hactlr2 {}
impl Hactlr2 {
    #[inline]
    /// Reads HACTLR2 (*Hyp Auxiliary Control Register 2*)
    pub fn read() -> Hactlr2 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hactlr2 {}
impl Hactlr2 {
    #[inline]
    /// Writes HACTLR2 (*Hyp Auxiliary Control Register 2*)
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
