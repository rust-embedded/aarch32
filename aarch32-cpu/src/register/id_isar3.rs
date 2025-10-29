//! Code for managing ID_ISAR3 (*Instruction Set Attribute Register 3*)

use crate::register::{SysReg, SysRegRead};

/// ID_ISAR3 (*Instruction Set Attribute Register 3*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdIsar3(pub u32);
impl SysReg for IdIsar3 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 3;
}
impl crate::register::SysRegRead for IdIsar3 {}
impl IdIsar3 {
    #[inline]
    /// Reads ID_ISAR3 (*Instruction Set Attribute Register 3*)
    pub fn read() -> IdIsar3 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
