//! Code for managing IMP_TESTR0 (*Test Register 0*)

use crate::register::{SysReg, SysRegRead};

/// IMP_TESTR0 (*Test Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpTestr0(pub u32);
impl SysReg for ImpTestr0 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for ImpTestr0 {}
impl ImpTestr0 {
    #[inline]
    /// Reads IMP_TESTR0 (*Test Register 0*)
    pub fn read() -> ImpTestr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
