//! Code for managing IMP_QOSR (*Quality Of Service Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_QOSR (*Quality Of Service Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpQosr(pub u32);
impl SysReg for ImpQosr {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 1;
    const CRM: u32 = 3;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for ImpQosr {}
impl ImpQosr {
    #[inline]
    /// Reads IMP_QOSR (*Quality Of Service Register*)
    pub fn read() -> ImpQosr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpQosr {}
impl ImpQosr {
    #[inline]
    /// Writes IMP_QOSR (*Quality Of Service Register*)
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
