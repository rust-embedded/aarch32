//! Code for managing ID_MMFR0 (*Memory Model Feature Register 0*)

use crate::register::{SysReg, SysRegRead};

/// ID_MMFR0 (*Memory Model Feature Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdMmfr0(pub u32);

impl SysReg for IdMmfr0 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for IdMmfr0 {}

impl IdMmfr0 {
    #[inline]
    /// Reads ID_MMFR0 (*Memory Model Feature Register 0*)
    pub fn read() -> IdMmfr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
