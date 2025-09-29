//! Code for managing PRBAR (*Protection Region Base Address Register*)

use arbitrary_int::u26;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// Shareability for an MPU Region
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[bitbybit::bitenum(u2, exhaustive = true)]
pub enum Shareability {
    /// Non-shareable
    NonShareable = 0b00,
    /// Reserved
    Reserved = 0b01,
    /// Outer-Shareable
    OuterShareable = 0b10,
    /// Inner-Shareable
    InnerShareable = 0b11,
}

/// Access Permissions for an MPU Region
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[bitbybit::bitenum(u2, exhaustive = true)]
pub enum AccessPerms {
    /// Read-Write at EL1, No access at EL0
    ReadWriteNoEL0 = 0b00,
    /// Read-Write at EL1 and EL0
    ReadWrite = 0b01,
    /// Read-Only at EL1, No access at EL0
    ReadOnlyNoEL0 = 0b10,
    /// Read-Only at EL1 and EL0
    ReadOnly = 0b11,
}

/// PRBAR (*Protection Region Base Address Register*)
#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
pub struct Prbar {
    /// Address
    #[bits(6..=31, rw)]
    base: u26,
    /// Shareability
    #[bits(3..=4, rw)]
    shareability: Shareability,
    /// Access Permissions
    #[bits(1..=2, rw)]
    access_perms: AccessPerms,
    /// Execute Never
    #[bits(0..=0, rw)]
    nx: bool,
}

impl SysReg for Prbar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 3;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Prbar {}
impl Prbar {
    #[inline]
    /// Reads PRBAR (*Protection Region Base Address Register*)
    pub fn read() -> Prbar {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar {}
impl Prbar {
    #[inline]
    /// Writes PRBAR (*Protection Region Base Address Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
