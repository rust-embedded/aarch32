//! Code for managing PMCCNTR (*Performance Monitors Cycle Count Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMCCNTR (*Performance Monitors Cycle Count Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmccntr(pub u32);
impl SysReg for Pmccntr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 13;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Pmccntr {}
impl Pmccntr {
    #[inline]
    /// Reads PMCCNTR (*Performance Monitors Cycle Count Register*)
    pub fn read() -> Pmccntr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Pmccntr {}
impl Pmccntr {
    #[inline]
    /// Writes PMCCNTR (*Performance Monitors Cycle Count Register*)
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
