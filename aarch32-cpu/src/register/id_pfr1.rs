//! Code for managing ID_PFR1 (*Processor Feature Register 1*)

use crate::register::{SysReg, SysRegRead};

/// ID_PFR1 (*Processor Feature Register 1*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdPfr1(pub u32);

impl SysReg for IdPfr1 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for IdPfr1 {}

impl IdPfr1 {
    #[inline]
    /// Reads ID_PFR1 (*Processor Feature Register 1*)
    pub fn read() -> IdPfr1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
