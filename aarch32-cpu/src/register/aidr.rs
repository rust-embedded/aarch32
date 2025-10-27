//! Code for managing AIDR (*Auxiliary ID Register*)

use crate::register::{SysReg, SysRegRead};

/// AIDR (*Auxiliary ID Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Aidr(pub u32);

impl SysReg for Aidr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 1;
    const CRM: u32 = 0;
    const OP2: u32 = 7;
}

impl crate::register::SysRegRead for Aidr {}

impl Aidr {
    #[inline]
    /// Reads AIDR (*Auxiliary ID Register*)
    pub fn read() -> Aidr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
