//! Code for managing IRACR (*Instruction Region Access Control Register*)

use arbitrary_int::u3;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IRACR (*Instruction Region Access Control Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Iracr {
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

impl SysReg for Iracr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Iracr {}

impl Iracr {
    #[inline]
    /// Reads IRACR (*Instruction Region Access Control Register*)
    ///
    /// Set RGNR to control which region this reads.
    pub fn read() -> Iracr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Iracr {}

impl Iracr {
    #[inline]
    /// Writes IRACR (*Instruction Region Access Control Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Iracr) {
        unsafe { <Self as SysRegWrite>::write_raw(value.raw_value()) }
    }
}
