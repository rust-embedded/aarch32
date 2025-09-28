//! Code for managing CNTFRQ (*Counter-timer Frequency Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTFRQ (*Counter-timer Frequency Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cntfrq(pub u32);
impl SysReg for Cntfrq {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl SysRegRead for Cntfrq {}

impl Cntfrq {
    #[inline]
    /// Reads CNTFRQ (*Counter-timer Frequency Register*)
    pub fn read() -> Cntfrq {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for Cntfrq {}

impl Cntfrq {
    #[inline]
    /// Writes CNTFRQ (*Counter-timer Frequency Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
