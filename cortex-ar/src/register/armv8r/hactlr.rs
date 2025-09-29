//! Code for managing HACTRL (*Hyp Auxiliary Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HACTRL (*Hyp Auxiliary Control Register*)
#[bitbybit::bitfield(u32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hactlr {
    /// Controls access to IMP_TESTR1 at EL0 and EL1
    #[bits(15..=15, rw)]
    testr1: bool,
    /// Controls access to IMP_DCERR0, IMP_DCERR1, IMP_ICERR0, IMP_ICERR1,
    /// IMP_TCMERR0, IMP_TCMERR1, IMP_FLASHERR0, and IMP_FLASHERR1 registers
    #[bits(13..=13, rw)]
    err: bool,
    /// Controls access to IMP_INTMONR at EL1
    #[bits(12..=12, rw)]
    intmonr: bool,
    /// Controls access to IMP_BUSTIMEOUTR at EL1
    #[bits(10..=10, rw)]
    bustimeoutr: bool,
    /// Controls access to QOSR at EL1
    #[bits(9..=9, rw)]
    qosr: bool,
    /// Controls access to IMP_PERIPHPREGIONR at EL1
    #[bits(8..=8, rw)]
    periphpregionr: bool,
    /// Controls access to IMP_FLASHIFREGIONR at EL1
    #[bits(7..=7, rw)]
    flashifregionr: bool,
    /// Controls access to CDBGDCI at EL1
    #[bits(1..=1, rw)]
    cdbgdci: bool,
    /// IMP_CPUACTLR write access control
    #[bits(0..=0, rw)]
    cpuactlr: bool,
}

impl SysReg for Hactlr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}

impl SysRegRead for Hactlr {}

impl SysRegWrite for Hactlr {}

impl Hactlr {
    /// Read HACTRL (*Hyp Auxiliary Control Register*)
    #[inline]
    pub fn read() -> Hactlr {
        // Safety: Reading this register has no side-effects and is atomic
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }

    /// Write HACTRL (*Hyp Auxiliary Control Register*)
    #[inline]
    pub fn write(value: Self) {
        // Safety: Writing this register is atomic
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }

    /// Modify HACTRL (*Hyp Auxiliary Control Register*)
    #[inline]
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }
}

impl core::fmt::Debug for Hactlr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HACTLR {{ CPUACTLR={}, CDBGDCI={}, FLASHIFREGIONR={}, PERIPHPREGIONR={}, QOSR={}, BUSTIMEOUTR={}, INTMONR={}, ERR={}, TESTR1={} }}",
            self.cpuactlr() as u8,
            self.cdbgdci() as u8,
            self.flashifregionr() as u8,
            self.periphpregionr() as u8,
            self.qosr() as u8,
            self.bustimeoutr() as u8,
            self.intmonr() as u8,
            self.err() as u8,
            self.testr1() as u8
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Hactlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HACTLR {{ CPUACTLR={0=0..1}, CDBGDCI={0=1..2}, FLASHIFREGIONR={0=7..8}, PERIPHPREGIONR={0=8..9}, QOSR={0=9..10}, BUSTIMEOUTR={0=10..11}, INTMONR={0=12..13}, ERR={0=13..14}, TESTR1={0=15..16} }}", self.raw_value())
    }
}
