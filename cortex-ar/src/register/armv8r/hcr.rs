//! Code for managing HCR (*Hyp Configuration Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HCR (*Hyp Configuration Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hcr(pub u32);
impl SysReg for Hcr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Hcr {}
impl Hcr {
    #[inline]
    /// Reads HCR (*Hyp Configuration Register*)
    pub fn read() -> Hcr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hcr {}
impl Hcr {
    #[inline]
    /// Writes HCR (*Hyp Configuration Register*)
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
