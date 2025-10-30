//! Code for managing PRSELR (*Protection Region Selection Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRSELR (*Protection Region Selection Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prselr(pub u32);

impl SysReg for Prselr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Prselr {}

impl Prselr {
    #[inline]
    /// Reads PRSELR (*Protection Region Selection Register*)
    pub fn read() -> Prselr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prselr {}

impl Prselr {
    #[inline]
    /// Writes PRSELR (*Protection Region Selection Register*)
    ///
    /// Controls what appears in PRLAR and PRBAR
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
