//! Code for managing IMP_DCERR0 (*Data Cache Error Record Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_DCERR0 (*Data Cache Error Record Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpDcerr0(pub u32);

impl SysReg for ImpDcerr0 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 2;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for ImpDcerr0 {}

impl ImpDcerr0 {
    #[inline]
    /// Reads IMP_DCERR0 (*Data Cache Error Record Register 0*)
    pub fn read() -> ImpDcerr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for ImpDcerr0 {}

impl ImpDcerr0 {
    #[inline]
    /// Writes IMP_DCERR0 (*Data Cache Error Record Register 0*)
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
