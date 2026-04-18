//! Code for managing HCPTR (*Hyp Architectural Feature Trap Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HCPTR (*Hyp Architectural Feature Trap Register*)
#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hcptr {
    /// TCPAC - Traps EL1 accesses to the CPACR to Hyp mode
    #[bit(31, rw)]
    tcpac: bool,
    /// TTA - Traps System register accesses to all implemented trace registers to Hyp mode
    #[bit(20, rw)]
    tta: bool,
    /// TASE - Traps execution of Advanced SIMD instructions to Hyp mode when the value of HCPTR.TCP10 is 0.
    #[bit(15, rw)]
    tase: bool,
    /// TCP - Trap accesses to Advanced SIMD and floating-point functionality to Hyp mode
    #[bit(10, rw)]
    tcp: bool,
}

impl SysReg for Hcptr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 2;
}

impl crate::register::SysRegRead for Hcptr {}

impl Hcptr {
    #[inline]
    /// Reads HCPTR (*Hyp Architectural Feature Trap Register*)
    pub fn read() -> Hcptr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }

    /// Modify HCPTR (*Hyp Architectural Feature Trap Register*)
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

impl crate::register::SysRegWrite for Hcptr {}

impl Hcptr {
    #[inline]
    /// Writes HCPTR (*Hyp Architectural Feature Trap Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
