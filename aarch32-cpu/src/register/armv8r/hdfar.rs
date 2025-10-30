//! Code for managing HDFAR (*Hyp Data Fault Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HDFAR (*Hyp Data Fault Address Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hdfar(pub u32);

impl SysReg for Hdfar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Hdfar {}

impl Hdfar {
    #[inline]
    /// Reads HDFAR (*Hyp Data Fault Address Register*)
    pub fn read() -> Hdfar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hdfar {}

impl Hdfar {
    #[inline]
    /// Writes HDFAR (*Hyp Data Fault Address Register*)
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
