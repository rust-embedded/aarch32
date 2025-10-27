//! Code for managing NSACR (*Non-Secure Access Control Register*)

use crate::register::{SysReg, SysRegRead};

/// NSACR (*Non-Secure Access Control Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Nsacr(pub u32);

impl SysReg for Nsacr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 2;
}

impl crate::register::SysRegRead for Nsacr {}

impl Nsacr {
    #[inline]
    /// Reads NSACR (*Non-Secure Access Control Register*)
    pub fn read() -> Nsacr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
