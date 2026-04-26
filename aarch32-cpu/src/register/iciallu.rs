//! Code for managing ICIALLU (*Invalidate entire instruction cache to PoU Register*)
//!
//! Starting with ARMv6, the type of cache can be determined from the System Coprocessor register 0,
//! and controlled through registers 1, 7 and 9. In earlier architecture variants, it is
//! IMPLEMENTATION DEFINED whether this instruction is supported.
use crate::register::{SysReg, SysRegWrite};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// ICIALLU (*Invalidate entire instruction cache to PoU Register*)
pub struct Iciallu;

impl SysReg for Iciallu {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 0;
}

impl SysRegWrite for Iciallu {}

impl Iciallu {
    #[inline]
    /// Writes 0 to ICIALLU (*Invalidate entire instruction cache to PoU Register*) to trigger operation
    pub fn write() {
        unsafe {
            <Self as SysRegWrite>::write_raw(0);
        }
    }
}
