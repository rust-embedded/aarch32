//! Code for managing CNTP_TVAL (*Physical Counter-timer TimerValue Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTP_TVAL (*Physical Counter-timer TimerValue Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CntpTval(pub u32);
impl SysReg for CntpTval {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}

impl SysRegRead for CntpTval {}

impl CntpTval {
    #[inline]
    /// Reads CNTP_TVAL (*Physical Counter-timer TimerValue Register*)
    pub fn read() -> CntpTval {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for CntpTval {}

impl CntpTval {
    #[inline]
    /// Writes CNTP_TVAL (*Physical Counter-timer TimerValue Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
