//! Code for managing IMP_PERIPHPREGIONR (*Peripheral Port Region Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_PERIPHPREGIONR (*Peripheral Port Region Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpPeriphpregionr(pub u32);

impl SysReg for ImpPeriphpregionr {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for ImpPeriphpregionr {}

impl ImpPeriphpregionr {
    #[inline]
    /// Reads IMP_PERIPHPREGIONR (*Peripheral Port Region Register*)
    pub fn read() -> ImpPeriphpregionr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for ImpPeriphpregionr {}

impl ImpPeriphpregionr {
    #[inline]
    /// Writes IMP_PERIPHPREGIONR (*Peripheral Port Region Register*)
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
