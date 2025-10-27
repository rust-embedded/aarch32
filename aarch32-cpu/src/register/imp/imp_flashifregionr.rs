//! Code for managing IMP_FLASHIFREGIONR (*Flash Interface Region Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_FLASHIFREGIONR (*Flash Interface Region Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpFlashifregionr(pub u32);

impl SysReg for ImpFlashifregionr {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for ImpFlashifregionr {}

impl ImpFlashifregionr {
    #[inline]
    /// Reads IMP_FLASHIFREGIONR (*Flash Interface Region Register*)
    pub fn read() -> ImpFlashifregionr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for ImpFlashifregionr {}

impl ImpFlashifregionr {
    #[inline]
    /// Writes IMP_FLASHIFREGIONR (*Flash Interface Region Register*)
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
