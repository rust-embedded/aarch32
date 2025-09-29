//! Code for managing CNTHP_CTL (*Hyp Physical Counter-timer Control Register (EL2)*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTHP_CTL (*Hyp Physical Counter-timer Control Register (EL2)*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CnthpCtl {
    /// The status of the timer interrupt.
    #[bit(2, r)]
    istatus: bool,
    /// Timer interrupt mask bit.
    ///
    /// * true: masked
    /// * false: not masked
    #[bit(1, rw)]
    imask: bool,
    /// Enables the timer.
    #[bit(0, rw)]
    enable: bool,
}

impl SysReg for CnthpCtl {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 4;
    const CRM: u32 = 2;
    const OP2: u32 = 1;
}

impl SysRegRead for CnthpCtl {}

impl CnthpCtl {
    #[inline]
    /// Reads CNTHP_CTL (*Hyp Physical Counter-timer Control Register (EL2)*)
    pub fn read() -> CnthpCtl {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for CnthpCtl {}

impl CnthpCtl {
    #[inline]
    /// Writes CNTHP_CTL (*Hyp Physical Counter-timer Control Register (EL2)*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }

    /// Modify CNTHP_CTL (*Hyp Physical Counter-timer Control Register (EL2)*)
    #[inline]
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }
}
