//! Code for managing CPACR (*Architectural Feature Access Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CPACR (*Architectural Feature Access Control Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cpacr(pub u32);

impl SysReg for Cpacr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 2;
}

impl crate::register::SysRegRead for Cpacr {}

impl Cpacr {
    #[inline]
    /// Reads CPACR (*Architectural Feature Access Control Register*)
    pub fn read() -> Cpacr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Cpacr {}

impl Cpacr {
    #[inline]
    /// Writes CPACR (*Architectural Feature Access Control Register*)
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
