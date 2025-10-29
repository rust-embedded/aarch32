//! Code for managing IMP_BUSTIMEOUTR (*Bus Timeout Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_BUSTIMEOUTR (*Bus Timeout Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpBustimeoutr(pub u32);
impl SysReg for ImpBustimeoutr {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 1;
    const CRM: u32 = 3;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for ImpBustimeoutr {}
impl ImpBustimeoutr {
    #[inline]
    /// Reads IMP_BUSTIMEOUTR (*Bus Timeout Register*)
    pub fn read() -> ImpBustimeoutr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpBustimeoutr {}
impl ImpBustimeoutr {
    #[inline]
    /// Writes IMP_BUSTIMEOUTR (*Bus Timeout Register*)
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
