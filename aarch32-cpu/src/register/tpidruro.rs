//! Code for managing TPIDRURO (*EL0 Read-Only Software Thread ID Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// TPIDRURO (*EL0 Read-Only Software Thread ID Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tpidruro(pub u32);

impl SysReg for Tpidruro {
    const CP: u32 = 15;
    const CRN: u32 = 13;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 3;
}

impl crate::register::SysRegRead for Tpidruro {}

impl Tpidruro {
    #[inline]
    /// Reads TPIDRURO (*EL0 Read-Only Software Thread ID Register*)
    pub fn read() -> Tpidruro {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Tpidruro {}

impl Tpidruro {
    #[inline]
    /// Writes TPIDRURO (*EL0 Read-Only Software Thread ID Register*)
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
