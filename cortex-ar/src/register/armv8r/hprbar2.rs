//! Code for managing HPRBAR2 (*Hyp Protection Region Base Address Register 2*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR2 (*Hyp Protection Region Base Address Register 2*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar2(pub u32);
impl SysReg for Hprbar2 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 9;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Hprbar2 {}
impl Hprbar2 {
    #[inline]
    /// Reads HPRBAR2 (*Hyp Protection Region Base Address Register 2*)
    pub fn read() -> Hprbar2 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprbar2 {}
impl Hprbar2 {
    #[inline]
    /// Writes HPRBAR2 (*Hyp Protection Region Base Address Register 2*)
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
