//! Code for managing IMP_FLASHERR0 (*Flash Error Record Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_FLASHERR0 (*Flash Error Record Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpFlasherr0(pub u32);
impl SysReg for ImpFlasherr0 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 2;
    const CRM: u32 = 3;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for ImpFlasherr0 {}
impl ImpFlasherr0 {
    #[inline]
    /// Reads IMP_FLASHERR0 (*Flash Error Record Register 0*)
    pub fn read() -> ImpFlasherr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpFlasherr0 {}
impl ImpFlasherr0 {
    #[inline]
    /// Writes IMP_FLASHERR0 (*Flash Error Record Register 0*)
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
