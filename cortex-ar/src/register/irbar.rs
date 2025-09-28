//! Code for managing IRBAR (*Instruction Region Base Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IRBAR (*Instruction Region Base Address Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Irbar(pub *mut u8);
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
        unsafe { Self(<Self as SysRegRead>::read_raw() as *mut u8) }
    }
}

impl crate::register::SysRegWrite for Irbar {}
impl Irbar {
    #[inline]
    /// Writes IRBAR (*Instruction Region Base Address Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Irbar) {
        unsafe { <Self as SysRegWrite>::write_raw(value.0 as u32) }
    }
}
