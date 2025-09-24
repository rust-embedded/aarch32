//! Code for managing HMPUIR (*Hyp MPU Type Register*)

use crate::register::{SysReg, SysRegRead};

/// HMPUIR (*Hyp MPU Type Register*)
#[bitbybit::bitfield(u32)]
pub struct Hmpuir {
    /// The number of EL2 MPU regions implemented
    #[bits(0..=7, r)]
    region: u8,
}

impl SysReg for Hmpuir {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Hmpuir {}
impl Hmpuir {
    #[inline]
    /// Reads HMPUIR (*Hyp MPU Type Register*)
    pub fn read() -> Hmpuir {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}
