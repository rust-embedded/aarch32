//! Code for managing DMP_ICERR1 (*Data Cache Error Record Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DMP_ICERR1 (*Data Cache Error Record Register 1*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpDcerr1(pub u32);
impl SysReg for ImpDcerr1 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 2;
    const CRM: u32 = 1;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for ImpDcerr1 {}
impl ImpDcerr1 {
    #[inline]
    /// Reads DMP_ICERR1 (*Data Cache Error Record Register 1*)
    pub fn read() -> ImpDcerr1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpDcerr1 {}
impl ImpDcerr1 {
    #[inline]
    /// Writes DMP_ICERR1 (*Data Cache Error Record Register 1*)
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
