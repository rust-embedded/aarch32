//! Code for managing FCSEIDR (*FCSE Process ID Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// FCSEIDR (*FCSE Process ID Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fcseidr(pub u32);
impl SysReg for Fcseidr {
    const CP: u32 = 15;
    const CRN: u32 = 13;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Fcseidr {}
impl Fcseidr {
    #[inline]
    /// Reads FCSEIDR (*FCSE Process ID Register*)
    pub fn read() -> Fcseidr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Fcseidr {}
impl Fcseidr {
    #[inline]
    /// Writes FCSEIDR (*FCSE Process ID Register*)
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
