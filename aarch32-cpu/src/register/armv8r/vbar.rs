//! Code for managing VBAR (*Vector Base Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// VBAR (*Vector Base Address Register*)
///
/// There is no `modify` method because this register holds a single 32-bit address.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vbar(pub *mut u32);

impl SysReg for Vbar {
    const CP: u32 = 15;
    const CRN: u32 = 12;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl SysRegRead for Vbar {}

impl SysRegWrite for Vbar {}

impl Vbar {
    /// Read VBAR (*Vector Base Address Register*)
    #[inline]
    pub fn read() -> Vbar {
        // Safety: Reading this register has no side-effects and is atomic
        unsafe { Self(<Self as SysRegRead>::read_raw() as *mut u32) }
    }

    /// Write VBAR (*Vector Base Address Register*)
    ///
    /// # Safety
    ///
    /// You must supply a correctly-aligned address of a valid Arm Cortex-R
    /// Vector Table.
    #[inline]
    pub unsafe fn write(value: Self) {
        // Safety: Writing this register is atomic
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0 as u32);
        }
    }
}

impl core::fmt::Debug for Vbar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VBAR {{ {:010p} }}", self.0)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Vbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VBAR {{ 0x{=usize:08x} }}", self.0 as usize)
    }
}
