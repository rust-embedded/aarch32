//! Code for managing HPRBAR1 (*Hyp Protection Region Base Address Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR1 (*Hyp Protection Region Base Address Register 1*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar1(pub u32);
impl SysReg for Hprbar1 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 8;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Hprbar1 {}
impl Hprbar1 {
    #[inline]
    /// Reads HPRBAR1 (*Hyp Protection Region Base Address Register 1*)
    pub fn read() -> Hprbar1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprbar1 {}
impl Hprbar1 {
    #[inline]
    /// Writes HPRBAR1 (*Hyp Protection Region Base Address Register 1*)
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
