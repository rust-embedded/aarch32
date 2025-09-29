//! Code for managing HSTR (*Hyp System Trap Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HSTR (*Hyp System Trap Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hstr(pub u32);
impl SysReg for Hstr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 3;
}
impl crate::register::SysRegRead for Hstr {}
impl Hstr {
    #[inline]
    /// Reads HSTR (*Hyp System Trap Register*)
    pub fn read() -> Hstr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hstr {}
impl Hstr {
    #[inline]
    /// Writes HSTR (*Hyp System Trap Register*)
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
