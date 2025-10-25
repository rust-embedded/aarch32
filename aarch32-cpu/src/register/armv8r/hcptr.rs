//! Code for managing HCPTR (*Hyp Architectural Feature Trap Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HCPTR (*Hyp Architectural Feature Trap Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hcptr(pub u32);
impl SysReg for Hcptr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for Hcptr {}
impl Hcptr {
    #[inline]
    /// Reads HCPTR (*Hyp Architectural Feature Trap Register*)
    pub fn read() -> Hcptr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hcptr {}
impl Hcptr {
    #[inline]
    /// Writes HCPTR (*Hyp Architectural Feature Trap Register*)
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
