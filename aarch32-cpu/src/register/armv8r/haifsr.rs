//! Code for managing HAIFSR (*Hyp Auxiliary Instruction Fault Status Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HAIFSR (*Hyp Auxiliary Instruction Fault Status Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Haifsr(pub u32);

impl SysReg for Haifsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Haifsr {}

impl Haifsr {
    #[inline]
    /// Reads HAIFSR (*Hyp Auxiliary Instruction Fault Status Register*)
    pub fn read() -> Haifsr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Haifsr {}

impl Haifsr {
    #[inline]
    /// Writes HAIFSR (*Hyp Auxiliary Instruction Fault Status Register*)
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
