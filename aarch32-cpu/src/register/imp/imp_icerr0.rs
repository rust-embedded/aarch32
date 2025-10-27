//! Code for managing IMP_ICERR0 (*Instruction Cache Error Record Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_ICERR0 (*Instruction Cache Error Record Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpIcerr0(pub u32);

impl SysReg for ImpIcerr0 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 2;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for ImpIcerr0 {}

impl ImpIcerr0 {
    #[inline]
    /// Reads IMP_ICERR0 (*Instruction Cache Error Record Register 0*)
    pub fn read() -> ImpIcerr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for ImpIcerr0 {}

impl ImpIcerr0 {
    #[inline]
    /// Writes IMP_ICERR0 (*Instruction Cache Error Record Register 0*)
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
