//! Code for managing IMP_BPCTLR (*Branch Predictor Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_BPCTLR (*Branch Predictor Control Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpBpctlr(pub u32);

impl SysReg for ImpBpctlr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 1;
    const CRM: u32 = 1;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for ImpBpctlr {}

impl ImpBpctlr {
    #[inline]
    /// Reads IMP_BPCTLR (*Branch Predictor Control Register*)
    pub fn read() -> ImpBpctlr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for ImpBpctlr {}

impl ImpBpctlr {
    #[inline]
    /// Writes IMP_BPCTLR (*Branch Predictor Control Register*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
