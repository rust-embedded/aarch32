//! Code for managing HSR (*Hyp Syndrome Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HSR (*Hyp Syndrome Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hsr(pub u32);
impl SysReg for Hsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 4;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Hsr {}
impl Hsr {
    #[inline]
    /// Reads HSR (*Hyp Syndrome Register*)
    pub fn read() -> Hsr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hsr {}
impl Hsr {
    #[inline]
    /// Writes HSR (*Hyp Syndrome Register*)
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
