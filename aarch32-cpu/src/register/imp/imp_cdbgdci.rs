//! Code for managing IMP_CDBGDCI (*Invalidate All Register*)

use crate::register::{SysReg, SysRegWrite};

/// IMP_CDBGDCI (*Invalidate All Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpCdbgdci(pub u32);
impl SysReg for ImpCdbgdci {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 1;
    const CRM: u32 = 14;
    const OP2: u32 = 0;
}
impl crate::register::SysRegWrite for ImpCdbgdci {}
impl ImpCdbgdci {
    #[inline]
    /// Writes IMP_CDBGDCI (*Invalidate All Register*)
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
