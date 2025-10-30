//! Code for managing ID_MMFR4 (*Memory Model Feature Register 4*)

use crate::register::{SysReg, SysRegRead};

/// ID_MMFR4 (*Memory Model Feature Register 4*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdMmfr4(pub u32);

impl SysReg for IdMmfr4 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 6;
}

impl crate::register::SysRegRead for IdMmfr4 {}

impl IdMmfr4 {
    #[inline]
    /// Reads ID_MMFR4 (*Memory Model Feature Register 4*)
    pub fn read() -> IdMmfr4 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
