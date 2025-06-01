//! TLBIALL (*TLB Invalidate All Register*)

use crate::register::SysReg;

/// TLBIALL (*TLB Invalidate All Register*)
pub struct TlbIAll;

impl SysReg for TlbIAll {
    const CP: u32 = 15;
    const CRN: u32 = 8;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 0;
}

impl crate::register::SysRegWrite for TlbIAll {}

impl TlbIAll {
    #[inline]
    pub fn write() {
        unsafe { <Self as crate::register::SysRegWrite>::write_raw(0) }
    }
}
