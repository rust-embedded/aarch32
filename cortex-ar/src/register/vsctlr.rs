//! Code for managing VSCTLR (*Virtualization System Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// VSCTLR (*Virtualization System Control Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vsctlr(pub u32);
impl SysReg for Vsctlr {
    const CP: u32 = 15;
    const CRN: u32 = 2;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Vsctlr {}
impl Vsctlr {
    #[inline]
    /// Reads VSCTLR (*Virtualization System Control Register*)
    pub fn read() -> Vsctlr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Vsctlr {}
impl Vsctlr {
    #[inline]
    /// Writes VSCTLR (*Virtualization System Control Register*)
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
