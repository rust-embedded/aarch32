//! Code for managing HPRBAR4 (*Hyp Protection Region Base Address Register 4*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR4 (*Hyp Protection Region Base Address Register 4*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar4(pub u32);
impl SysReg for Hprbar4 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 10;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Hprbar4 {}
impl Hprbar4 {
    #[inline]
    /// Reads HPRBAR4 (*Hyp Protection Region Base Address Register 4*)
    pub fn read() -> Hprbar4 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprbar4 {}
impl Hprbar4 {
    #[inline]
    /// Writes HPRBAR4 (*Hyp Protection Region Base Address Register 4*)
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
