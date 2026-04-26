//! Code for managing TLBIALL (*TLB Invalidate All Register*)

use crate::register::{SysReg, SysRegWrite};

/// TLBIALL (*TLB Invalidate All Register*)
pub struct TlbIAll;

impl SysReg for TlbIAll {
    const CP: u32 = 15;
    const CRN: u32 = 8;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 0;
}

impl SysRegWrite for TlbIAll {}

impl TlbIAll {
    #[inline]
    /// Writes 0 to TLBIALL (*TLB Invalidate All Register*) to trigger operation
    pub fn write() {
        unsafe { <Self as SysRegWrite>::write_raw(0) }
    }
}
