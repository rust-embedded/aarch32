//! Code for managing HPRENR (*Hyp MPU Region Enable Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRENR (*Hyp MPU Region Enable Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprenr(pub u32);
impl SysReg for Hprenr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Hprenr {}
impl Hprenr {
    #[inline]
    /// Reads HPRENR (*Hyp MPU Region Enable Register*)
    pub fn read() -> Hprenr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprenr {}
impl Hprenr {
    #[inline]
    /// Writes HPRENR (*Hyp MPU Region Enable Register*)
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
