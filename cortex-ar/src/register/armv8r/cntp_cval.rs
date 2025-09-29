//! Code for managing CNTP_CVAL (*Physical Counter-timer CompareValue Register*)

use crate::register::{SysReg64, SysRegRead64, SysRegWrite64};

/// CNTP_CVAL (*Physical Counter-timer CompareValue Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CntpCval(pub u64);

impl SysReg64 for CntpCval {
    const CP: u32 = 15;
    const OP1: u32 = 2;
    const CRM: u32 = 14;
}

impl SysRegRead64 for CntpCval {}

impl CntpCval {
    #[inline]
    /// Reads CNTP_CVAL (*Physical Counter-timer CompareValue Register*)
    pub fn read() -> CntpCval {
        unsafe { Self(<Self as SysRegRead64>::read_raw()) }
    }
}

impl SysRegWrite64 for CntpCval {}

impl CntpCval {
    #[inline]
    /// Writes CNTP_CVAL (*Physical Counter-timer CompareValue Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite64>::write_raw(value.0);
        }
    }
}
