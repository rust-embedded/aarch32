//! Code for managing HIFAR (*Hyp Instruction Fault Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HIFAR (*Hyp Instruction Fault Address Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hifar(pub u32);
impl SysReg for Hifar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for Hifar {}
impl Hifar {
    #[inline]
    /// Reads HIFAR (*Hyp Instruction Fault Address Register*)
    pub fn read() -> Hifar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hifar {}
impl Hifar {
    #[inline]
    /// Writes HIFAR (*Hyp Instruction Fault Address Register*)
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
