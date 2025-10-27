//! Code for managing PMCEID0 (*Performance Monitors Common Event Identification Register 0*)

use crate::register::{SysReg, SysRegRead};

/// PMCEID0 (*Performance Monitors Common Event Identification Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmceid0(pub u32);

impl SysReg for Pmceid0 {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 6;
}

impl crate::register::SysRegRead for Pmceid0 {}

impl Pmceid0 {
    #[inline]
    /// Reads PMCEID0 (*Performance Monitors Common Event Identification Register 0*)
    pub fn read() -> Pmceid0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
