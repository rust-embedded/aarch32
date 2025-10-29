//! Code for managing MIDR (*Main ID Register*)

use arbitrary_int::{u12, u4};

use super::{SysReg, SysRegRead};

/// MIDR (*Main ID Register*)
#[bitbybit::bitfield(u32)]
pub struct Midr {
    /// Implementer
    #[bits(24..=31, r)]
    implementer: u8,
    /// Variant
    #[bits(20..=23, r)]
    variant: u4,
    /// Architecture
    #[bits(16..=19, r)]
    arch: u4,
    /// Part Number
    #[bits(4..=15, r)]
    part_no: u12,
    /// Revision
    #[bits(0..=3, r)]
    rev: u4,
}

impl SysReg for Midr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl SysRegRead for Midr {}

impl Midr {
    /// Read MIDR (*Main ID Register*)
    #[inline]
    pub fn read() -> Midr {
        // Safety: Reading this register has no side-effects and is atomic
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl core::fmt::Debug for Midr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MIDR {{ implementer=0x{:02x} variant=0x{:x} arch=0x{:x} part_no=0x{:03x} rev=0x{:x} }}",
        self.implementer(), self.variant(), self.arch(), self.part_no(), self.rev())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Midr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MIDR {{ implementer=0x{0=24..32:02x} variant=0x{0=20..24:x} arch=0x{0=16..20:x} part_no=0x{0=4..16:03x} rev=0x{0=0..4:x} }}", self.raw_value())
    }
}
