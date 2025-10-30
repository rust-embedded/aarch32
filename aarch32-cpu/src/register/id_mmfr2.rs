//! Code for managing ID_MMFR2 (*Memory Model Feature Register 2*)

use crate::register::{SysReg, SysRegRead};

/// ID_MMFR2 (*Memory Model Feature Register 2*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdMmfr2(pub u32);

impl SysReg for IdMmfr2 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 6;
}

impl crate::register::SysRegRead for IdMmfr2 {}

impl IdMmfr2 {
    #[inline]
    /// Reads ID_MMFR2 (*Memory Model Feature Register 2*)
    pub fn read() -> IdMmfr2 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
