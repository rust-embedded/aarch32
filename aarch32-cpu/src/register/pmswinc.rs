//! Code for managing PMSWINC (*Performance Monitors Software Increment Register*)

use crate::register::{SysReg, SysRegWrite};

/// PMSWINC (*Performance Monitors Software Increment Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmswinc(pub u32);

impl SysReg for Pmswinc {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 4;
}

impl crate::register::SysRegWrite for Pmswinc {}

impl Pmswinc {
    #[inline]
    /// Writes PMSWINC (*Performance Monitors Software Increment Register*)
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
