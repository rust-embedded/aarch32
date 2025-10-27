//! Code for managing TLBTR (*TLB Type Register*)

use crate::register::{SysReg, SysRegRead};

/// TLBTR (*TLB Type Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tlbtr(pub u32);

impl SysReg for Tlbtr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 3;
}

impl crate::register::SysRegRead for Tlbtr {}

impl Tlbtr {
    #[inline]
    /// Reads TLBTR (*TLB Type Register*)
    pub fn read() -> Tlbtr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
