//! Code for managing DRBAR (*Data Region Base Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DRBAR (*Data Region Base Address Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Drbar(pub *mut u8);

impl SysReg for Drbar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Drbar {}
impl Drbar {
    #[inline]
    /// Reads DRBAR (*Data Region Base Address Register*)
    ///
    /// Set RGNR to control which region this reads.
    pub fn read() -> Drbar {
        unsafe { Self(<Self as SysRegRead>::read_raw() as *mut u8) }
    }
}

impl crate::register::SysRegWrite for Drbar {}
impl Drbar {
    #[inline]
    /// Writes DRBAR (*Data Region Base Address Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Drbar) {
        unsafe { <Self as SysRegWrite>::write_raw(value.0 as u32) }
    }
}
