//! Code for managing BPIALL (*Branch Predictor Invalidate All*)

use crate::register::{SysReg, SysRegWrite};

/// BPIALL (*Branch Predictor Invalidate All*)
pub struct BpIAll;

impl SysReg for BpIAll {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 6;
}

impl SysRegWrite for BpIAll {}

impl BpIAll {
    #[inline]
    /// Writes 0 to BPIALL (*Branch Predictor Invalidate All*) to trigger operation
    pub fn write() {
        unsafe { <Self as SysRegWrite>::write_raw(0) }
    }
}
