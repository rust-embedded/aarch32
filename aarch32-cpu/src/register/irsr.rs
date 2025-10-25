//! Code for managing IRSR (*Instruction Region Size and Enable Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

pub use super::drsr::RegionSize;

/// IRSR (*Instruction Region Size and Enable Register*)
#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Irsr {
    /// Sub-region bitmask
    ///
    /// The region is divided into exactly eight equal sized subregions.
    /// Subregion 0 is the subregion at the least significant address.
    ///
    /// A 1 bit means that sub-region is disabled.
    ///
    /// Only applies to regions sized 256 bytes or larger.
    #[bits(8..=15, rw)]
    subregion_mask: u8,
    /// Region Size
    #[bits(1..=5, rw)]
    region_size: RegionSize,
    /// Is region enabled?
    #[bits(0..=0, rw)]
    enabled: bool,
}

impl SysReg for Irsr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 3;
}

impl crate::register::SysRegRead for Irsr {}

impl Irsr {
    #[inline]
    /// Reads IRSR (*Instruction Region Size and Enable Register*)
    ///
    /// Set RGNR to control which region this reads.
    pub fn read() -> Irsr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Irsr {}

impl Irsr {
    #[inline]
    /// Writes IRSR (*Instruction Region Size and Enable Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Irsr) {
        unsafe { <Self as SysRegWrite>::write_raw(value.raw_value()) }
    }
}
