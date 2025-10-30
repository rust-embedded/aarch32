//! Code for HVBAR (*Hyp Vector Base Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HVBAR (*Hyp Vector Base Address Register*)
///
/// There is no `modify` method because this register holds a single 32-bit address.
///
/// This is only available in EL2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hvbar(pub u32);

impl SysReg for Hvbar {
    const CP: u32 = 15;
    const CRN: u32 = 12;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl SysRegRead for Hvbar {}

impl SysRegWrite for Hvbar {}

impl Hvbar {
    /// Read HVBAR (*Hyp Vector Base Address Register*)
    #[inline]
    pub fn read() -> Hvbar {
        // Safety: Reading this register has no side-effects and is atomic
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }

    /// Write HVBAR (*Hyp Vector Base Address Register*)
    ///
    /// # Safety
    ///
    /// You must supply a correctly-aligned address of a valid Arm AArch32
    /// Vector Table.
    #[inline]
    pub unsafe fn write(value: Self) {
        // Safety: Writing this register is atomic
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0 as u32);
        }
    }
}
