//! Code for managing RGNR (*MPU Region Number Register*)

use crate::register::{SysReg, SysRegRead};

use super::SysRegWrite;

/// RGNR (*MPU Region Number Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgnr(pub u32);
impl SysReg for Rgnr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Rgnr {}
impl Rgnr {
    #[inline]
    /// Reads RGNR (*MPU Region Number Register*)
    pub fn read() -> Rgnr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Rgnr {}
impl Rgnr {
    #[inline]
    /// Writes RGNR (*MPU Region Number Register*)
    ///
    /// This affects what DRACR, IRACR, IRSR and DRSR give you.
    pub fn write(value: Rgnr) {
        unsafe { <Self as SysRegWrite>::write_raw(value.0) }
    }
}
