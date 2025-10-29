//! Code for managing IMP_INTMONR (*Interrupt Monitoring Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_INTMONR (*Interrupt Monitoring Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpIntmonr(pub u32);
impl SysReg for ImpIntmonr {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 1;
    const CRM: u32 = 3;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for ImpIntmonr {}
impl ImpIntmonr {
    #[inline]
    /// Reads IMP_INTMONR (*Interrupt Monitoring Register*)
    pub fn read() -> ImpIntmonr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpIntmonr {}
impl ImpIntmonr {
    #[inline]
    /// Writes IMP_INTMONR (*Interrupt Monitoring Register*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
