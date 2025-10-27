//! Code for managing PMOVSR (*Performance Monitor Overflow Flag Status Clear Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMOVSR (*Performance Monitor Overflow Flag Status Clear Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pmovsr(pub u32);

impl SysReg for Pmovsr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 3;
}

impl crate::register::SysRegRead for Pmovsr {}

impl Pmovsr {
    #[inline]
    /// Reads PMOVSR (*Performance Monitor Overflow Flag Status Clear Register*)
    pub fn read() -> Pmovsr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Pmovsr {}

impl Pmovsr {
    #[inline]
    /// Writes PMOVSR (*Performance Monitor Overflow Flag Status Clear Register*)
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
