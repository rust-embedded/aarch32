//! Code for managing IMP_CBAR (*Configuration Base Address Register*)

use crate::register::{SysReg, SysRegRead};

/// IMP_CBAR (*Configuration Base Address Register*)
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpCbar(pub u32);

impl SysReg for ImpCbar {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 1;
    const CRM: u32 = 3;
    const OP2: u32 = 0;
}

impl SysRegRead for ImpCbar {}

impl ImpCbar {
    /// Read IMP_CBAR (*Configuration Base Address Register*)
    #[inline]
    pub fn read() -> ImpCbar {
        // Safety: this read has no side-effects
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }

    /// Get the periphbase address
    pub fn periphbase(self) -> *mut u32 {
        (self.0 & 0xFFF00000) as *mut u32
    }
}

impl core::fmt::Debug for ImpCbar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "IMP_CBAR {{ {:010p} }}", self.periphbase())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ImpCbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IMP_CBAR {{ 0x{=usize:08x} }}", self.0 as usize)
    }
}
