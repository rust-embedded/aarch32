//! Code for managing IMP_CSCTLR (*Cache Segregation Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_CSCTLR (*Cache Segregation Control Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpCsctlr(pub u32);
impl SysReg for ImpCsctlr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 1;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for ImpCsctlr {}
impl ImpCsctlr {
    #[inline]
    /// Reads IMP_CSCTLR (*Cache Segregation Control Register*)
    pub fn read() -> ImpCsctlr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpCsctlr {}
impl ImpCsctlr {
    #[inline]
    /// Writes IMP_CSCTLR (*Cache Segregation Control Register*)
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
