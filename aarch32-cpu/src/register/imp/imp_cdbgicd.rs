//! Code for managing IMP_CDBGICD (*Instruction Cache Data Read Operation.*)

use crate::register::{SysReg, SysRegWrite};

/// IMP_CDBGICD (*Instruction Cache Data Read Operation.*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpCdbgicd(pub u32);

impl SysReg for ImpCdbgicd {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 3;
    const CRM: u32 = 4;
    const OP2: u32 = 1;
}

impl crate::register::SysRegWrite for ImpCdbgicd {}

impl ImpCdbgicd {
    #[inline]
    /// Writes IMP_CDBGICD (*Instruction Cache Data Read Operation.*)
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
