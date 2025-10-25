//! Code for managing IFAR (*Instruction Fault Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IFAR (*Instruction Fault Address Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ifar(pub u32);
impl SysReg for Ifar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for Ifar {}
impl Ifar {
    #[inline]
    /// Reads IFAR (*Instruction Fault Address Register*)
    pub fn read() -> Ifar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Ifar {}
impl Ifar {
    #[inline]
    /// Writes IFAR (*Instruction Fault Address Register*)
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
