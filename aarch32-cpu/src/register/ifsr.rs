//! Code for managing IFSR (*Instruction Fault Status Register*)

#[allow(unused)]
use arbitrary_int::u4;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IFSR (*Instruction Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v5te")]
pub struct Ifsr {
    /// Which domain was being accessed
    #[bits(4..=7, rw)]
    domain: u4,
    /// Status
    #[bits([0..=3], rw)]
    status: Option<IfsrStatus>,
}

/// IFSR (*Instruction Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v6")]
pub struct Ifsr {
    /// Which domain was being accessed
    #[bits(4..=7, rw)]
    domain: u4,
    /// Status bitfield.
    #[bits([0..=3], rw)]
    status: Option<IfsrStatus>,
}

/// IFSR (*Instruction Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v7-r")]
pub struct Ifsr {
    /// AXI Decode or Slave
    #[bit(12, r)]
    sd: bool,
    /// Which domain was being accessed
    #[bits(4..=7, rw)]
    domain: u4,
    /// Status bitfield.
    #[bits([0..=3, 10], rw)]
    status: Option<IfsrStatus>,
}

/// IFSR (*Instruction Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v7-a")]
pub struct Ifsr {
    /// FAR not Valid
    #[bit(16, rw)]
    fnv: bool,
    /// External Abort type
    #[bit(12, rw)]
    ext: bool,
    /// Status bitfield.
    #[bits([0..=3, 10], rw)]
    status: Option<IfsrStatus>,
}

/// IFSR (*Instruction Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v8-r")]
pub struct Ifsr {
    /// FAR not Valid
    #[bit(16, rw)]
    fnv: bool,
    /// External Abort type
    #[bit(12, rw)]
    ext: bool,
    /// Status bitfield.
    #[bits([0..=5], rw)]
    status: Option<IfsrStatus>,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u4, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[cfg(arm_architecture = "v5te")]
pub enum IfsrStatus {
    Alignment = 1,
    DebugEvent = 2,
    AlignmentAlt = 3,
    TranslationFaultFirstLevel = 5,
    TranslationFaultSecondLevel = 7,
    SyncExtAbort = 8,
    DomainFaultFirstLevel = 9,
    SyncExtAbortAlt = 10,
    DomainFaultSecondLevel = 11,
    SyncExtAbortOnTranslationTableWalkFirstLevel = 12,
    PermissionFaultFirstLevel = 13,
    SyncExtAbortOnTranslationTableWalkSecondLevel = 14,
    PermissionFaultSecondLevel = 15,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u4, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[cfg(arm_architecture = "v6")]
pub enum IfsrStatus {
    Alignment = 1,
    DebugEvent = 2,
    AccessFlagFaultFirstLevel = 3,
    TranslationFaultFirstLevel = 5,
    AccessFlagFaultSecondLevel = 6,
    TranslationFaultSecondLevel = 7,
    SyncExtAbort = 8,
    DomainFaultFirstLevel = 9,
    DomainFaultSecondLevel = 11,
    SyncExtAbortOnTranslationTableWalkFirstLevel = 12,
    PermissionFaultFirstLevel = 13,
    SyncExtAbortOnTranslationTableWalkSecondLevel = 14,
    PermissionFaultSecondLevel = 15,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u5, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[cfg(arm_architecture = "v7-r")]
pub enum IfsrStatus {
    Alignment = 1,
    DebugEvent = 2,
    SyncExtAbort = 8,
    PermissionFaultFirstLevel = 13,
    AsyncExtAbort = 21,
    SyncParityEccError = 25,
    AsyncParityEccError = 24,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u5, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[cfg(arm_architecture = "v7-a")]
pub enum IfsrStatus {
    SyncExtAbortOnTranslationTableWalkFirstLevel = 0b01100,
    SyncExtAbortOnTranslationTableWalkSecondLevel = 0b01110,
    SyncParErrorOnTranslationTableWalkFirstLevel = 0b11100,
    SyncParErrorOnTranslationTableWalkSecondLevel = 0b11110,
    TranslationFaultFirstLevel = 0b00101,
    TranslationFaultSecondLevel = 0b00111,
    AccessFlagFaultFirstLevel = 0b00011,
    AccessFlagFaultSecondLevel = 0b00110,
    DomainFaultFirstLevel = 0b01001,
    DomainFaultSecondLevel = 0b01011,
    PermissionFaultFirstLevel = 0b01101,
    PermissionFaultSecondLevel = 0b01111,
    DebugEvent = 0b00010,
    SyncExtAbort = 0b01000,
    TlbConflictAbort = 0b10000,
    Lockdown = 0b10100,
    CoprocessorAbort = 0b11010,
    SyncParErrorOnMemAccess = 0b11001,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u6, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[cfg(arm_architecture = "v8-r")]
pub enum IfsrStatus {
    Translation = 4,
    Permission = 12,
    SyncExtAbort = 16,
    SyncParityEccError = 24,
    PcAlignment = 33,
    Debug = 34,
}

impl SysReg for Ifsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Ifsr {}

impl Ifsr {
    #[inline]
    /// Reads IFSR (*Instruction Fault Status Register*)
    pub fn read() -> Ifsr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Ifsr {}

impl Ifsr {
    #[inline]
    /// Writes IFSR (*Instruction Fault Status Register*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
