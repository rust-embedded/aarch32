//! Code for managing HTPIDR (*Hyp Software Thread ID Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HTPIDR (*Hyp Software Thread ID Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Htpidr(pub u32);
impl SysReg for Htpidr {
    const CP: u32 = 15;
    const CRN: u32 = 13;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for Htpidr {}
impl Htpidr {
    #[inline]
    /// Reads HTPIDR (*Hyp Software Thread ID Register*)
    pub fn read() -> Htpidr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Htpidr {}
impl Htpidr {
    #[inline]
    /// Writes HTPIDR (*Hyp Software Thread ID Register*)
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
