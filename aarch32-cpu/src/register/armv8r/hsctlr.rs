//! Code for managing HSCTLR (*Hyp System Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HSCTLR (*Hyp System Control Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct Hsctlr {
    /// T32 Exception Enable. Controls whether exceptions to EL2 are taken to A32 or T32 state
    #[bits(30..=30, rw)]
    te: bool,
    /// Exception Endianness. The value of the PSTATE.E bit on entry to Hyp mode
    #[bits(25..=25, rw)]
    ee: bool,
    /// Fast Interrupts enable
    #[bits(21..=21, rw)]
    fi: bool,
    /// Write permission implies XN (Execute-never)
    #[bits(19..=19, rw)]
    wxn: bool,
    /// Background Region enable for EL2
    #[bits(17..=17, rw)]
    br: bool,
    /// Instruction access Cacheability control, for accesses at EL2
    #[bits(12..=12, rw)]
    i: bool,
    /// SETEND instruction disable. Disables SETEND instructions at EL2
    #[bits(8..=8, rw)]
    sed: bool,
    /// IT Disable. Disables some uses of IT instructions at EL2
    #[bits(7..=7, rw)]
    itd: bool,
    /// System instruction memory barrier enable
    #[bits(5..=5, rw)]
    cp15ben: bool,
    /// Cacheability control, for data accesses at EL2
    #[bits(2..=2, rw)]
    c: bool,
    /// Alignment check enable
    #[bits(1..=1, rw)]
    a: bool,
    /// MPU enable for the EL2 MPU
    #[bits(0..=0, rw)]
    m: bool,
}

impl SysReg for Hsctlr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Hsctlr {}

impl Hsctlr {
    #[inline]
    /// Reads HSCTLR (*Hyp System Control Register*)
    pub fn read() -> Hsctlr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hsctlr {}

impl Hsctlr {
    #[inline]
    /// Writes HSCTLR (*Hyp System Control Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
    /// Read-modify-write this register
    #[inline]
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut val = Self::read();
        f(&mut val);
        Self::write(val);
    }
}
