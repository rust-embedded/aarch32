//! Code for managing IMP_CDBGDR0 (*Cache Debug Data Register 0.*)

use crate::register::{SysReg, SysRegRead};

/// IMP_CDBGDR0 (*Cache Debug Data Register 0.*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpCdbgdr0(pub u32);
impl SysReg for ImpCdbgdr0 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 3;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for ImpCdbgdr0 {}
impl ImpCdbgdr0 {
    #[inline]
    /// Reads IMP_CDBGDR0 (*Cache Debug Data Register 0.*)
    pub fn read() -> ImpCdbgdr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
