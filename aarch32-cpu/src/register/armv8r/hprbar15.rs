//! Code for managing HPRBAR15 (*Hyp Protection Region Base Address Register 15*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR15 (*Hyp Protection Region Base Address Register 15*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hprbar15(pub u32);

impl SysReg for Hprbar15 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 15;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Hprbar15 {}

impl Hprbar15 {
    #[inline]
    /// Reads HPRBAR15 (*Hyp Protection Region Base Address Register 15*)
    pub fn read() -> Hprbar15 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hprbar15 {}

impl Hprbar15 {
    #[inline]
    /// Writes HPRBAR15 (*Hyp Protection Region Base Address Register 15*)
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
