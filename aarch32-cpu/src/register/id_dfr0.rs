//! Code for managing ID_DFR0 (*Debug Feature Register 0*)

use crate::register::{SysReg, SysRegRead};

/// ID_DFR0 (*Debug Feature Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdDfr0(pub u32);

impl SysReg for IdDfr0 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 2;
}

impl crate::register::SysRegRead for IdDfr0 {}

impl IdDfr0 {
    #[inline]
    /// Reads ID_DFR0 (*Debug Feature Register 0*)
    pub fn read() -> IdDfr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
