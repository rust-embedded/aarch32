//! Code for managing DRACR (*Data Region Access Control Register*)

use arbitrary_int::u3;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DRACR (*Data Region Access Control Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dracr {
    /// Execute Never
    #[bits(12..=12, rw)]
    nx: bool,
    /// Access Permission bits
    #[bits(8..=10, rw)]
    ap: u3,
    /// TEX bits
    #[bits(3..=5, rw)]
    tex: u3,
    /// S bit
    #[bits(2..=2, rw)]
    s: bool,
    /// C bit
    #[bits(1..=1, rw)]
    c: bool,
    /// B bit
    #[bits(0..=0, rw)]
    b: bool,
}

impl SysReg for Dracr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Dracr {}

impl Dracr {
    #[inline]
    /// Reads DRACR (*Data Region Access Control Register*)
    ///
    /// Set RGNR to control which region this reads.
    pub fn read() -> Dracr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Dracr {}

impl Dracr {
    #[inline]
    /// Writes DRACR (*Data Region Access Control Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Dracr) {
        unsafe { <Self as SysRegWrite>::write_raw(value.raw_value()) }
    }
}
