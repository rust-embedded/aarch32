//! DCCIMVAC (*Clean And Invalidate Data Cache Or Unified Cache Line by MVA to Point of Coherence.*)
use crate::register::{SysReg, SysRegWrite};

pub struct Dccimvac(pub u32);

impl Dccimvac {
    #[inline]
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

impl crate::register::SysRegWrite for Dccimvac {}

impl Dccimvac {
    #[inline]
    /// Writes DCCIMVAC (*Clean And Invalidate Data Cache Or Unified Cache Line by MVA to Point of Coherence.*)
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
