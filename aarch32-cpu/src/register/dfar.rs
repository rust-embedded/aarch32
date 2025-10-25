//! Code for managing DFAR (*Data Fault Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DFAR (*Data Fault Address Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dfar(pub u32);
impl SysReg for Dfar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Dfar {}
impl Dfar {
    #[inline]
    /// Reads DFAR (*Data Fault Address Register*)
    pub fn read() -> Dfar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Dfar {}
impl Dfar {
    #[inline]
    /// Writes DFAR (*Data Fault Address Register*)
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
