//! DCCMVAU (*Clean Data Cache Or Unified Cache Line by MVA to Point of Unification.*)
use crate::register::{SysReg, SysRegWrite};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dccmvau(pub u32);

impl Dccmvau {
    #[inline]
    pub const fn new(addr: u32) -> Self {
        Self(addr)
    }
}

impl SysReg for Dccmvau {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 11;
    const OP2: u32 = 1;
}

impl crate::register::SysRegWrite for Dccmvau {}

impl Dccmvau {
    #[inline]
    /// Writes DCCMVAU (*Clean Data Cache Or Unified Cache Line by MVA to Point of Unification.*)
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
