//! Code for managing ID_MMFR3 (*Memory Model Feature Register 3*)

use crate::register::{SysReg, SysRegRead};

/// ID_MMFR3 (*Memory Model Feature Register 3*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdMmfr3(pub u32);

impl SysReg for IdMmfr3 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 7;
}

impl crate::register::SysRegRead for IdMmfr3 {}

impl IdMmfr3 {
    #[inline]
    /// Reads ID_MMFR3 (*Memory Model Feature Register 3*)
    pub fn read() -> IdMmfr3 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
