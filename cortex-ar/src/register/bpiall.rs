//! BPIALL: Invalidate all entries from branch predictors

use crate::register::SysReg;

/// BPIALL: Invalidate all entries from branch predictors
pub struct BpIAll;

impl SysReg for BpIAll {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 6;
}

impl crate::register::SysRegWrite for BpIAll {}

impl BpIAll {
    #[inline]
    pub fn write() {
        unsafe { <Self as crate::register::SysRegWrite>::write_raw(0) }
    }
}
