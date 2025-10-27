//! Code for managing DLR (*Debug Link Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DLR (*Debug Link Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dlr(pub u32);

impl SysReg for Dlr {
    const CP: u32 = 15;
    const CRN: u32 = 4;
    const OP1: u32 = 3;
    const CRM: u32 = 5;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Dlr {}

impl Dlr {
    #[inline]
    /// Reads DLR (*Debug Link Register*)
    pub fn read() -> Dlr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Dlr {}

impl Dlr {
    #[inline]
    /// Writes DLR (*Debug Link Register*)
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
