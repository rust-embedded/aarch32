//! Code for managing ID_ISAR0 (*Instruction Set Attribute Register 0*)

use crate::register::{SysReg, SysRegRead};

/// ID_ISAR0 (*Instruction Set Attribute Register 0*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdIsar0(pub u32);
impl SysReg for IdIsar0 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for IdIsar0 {}
impl IdIsar0 {
    #[inline]
    /// Reads ID_ISAR0 (*Instruction Set Attribute Register 0*)
    pub fn read() -> IdIsar0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
