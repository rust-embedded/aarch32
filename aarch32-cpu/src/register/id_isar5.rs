//! Code for managing ID_ISAR5 (*Instruction Set Attribute Register 5*)

use crate::register::{SysReg, SysRegRead};

/// ID_ISAR5 (*Instruction Set Attribute Register 5*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdIsar5(pub u32);

impl SysReg for IdIsar5 {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for IdIsar5 {}

impl IdIsar5 {
    #[inline]
    /// Reads ID_ISAR5 (*Instruction Set Attribute Register 5*)
    pub fn read() -> IdIsar5 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
