//! Code for managing DSPSR (*Debug Saved Program Status Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DSPSR (*Debug Saved Program Status Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dspsr(pub u32);

impl SysReg for Dspsr {
    const CP: u32 = 15;
    const CRN: u32 = 4;
    const OP1: u32 = 3;
    const CRM: u32 = 5;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Dspsr {}

impl Dspsr {
    #[inline]
    /// Reads DSPSR (*Debug Saved Program Status Register*)
    pub fn read() -> Dspsr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Dspsr {}

impl Dspsr {
    #[inline]
    /// Writes DSPSR (*Debug Saved Program Status Register*)
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
