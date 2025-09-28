//! Code for managing IMP_CDBGDCD (*Data Cache Data Read Operation.*)

use crate::register::{SysReg, SysRegWrite};

/// IMP_CDBGDCD (*Data Cache Data Read Operation.*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpCdbgdcd(pub u32);
impl SysReg for ImpCdbgdcd {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 3;
    const CRM: u32 = 4;
    const OP2: u32 = 0;
}
impl crate::register::SysRegWrite for ImpCdbgdcd {}
impl ImpCdbgdcd {
    #[inline]
    /// Writes IMP_CDBGDCD (*Data Cache Data Read Operation.*)
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
