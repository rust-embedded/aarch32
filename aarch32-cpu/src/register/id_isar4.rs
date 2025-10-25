//! Code for managing ID_ISAR4 (*Instruction Set Attribute Register 4*)

use crate::register::{SysReg, SysRegRead};

/// ID_ISAR4 (*Instruction Set Attribute Register 4*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdIsar4(pub u32);
impl SysReg for IdIsar4 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for IdIsar4 {}
impl IdIsar4 {
    #[inline]
    /// Reads ID_ISAR4 (*Instruction Set Attribute Register 4*)
    pub fn read() -> IdIsar4 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
