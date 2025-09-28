//! Code for managing CNTPCT (*Physical Counter-timer Count Register*)

use crate::register::{SysReg64, SysRegRead64};

/// CNTPCT (*Physical Counter-timer Count Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CntPct(pub u64);

impl SysReg64 for CntPct {
    const CP: u32 = 15;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
}

impl SysRegRead64 for CntPct {}

impl CntPct {
    #[inline]
    /// Reads CNTPCT (*Physical Counter-timer Count Register*)
    pub fn read() -> CntPct {
        unsafe { Self(<Self as SysRegRead64>::read_raw()) }
    }
}
