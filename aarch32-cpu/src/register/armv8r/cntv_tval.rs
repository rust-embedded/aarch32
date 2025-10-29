//! Code for managing CNTV_TVAL (*Virtual Counter-timer TimerValue Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTV_TVAL (*Virtual Counter-timer TimerValue Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CntvTval(pub u32);
impl SysReg for CntvTval {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 3;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for CntvTval {}
impl CntvTval {
    #[inline]
    /// Reads CNTV_TVAL (*Virtual Counter-timer TimerValue Register*)
    pub fn read() -> CntvTval {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for CntvTval {}
impl CntvTval {
    #[inline]
    /// Writes CNTV_TVAL (*Virtual Counter-timer TimerValue Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
