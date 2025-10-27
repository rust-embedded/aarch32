//! Code for managing VMPIDR (*Virtualization Multiprocessor ID Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// VMPIDR (*Virtualization Multiprocessor ID Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vmpidr(pub u32);

impl SysReg for Vmpidr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 5;
}

impl crate::register::SysRegRead for Vmpidr {}

impl Vmpidr {
    #[inline]
    /// Reads VMPIDR (*Virtualization Multiprocessor ID Register*)
    pub fn read() -> Vmpidr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Vmpidr {}

impl Vmpidr {
    #[inline]
    /// Writes VMPIDR (*Virtualization Multiprocessor ID Register*)
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
