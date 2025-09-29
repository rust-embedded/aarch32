//! Code for managing MPIDR (*Multiprocessor Affinity Register*)

use crate::register::{SysReg, SysRegRead};

/// MPIDR (*Multiprocessor Affinity Register*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mpidr(pub u32);
impl SysReg for Mpidr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Mpidr {}
impl Mpidr {
    #[inline]
    /// Reads MPIDR (*Multiprocessor Affinity Register*)
    pub fn read() -> Mpidr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
