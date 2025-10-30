//! Code for managing PMCCFILTR (*Performance Monitors Cycle Count Filter Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMCCFILTR (*Performance Monitors Cycle Count Filter Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmccfiltr(pub u32);

impl SysReg for Pmccfiltr {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 15;
    const OP2: u32 = 7;
}

impl crate::register::SysRegRead for Pmccfiltr {}

impl Pmccfiltr {
    #[inline]
    /// Reads PMCCFILTR (*Performance Monitors Cycle Count Filter Register*)
    pub fn read() -> Pmccfiltr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmccfiltr {}

impl Pmccfiltr {
    #[inline]
    /// Writes PMCCFILTR (*Performance Monitors Cycle Count Filter Register*)
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
