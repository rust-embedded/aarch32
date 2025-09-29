//! Code for managing IMP_CTCMREGIONR (*TCM Region Registers A B and C*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_CTCMREGIONR (*TCM Region Registers A B and C*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpCtcmregionr(pub u32);
impl SysReg for ImpCtcmregionr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for ImpCtcmregionr {}
impl ImpCtcmregionr {
    #[inline]
    /// Reads IMP_CTCMREGIONR (*TCM Region Registers A B and C*)
    pub fn read() -> ImpCtcmregionr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpCtcmregionr {}
impl ImpCtcmregionr {
    #[inline]
    /// Writes IMP_CTCMREGIONR (*TCM Region Registers A B and C*)
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
