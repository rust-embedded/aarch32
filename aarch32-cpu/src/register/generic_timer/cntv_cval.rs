//! Code for managing CNTV_CVAL (*Virtual Counter-timer CompareValue Register*)

use crate::register::{SysReg64, SysRegRead64, SysRegWrite64};

/// CNTV_CVAL (*Virtual Counter-timer CompareValue Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CntvCval(pub u64);

impl SysReg64 for CntvCval {
    const CP: u32 = 15;
    const OP1: u32 = 3;
    const CRM: u32 = 14;
}

impl SysRegRead64 for CntvCval {}

impl CntvCval {
    #[inline]
    /// Reads CNTV_CVAL (*Virtual Counter-timer CompareValue Register*)
    pub fn read() -> CntvCval {
        unsafe { Self(<Self as SysRegRead64>::read_raw()) }
    }
}

impl SysRegWrite64 for CntvCval {}

impl CntvCval {
    #[inline]
    /// Writes CNTV_CVAL (*Virtual Counter-timer CompareValue Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite64>::write_raw(value.0);
        }
    }
}
