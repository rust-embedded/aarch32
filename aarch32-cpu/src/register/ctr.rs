//! Code for managing CTR (*Cache Type Register*)

use crate::register::{SysReg, SysRegRead};

/// CTR (*Cache Type Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ctr(pub u32);

impl SysReg for Ctr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Ctr {}

impl Ctr {
    #[inline]
    /// Reads CTR (*Cache Type Register*)
    pub fn read() -> Ctr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
