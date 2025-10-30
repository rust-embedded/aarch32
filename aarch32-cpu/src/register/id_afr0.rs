//! Code for managing ID_AFR0 (*Auxiliary Feature Register 0*)

use crate::register::{SysReg, SysRegRead};

/// ID_AFR0 (*Auxiliary Feature Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdAfr0(pub u32);

impl SysReg for IdAfr0 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 3;
}

impl crate::register::SysRegRead for IdAfr0 {}

impl IdAfr0 {
    #[inline]
    /// Reads ID_AFR0 (*Auxiliary Feature Register 0*)
    pub fn read() -> IdAfr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
