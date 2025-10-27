//! Code for managing IMP_CDBGDCT (*Data Cache Tag Read Operation.*)

use crate::register::{SysReg, SysRegWrite};

/// IMP_CDBGDCT (*Data Cache Tag Read Operation.*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpCdbgdct(pub u32);

impl SysReg for ImpCdbgdct {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 3;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}

impl crate::register::SysRegWrite for ImpCdbgdct {}

impl ImpCdbgdct {
    #[inline]
    /// Writes IMP_CDBGDCT (*Data Cache Tag Read Operation.*)
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
