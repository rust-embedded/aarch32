//! Code for managing PAR (*Physical Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PAR (*Physical Address Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Par(pub u32);
impl SysReg for Par {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 4;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Par {}
impl Par {
    #[inline]
    /// Reads PAR (*Physical Address Register*)
    pub fn read() -> Par {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Par {}
impl Par {
    #[inline]
    /// Writes PAR (*Physical Address Register*)
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
