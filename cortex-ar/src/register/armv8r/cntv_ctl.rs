//! Code for managing CNTV_CTL (*Virtual Counter-timer Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTV_CTL (*Virtual Counter-timer Control Register*)
#[bitbybit::bitfield(u32, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CntvCtl {
    /// The status of the timer interrupt.
    #[bits(2..=2, r)]
    istatus: bool,
    /// Timer interrupt mask bit.
    ///
    /// * true: masked
    /// * false: not masked
    #[bits(1..=1, rw)]
    imask: bool,
    /// Enables the timer.
    #[bits(0..=0, rw)]
    enable: bool,
}

impl core::fmt::Debug for CntvCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CntvCtl")
            .field("istatus", &self.istatus())
            .field("imask", &self.imask())
            .field("enable", &self.enable())
            .finish()
    }
}

impl SysReg for CntvCtl {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 3;
    const OP2: u32 = 1;
}

impl SysRegRead for CntvCtl {}

impl CntvCtl {
    #[inline]
    /// Reads CNTV_CTL (*Virtual Counter-timer Control Register*)
    pub fn read() -> CntvCtl {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for CntvCtl {}

impl CntvCtl {
    #[inline]
    /// Writes CNTV_CTL (*Virtual Counter-timer Control Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }

    #[inline]
    /// Modifies CNTV_CTL (*Virtual Counter-timer Control Register*)
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }
}
