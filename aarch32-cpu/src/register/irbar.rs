//! Code for managing IRBAR (*Instruction Region Base Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IRBAR (*Instruction Region Base Address Register*)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Irbar(pub u32);

impl SysReg for Irbar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Irbar {}

impl Irbar {
    #[inline]
    /// Reads IRBAR (*Instruction Region Base Address Register*)
    ///
    /// Set RGNR to control which region this reads.
    pub fn read() -> Irbar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Irbar {}

impl Irbar {
    #[inline]
    /// Writes IRBAR (*Instruction Region Base Address Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Irbar) {
        unsafe { <Self as SysRegWrite>::write_raw(value.0) }
    }
}
