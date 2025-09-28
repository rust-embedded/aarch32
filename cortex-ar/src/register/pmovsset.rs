//! Code for managing PMOVSSET (*Performance Monitor Overflow Flag Status Set Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMOVSSET (*Performance Monitor Overflow Flag Status Set Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmovsset(pub u32);
impl SysReg for Pmovsset {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 3;
}
impl crate::register::SysRegRead for Pmovsset {}
impl Pmovsset {
    #[inline]
    /// Reads PMOVSSET (*Performance Monitor Overflow Flag Status Set Register*)
    pub fn read() -> Pmovsset {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Pmovsset {}
impl Pmovsset {
    #[inline]
    /// Writes PMOVSSET (*Performance Monitor Overflow Flag Status Set Register*)
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
