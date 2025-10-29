//! Code for managing HPRBAR14 (*Hyp Protection Region Base Address Register 14*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR14 (*Hyp Protection Region Base Address Register 14*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar14(pub u32);
impl SysReg for Hprbar14 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 15;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Hprbar14 {}
impl Hprbar14 {
    #[inline]
    /// Reads HPRBAR14 (*Hyp Protection Region Base Address Register 14*)
    pub fn read() -> Hprbar14 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprbar14 {}
impl Hprbar14 {
    #[inline]
    /// Writes HPRBAR14 (*Hyp Protection Region Base Address Register 14*)
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
