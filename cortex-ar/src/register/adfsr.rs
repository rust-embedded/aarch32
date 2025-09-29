//! Code for managing ADFSR (*Auxiliary Data Fault Status Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// ADFSR (*Auxiliary Data Fault Status Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Adfsr(pub u32);
impl SysReg for Adfsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Adfsr {}
impl Adfsr {
    #[inline]
    /// Reads ADFSR (*Auxiliary Data Fault Status Register*)
    pub fn read() -> Adfsr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Adfsr {}
impl Adfsr {
    #[inline]
    /// Writes ADFSR (*Auxiliary Data Fault Status Register*)
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
