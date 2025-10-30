//! Code for managing HCR2 (*Hyp Configuration Register 2*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HCR2 (*Hyp Configuration Register 2*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hcr2(pub u32);

impl SysReg for Hcr2 {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Hcr2 {}

impl Hcr2 {
    #[inline]
    /// Reads HCR2 (*Hyp Configuration Register 2*)
    pub fn read() -> Hcr2 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hcr2 {}

impl Hcr2 {
    #[inline]
    /// Writes HCR2 (*Hyp Configuration Register 2*)
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
