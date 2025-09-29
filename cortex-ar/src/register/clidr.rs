//! Code for managing CLIDR (*Cache Level ID Register*)

use crate::register::{SysReg, SysRegRead};

/// CLIDR (*Cache Level ID Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Clidr(pub u32);
impl SysReg for Clidr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 1;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Clidr {}
impl Clidr {
    #[inline]
    /// Reads CLIDR (*Cache Level ID Register*)
    pub fn read() -> Clidr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
