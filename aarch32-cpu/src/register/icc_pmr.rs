//! Code for managing ICC_PMR (*Interrupt Controller Interrupt Priority Mask Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// ICC_PMR (*Interrupt Controller Interrupt Priority Mask Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IccPmr(pub u32);

impl SysReg for IccPmr {
    const CP: u32 = 15;
    const CRN: u32 = 4;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for IccPmr {}

impl IccPmr {
    #[inline]
    /// Reads ICC_PMR (*Interrupt Controller Interrupt Priority Mask Register*)
    pub fn read() -> IccPmr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for IccPmr {}

impl IccPmr {
    #[inline]
    /// Writes ICC_PMR (*Interrupt Controller Interrupt Priority Mask Register*)
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
