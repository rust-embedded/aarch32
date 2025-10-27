//! Code for managing TPIDRPRW (*EL1 Software Thread ID Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// TPIDRPRW (*EL1 Software Thread ID Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tpidrprw(pub u32);

impl SysReg for Tpidrprw {
    const CP: u32 = 15;
    const CRN: u32 = 13;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Tpidrprw {}

impl Tpidrprw {
    #[inline]
    /// Reads TPIDRPRW (*EL1 Software Thread ID Register*)
    pub fn read() -> Tpidrprw {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Tpidrprw {}

impl Tpidrprw {
    #[inline]
    /// Writes TPIDRPRW (*EL1 Software Thread ID Register*)
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
