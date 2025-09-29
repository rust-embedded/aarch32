//! Code for managing IMP_CDBGDR2 (*Cache Debug Data Register 2.*)

use crate::register::{SysReg, SysRegRead};

/// IMP_CDBGDR2 (*Cache Debug Data Register 2.*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpCdbgdr2(pub u32);
impl SysReg for ImpCdbgdr2 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 3;
    const CRM: u32 = 0;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for ImpCdbgdr2 {}
impl ImpCdbgdr2 {
    #[inline]
    /// Reads IMP_CDBGDR2 (*Cache Debug Data Register 2.*)
    pub fn read() -> ImpCdbgdr2 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
