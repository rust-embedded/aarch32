//! Code for managing IMP_FLASHERR1 (*Flash Error Record Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_FLASHERR1 (*Flash Error Record Register 1*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpFlasherr1(pub u32);
impl SysReg for ImpFlasherr1 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 2;
    const CRM: u32 = 3;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for ImpFlasherr1 {}
impl ImpFlasherr1 {
    #[inline]
    /// Reads IMP_FLASHERR1 (*Flash Error Record Register 1*)
    pub fn read() -> ImpFlasherr1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpFlasherr1 {}
impl ImpFlasherr1 {
    #[inline]
    /// Writes IMP_FLASHERR1 (*Flash Error Record Register 1*)
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
