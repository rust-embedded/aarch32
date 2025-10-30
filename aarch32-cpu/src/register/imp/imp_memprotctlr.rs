//! Code for managing IMP_MEMPROTCTLR (*Memory Protection Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_MEMPROTCTLR (*Memory Protection Control Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpMemprotctlr(pub u32);

impl SysReg for ImpMemprotctlr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 1;
    const CRM: u32 = 1;
    const OP2: u32 = 2;
}

impl crate::register::SysRegRead for ImpMemprotctlr {}

impl ImpMemprotctlr {
    #[inline]
    /// Reads IMP_MEMPROTCTLR (*Memory Protection Control Register*)
    pub fn read() -> ImpMemprotctlr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for ImpMemprotctlr {}

impl ImpMemprotctlr {
    #[inline]
    /// Writes IMP_MEMPROTCTLR (*Memory Protection Control Register*)
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
