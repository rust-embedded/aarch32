//! Code for managing PRBAR5 (*Protection Region Base Address Register 5*)

use crate::register::{Prbar, SysReg, SysRegRead, SysRegWrite};

/// PRBAR5 (*Protection Region Base Address Register 5*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prbar5(pub u32);

impl SysReg for Prbar5 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 10;
    const OP2: u32 = 4;
}

impl SysRegRead for Prbar5 {}

impl Prbar5 {
    #[inline]
    /// Reads PRBAR5 (*Protection Region Base Address Register 5*)
    pub fn read() -> Prbar {
        unsafe { Prbar::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for Prbar5 {}

impl Prbar5 {
    #[inline]
    /// Writes PRBAR5 (*Protection Region Base Address Register 5*)
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
