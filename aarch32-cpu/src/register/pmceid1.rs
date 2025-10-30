//! Code for managing PMCEID1 (*Performance Monitors Common Event Identification Register 1*)

use crate::register::{SysReg, SysRegRead};

/// PMCEID1 (*Performance Monitors Common Event Identification Register 1*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmceid1(pub u32);

impl SysReg for Pmceid1 {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 7;
}

impl crate::register::SysRegRead for Pmceid1 {}

impl Pmceid1 {
    #[inline]
    /// Reads PMCEID1 (*Performance Monitors Common Event Identification Register 1*)
    pub fn read() -> Pmceid1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
