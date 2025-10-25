//! Code for managing HDCR (*Hyp Debug Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HDCR (*Hyp Debug Control Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hdcr(pub u32);
impl SysReg for Hdcr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Hdcr {}
impl Hdcr {
    #[inline]
    /// Reads HDCR (*Hyp Debug Control Register*)
    pub fn read() -> Hdcr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hdcr {}
impl Hdcr {
    #[inline]
    /// Writes HDCR (*Hyp Debug Control Register*)
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
