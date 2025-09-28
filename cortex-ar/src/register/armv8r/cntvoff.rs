//! Code for managing CNTVOFF (*Virtual Counter-timer Offset Register*)

use crate::register::{SysReg64, SysRegRead64, SysRegWrite64};

/// CNTVOFF (*Virtual Counter-timer Offset Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CntVoff(pub u64);

impl SysReg64 for CntVoff {
    const CP: u32 = 15;
    const OP1: u32 = 4;
    const CRM: u32 = 14;
}

impl SysRegRead64 for CntVoff {}

impl CntVoff {
    #[inline]
    /// Reads CNTVOFF (*Virtual Counter-timer Offset Register*)
    pub fn read() -> CntVoff {
        unsafe { Self(<Self as SysRegRead64>::read_raw()) }
    }
}

impl SysRegWrite64 for CntVoff {}

impl CntVoff {
    #[inline]
    /// Writes CNTVOFF (*Virtual Counter-timer Offset Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite64>::write_raw(value.0);
        }
    }
}
