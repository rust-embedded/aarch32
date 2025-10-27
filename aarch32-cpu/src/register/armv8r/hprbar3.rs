//! Code for managing HPRBAR3 (*Hyp Protection Region Base Address Register 3*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR3 (*Hyp Protection Region Base Address Register 3*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar3(pub u32);

impl SysReg for Hprbar3 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 9;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Hprbar3 {}

impl Hprbar3 {
    #[inline]
    /// Reads HPRBAR3 (*Hyp Protection Region Base Address Register 3*)
    pub fn read() -> Hprbar3 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprbar3 {}

impl Hprbar3 {
    #[inline]
    /// Writes HPRBAR3 (*Hyp Protection Region Base Address Register 3*)
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
