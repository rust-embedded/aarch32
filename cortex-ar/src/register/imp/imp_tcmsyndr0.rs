//! Code for managing IMP_TCMSYNDR0 (*TCM Syndrome Register 0*)

use crate::register::{SysReg, SysRegRead};

/// IMP_TCMSYNDR0 (*TCM Syndrome Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpTcmsyndr0(pub u32);
impl SysReg for ImpTcmsyndr0 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 2;
    const CRM: u32 = 2;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for ImpTcmsyndr0 {}
impl ImpTcmsyndr0 {
    #[inline]
    /// Reads IMP_TCMSYNDR0 (*TCM Syndrome Register 0*)
    pub fn read() -> ImpTcmsyndr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
