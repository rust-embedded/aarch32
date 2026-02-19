//! Code for managing PRBAR2 (*Protection Region Base Address Register 2*)

use crate::register::{Prbar, SysReg, SysRegRead, SysRegWrite};

/// PRBAR2 (*Protection Region Base Address Register 2*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar2(pub u32);

impl SysReg for Prbar2 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 9;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Prbar2 {}

impl Prbar2 {
    #[inline]
    /// Reads PRBAR0 (*Protection Region Base Address Register 2*)
    pub fn read() -> Prbar {
        unsafe { Prbar::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prbar2 {}

impl Prbar2 {
    #[inline]
    /// Writes PRBAR2 (*Protection Region Base Address Register 2*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Prbar) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
