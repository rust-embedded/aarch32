//! Code for managing IMP_TCMSYNDR1 (*TCM Syndrome Register 1*)

use crate::register::{SysReg, SysRegRead};

/// IMP_TCMSYNDR1 (*TCM Syndrome Register 1*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpTcmsyndr1(pub u32);

impl SysReg for ImpTcmsyndr1 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 2;
    const CRM: u32 = 2;
    const OP2: u32 = 3;
}

impl crate::register::SysRegRead for ImpTcmsyndr1 {}

impl ImpTcmsyndr1 {
    #[inline]
    /// Reads IMP_TCMSYNDR1 (*TCM Syndrome Register 1*)
    pub fn read() -> ImpTcmsyndr1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
