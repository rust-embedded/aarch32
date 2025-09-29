//! Code for managing CNTHP_TVAL (*Hyp Physical Counter-timer TimerValue Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTHP_TVAL (*Hyp Physical Counter-timer TimerValue Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CnthpTval(pub u32);

impl SysReg for CnthpTval {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 4;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}

impl SysRegRead for CnthpTval {}

impl CnthpTval {
    #[inline]
    /// Reads CNTHP_TVAL (*Hyp Physical Counter-timer TimerValue Register*)
    pub fn read() -> CnthpTval {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for CnthpTval {}

impl CnthpTval {
    #[inline]
    /// Writes CNTHP_TVAL (*Hyp Physical Counter-timer TimerValue Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
