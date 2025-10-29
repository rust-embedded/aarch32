//! Code for managing ID_ISAR1 (*Instruction Set Attribute Register 1*)

use crate::register::{SysReg, SysRegRead};

/// ID_ISAR1 (*Instruction Set Attribute Register 1*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdIsar1(pub u32);
impl SysReg for IdIsar1 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for IdIsar1 {}
impl IdIsar1 {
    #[inline]
    /// Reads ID_ISAR1 (*Instruction Set Attribute Register 1*)
    pub fn read() -> IdIsar1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
