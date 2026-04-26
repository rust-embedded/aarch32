//! Code for managing DACR (*Domain Access Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DACR (*Domain Access Control Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dacr {
    /// An array of bits controlling access to each of the 16 domains
    #[bits(0..=1, rw)]
    d: [DomainAccess; 16],
}

/// Domain Access Permissions
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bitbybit::bitenum(u2, exhaustive = true)]
pub enum DomainAccess {
    /// No access. Any access to the domain generates a Domain fault.
    NoAccess = 0b00,
    /// Cient. Accesses are checked against the permission bits in the translation tables.
    Client = 0b01,
    /// Reserved
    Reserved = 0b10,
    /// Manager. Accesses are not checked against the permission bits in the translation tables.
    Manager = 0b11,
}

impl SysReg for Dacr {
    const CP: u32 = 15;
    const CRN: u32 = 3;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl SysRegRead for Dacr {}

impl Dacr {
    #[inline]
    /// Reads DACR (*Domain Access Control Register*)
    pub fn read() -> Dacr {
        // Safety: it's OK to set bits with no accessors specified
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for Dacr {}

impl Dacr {
    #[inline]
    /// Writes DACR (*Domain Access Control Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }

    /// Modify DACR (*Domain Access Control Register*)
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
