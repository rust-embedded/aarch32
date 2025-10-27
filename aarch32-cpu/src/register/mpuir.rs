//! Code for managing MPUIR (*MPU Type Register*)

use crate::register::{SysReg, SysRegRead};

/// MPUIR (*MPU Type Register*)
#[bitbybit::bitfield(u32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mpuir {
    /// Specifies the number of Instruction regions implemented by the MPU.
    ///
    /// If the MPU implements a Unified memory map this field is UNK/SBZ.
    #[bits(16..=23, r)]
    iregions: u8,
    /// Specifies the number of Data or Unified regions implemented by the MPU.
    #[bits(8..=15, r)]
    dregions: u8,
    /// Is the MPU non-unified
    #[bits(0..=0, r)]
    non_unified: bool,
}

impl SysReg for Mpuir {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Mpuir {}

impl Mpuir {
    #[inline]
    /// Reads MPUIR (*MPU Type Register*)
    pub fn read() -> Mpuir {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl core::fmt::Debug for Mpuir {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Mpuir")
            .field("iregions", &self.iregions())
            .field("dregions", &self.dregions())
            .field("non_unified", &self.non_unified())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Mpuir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MPUIR {{ iregions={=u8}, dregions={=u8}, non_unified={=bool} }}",
            self.iregions(),
            self.dregions(),
            self.non_unified()
        )
    }
}
