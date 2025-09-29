//! Code for managing ID_ISAR2 (*Instruction Set Attribute Register 2*)

use crate::register::{SysReg, SysRegRead};

/// ID_ISAR2 (*Instruction Set Attribute Register 2*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdIsar2(pub u32);
impl SysReg for IdIsar2 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for IdIsar2 {}
impl IdIsar2 {
    #[inline]
    /// Reads ID_ISAR2 (*Instruction Set Attribute Register 2*)
    pub fn read() -> IdIsar2 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
