//! Code for managing IMP_BUILDOPTR (*Build Options Register*)

use crate::register::{SysReg, SysRegRead};

/// IMP_BUILDOPTR (*Build Options Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpBuildoptr(pub u32);
impl SysReg for ImpBuildoptr {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for ImpBuildoptr {}
impl ImpBuildoptr {
    #[inline]
    /// Reads IMP_BUILDOPTR (*Build Options Register*)
    pub fn read() -> ImpBuildoptr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
