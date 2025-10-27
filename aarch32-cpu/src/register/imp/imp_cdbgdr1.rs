//! Code for managing IMP_CDBGDR1 (*Cache Debug Data Register 1.*)

use crate::register::{SysReg, SysRegRead};

/// IMP_CDBGDR1 (*Cache Debug Data Register 1.*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpCdbgdr1(pub u32);

impl SysReg for ImpCdbgdr1 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 3;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for ImpCdbgdr1 {}

impl ImpCdbgdr1 {
    #[inline]
    /// Reads IMP_CDBGDR1 (*Cache Debug Data Register 1.*)
    pub fn read() -> ImpCdbgdr1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
