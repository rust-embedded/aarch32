//! Code for managing IMP_TCMERR0 (*TCM Error Record Register 0*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_TCMERR0 (*TCM Error Record Register 0*)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImpTcmerr0(pub u32);
impl SysReg for ImpTcmerr0 {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 2;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for ImpTcmerr0 {}
impl ImpTcmerr0 {
    #[inline]
    /// Reads IMP_TCMERR0 (*TCM Error Record Register 0*)
    pub fn read() -> ImpTcmerr0 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpTcmerr0 {}
impl ImpTcmerr0 {
    #[inline]
    /// Writes IMP_TCMERR0 (*TCM Error Record Register 0*)
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
