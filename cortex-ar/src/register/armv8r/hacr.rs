//! Code for managing HACR (*Hyp Auxiliary Configuration Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HACR (*Hyp Auxiliary Configuration Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hacr(pub u32);
impl SysReg for Hacr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 7;
}
impl crate::register::SysRegRead for Hacr {}
impl Hacr {
    #[inline]
    /// Reads HACR (*Hyp Auxiliary Configuration Register*)
    pub fn read() -> Hacr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hacr {}
impl Hacr {
    #[inline]
    /// Writes HACR (*Hyp Auxiliary Configuration Register*)
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
