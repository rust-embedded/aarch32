//! Code for managing ID_MMFR1 (*Memory Model Feature Register 1*)

use crate::register::{SysReg, SysRegRead};

/// ID_MMFR1 (*Memory Model Feature Register 1*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdMmfr1(pub u32);
impl SysReg for IdMmfr1 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for IdMmfr1 {}
impl IdMmfr1 {
    #[inline]
    /// Reads ID_MMFR1 (*Memory Model Feature Register 1*)
    pub fn read() -> IdMmfr1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
