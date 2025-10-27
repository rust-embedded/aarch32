//! Code for managing ACTLR (*Auxiliary Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// ACTLR (*Auxiliary Control Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Actlr(pub u32);

impl SysReg for Actlr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Actlr {}

impl Actlr {
    #[inline]
    /// Reads ACTLR (*Auxiliary Control Register*)
    pub fn read() -> Actlr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Actlr {}

impl Actlr {
    #[inline]
    /// Writes ACTLR (*Auxiliary Control Register*)
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
