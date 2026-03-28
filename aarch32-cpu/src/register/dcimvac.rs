//! Code for managing DCIMVAC (*Data Cache line Invalidate by VA to PoC Register*)

use crate::register::{SysReg, SysRegWrite};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// DCIMVAC (*Data Cache line Invalidate by VA to PoC Register*)
pub struct Dcimvac(pub u32);

impl Dcimvac {
    #[inline]
    /// Create a new DCIMVAC value, given an address
    pub const fn new(addr: u32) -> Self {
        Self(addr)
    }
}

impl SysReg for Dcimvac {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 1;
}

impl crate::register::SysRegWrite for Dcimvac {}

impl Dcimvac {
    #[inline]
    /// Writes DCIMVAC (*Data Cache line Invalidate by VA to PoC Register*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register. Generally, the address passed
    /// to the write call should be aligned to the cache line size.
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
