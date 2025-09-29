//! Code for managing CNTVCT (*Virtual Counter-timer Count Register*)

use crate::register::{SysReg64, SysRegRead64};

/// CNTVCT (*Virtual Counter-timer Count Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CntVct(pub u64);

impl SysReg64 for CntVct {
    const CP: u32 = 15;
    const OP1: u32 = 1;
    const CRM: u32 = 14;
}

impl SysRegRead64 for CntVct {}

impl CntVct {
    #[inline]
    /// Reads CNTVCT (*Virtual Counter-timer Count Register*)
    pub fn read() -> CntVct {
        unsafe { Self(<Self as SysRegRead64>::read_raw()) }
    }
}
