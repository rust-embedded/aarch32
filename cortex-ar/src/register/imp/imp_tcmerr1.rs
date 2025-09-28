//! Code for managing IMP_TCMERR1 (*TCM Error Record Register 1*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_TCMERR1 (*TCM Error Record Register 1*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpTcmerr1(pub u32);
impl SysReg for ImpTcmerr1 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 2;
    const CRM: u32 = 2;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for ImpTcmerr1 {}
impl ImpTcmerr1 {
    #[inline]
    /// Reads IMP_TCMERR1 (*TCM Error Record Register 1*)
    pub fn read() -> ImpTcmerr1 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpTcmerr1 {}
impl ImpTcmerr1 {
    #[inline]
    /// Writes IMP_TCMERR1 (*TCM Error Record Register 1*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
