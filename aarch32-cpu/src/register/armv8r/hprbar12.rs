//! Code for managing HPRBAR12 (*Hyp Protection Region Base Address Register 12*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR12 (*Hyp Protection Region Base Address Register 12*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar12(pub u32);
impl SysReg for Hprbar12 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 14;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Hprbar12 {}
impl Hprbar12 {
    #[inline]
    /// Reads HPRBAR12 (*Hyp Protection Region Base Address Register 12*)
    pub fn read() -> Hprbar12 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprbar12 {}
impl Hprbar12 {
    #[inline]
    /// Writes HPRBAR12 (*Hyp Protection Region Base Address Register 12*)
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
