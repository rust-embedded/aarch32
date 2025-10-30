//! Code for managing IMP_PINOPTR (*Pin Options Register*)

use crate::register::{SysReg, SysRegRead};

/// IMP_PINOPTR (*Pin Options Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpPinoptr(pub u32);

impl SysReg for ImpPinoptr {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 7;
}

impl crate::register::SysRegRead for ImpPinoptr {}

impl ImpPinoptr {
    #[inline]
    /// Reads IMP_PINOPTR (*Pin Options Register*)
    pub fn read() -> ImpPinoptr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
