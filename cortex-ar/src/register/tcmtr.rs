//! Code for managing TCMTR (*TCM Type Register*)

use crate::register::{SysReg, SysRegRead};

/// TCMTR (*TCM Type Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tcmtr(pub u32);
impl SysReg for Tcmtr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for Tcmtr {}
impl Tcmtr {
    #[inline]
    /// Reads TCMTR (*TCM Type Register*)
    pub fn read() -> Tcmtr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
