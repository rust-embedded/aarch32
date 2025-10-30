//! Code for managing HPRBAR9 (*Hyp Protection Region Base Address Register 9*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR9 (*Hyp Protection Region Base Address Register 9*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar9(pub u32);

impl SysReg for Hprbar9 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 12;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Hprbar9 {}

impl Hprbar9 {
    #[inline]
    /// Reads HPRBAR9 (*Hyp Protection Region Base Address Register 9*)
    pub fn read() -> Hprbar9 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprbar9 {}

impl Hprbar9 {
    #[inline]
    /// Writes HPRBAR9 (*Hyp Protection Region Base Address Register 9*)
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
