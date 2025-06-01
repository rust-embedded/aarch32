//! Code for managing CCSIDR (*Current Cache Size ID Register*)

use crate::register::{SysReg, SysRegRead};
use arbitrary_int::{u10, u15, u3};

/// CCSIDR (*Current Cache Size ID Register*)
#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct Ccsidr {
    #[bit(31, rw)]
    write_through: bool,
    #[bit(30, rw)]
    write_back: bool,
    #[bit(29, rw)]
    read_alloc: bool,
    #[bit(28, rw)]
    write_alloc: bool,
    #[bits(13..=27, rw)]
    num_sets: u15,
    #[bits(3..=12, rw)]
    associativity: u10,
    #[bits(0..=2, rw)]
    line_size: u3,
}

impl SysReg for Ccsidr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 1;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Ccsidr {}

impl Ccsidr {
    #[inline]
    /// Reads CCSIDR (*Current Cache Size ID Register*)
    pub fn read() -> Ccsidr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}
