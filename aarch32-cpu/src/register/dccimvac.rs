//! Code for managing DCCIMVAC (*Data Cache line Clean and Invalidate by VA to PoC Register*)

use crate::register::{SysReg, SysRegWrite};

/// DCCIMVAC (*Data Cache line Clean and Invalidate by VA to PoC Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dccimvac(pub u32);

impl Dccimvac {
    #[inline]
    /// Create a new DCCIMVAC containing the address to clean and invalidate
    pub const fn new(addr: u32) -> Self {
        Self(addr)
    }
}

impl SysReg for Dccimvac {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 1;
}

impl SysRegWrite for Dccimvac {}

impl Dccimvac {
    #[inline]
    /// Writes DCCIMVAC (*Data Cache line Clean and Invalidate by VA to PoC Register*)
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
