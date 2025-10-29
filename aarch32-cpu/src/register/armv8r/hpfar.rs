//! Code for managing HPFAR (*Hyp IPA Fault Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPFAR (*Hyp IPA Fault Address Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hpfar(pub u32);
impl SysReg for Hpfar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Hpfar {}
impl Hpfar {
    #[inline]
    /// Reads HPFAR (*Hyp IPA Fault Address Register*)
    pub fn read() -> Hpfar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hpfar {}
impl Hpfar {
    #[inline]
    /// Writes HPFAR (*Hyp IPA Fault Address Register*)
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
