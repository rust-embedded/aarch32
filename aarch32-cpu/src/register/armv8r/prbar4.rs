//! Code for managing PRBAR4 (*Protection Region Base Address Register 4*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR4 (*Protection Region Base Address Register 4*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar4(pub u32);

impl SysReg for Prbar4 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 10;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Prbar4 {}

impl Prbar4 {
    #[inline]
    /// Reads PRBAR4 (*Protection Region Base Address Register 4*)
    pub fn read() -> Prbar4 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Prbar4 {}

impl Prbar4 {
    #[inline]
    /// Writes PRBAR4 (*Protection Region Base Address Register 4*)
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
