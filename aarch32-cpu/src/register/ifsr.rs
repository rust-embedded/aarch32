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
    /// PC Alignment Fault
    Alignment = 1,
    /// Debug Exception
    DebugEvent = 2,
    /// Alternate value for PC Alignment Fault
    AlignmentAlt = 3,
    /// Translation fault, level 1
    TranslationFaultFirstLevel = 5,
    /// Translation fault, level 2
    TranslationFaultSecondLevel = 7,
    /// Synchronous External abort
    SyncExtAbort = 8,
    /// Domain fault, level 1
    DomainFaultFirstLevel = 9,
    /// Alternate value for Synchronous External abort
    SyncExtAbortAlt = 10,
    /// Domain fault, level 2
    DomainFaultSecondLevel = 11,
    /// Synchronous External abort, on translation table walk, level 1
    SyncExtAbortOnTranslationTableWalkFirstLevel = 12,
    /// Permission fault, level 1
    PermissionFaultFirstLevel = 13,
    /// Synchronous External abort, on translation table walk, level 2
    SyncExtAbortOnTranslationTableWalkSecondLevel = 14,
    /// Permission fault, level 2
    PermissionFaultSecondLevel = 15,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u4, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[cfg(arm_architecture = "v6")]
pub enum IfsrStatus {
    /// PC Alignment Fault
    Alignment = 1,
    /// Debug Exception
    DebugEvent = 2,
    /// Access Flag fault, level 1
    AccessFlagFaultFirstLevel = 3,
    /// Translation fault, level 1
    TranslationFaultFirstLevel = 5,
    /// Access Flag fault, level 2
    AccessFlagFaultSecondLevel = 6,
    /// Translation fault, level 2
    TranslationFaultSecondLevel = 7,
    /// Synchronouse External Abort
    SyncExtAbort = 8,
    /// Domain fault, level 1
    DomainFaultFirstLevel = 9,
    /// Domain fault, level 2
    DomainFaultSecondLevel = 11,
    /// Synchronous External abort, on translation table walk, level 1
    SyncExtAbortOnTranslationTableWalkFirstLevel = 12,
    /// Permission fault, level 1
    PermissionFaultFirstLevel = 13,
    /// Synchronous External abort, on translation table walk, level 2
    SyncExtAbortOnTranslationTableWalkSecondLevel = 14,
    /// Permission fault, level 2
    PermissionFaultSecondLevel = 15,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u5, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[cfg(arm_architecture = "v7-r")]
pub enum IfsrStatus {
    /// PC Alignment Fault
    Alignment = 1,
    /// Debug Exception
    DebugEvent = 2,
    /// Synchronous External abort
    SyncExtAbort = 8,
    /// Permission fault, level 1
    PermissionFaultFirstLevel = 13,
    /// Asynchronous External abort
    AsyncExtAbort = 21,
    /// Synchronous parity or ECC error
    SyncParityEccError = 25,
    /// asynchronous parity or ECC error
    AsyncParityEccError = 24,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u5, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[cfg(arm_architecture = "v7-a")]
pub enum IfsrStatus {
    /// Synchronous External abort, on translation table walk, level 1
    SyncExtAbortOnTranslationTableWalkFirstLevel = 0b01100,
    /// Synchronous External abort, on translation table walk, level 2
    SyncExtAbortOnTranslationTableWalkSecondLevel = 0b01110,
    /// Synchronous parity or ECC error on memory access, on translation table walk, level 1
    SyncParErrorOnTranslationTableWalkFirstLevel = 0b11100,
    /// Synchronous parity or ECC error on memory access, on translation table walk, level 2
    SyncParErrorOnTranslationTableWalkSecondLevel = 0b11110,
    /// Translation fault, level 1
    TranslationFaultFirstLevel = 0b00101,
    /// Translation fault, level 2
    TranslationFaultSecondLevel = 0b00111,
    /// Access flag fault, level 1
    AccessFlagFaultFirstLevel = 0b00011,
    /// Access flag fault, level 2
    AccessFlagFaultSecondLevel = 0b00110,
    /// Domain fault, level 1
    DomainFaultFirstLevel = 0b01001,
    /// Domain fault, level 2
    DomainFaultSecondLevel = 0b01011,
    /// Permission fault, level 1
    PermissionFaultFirstLevel = 0b01101,
    /// Permission fault, level 2
    PermissionFaultSecondLevel = 0b01111,
    /// Debug exception
    DebugEvent = 0b00010,
    /// Synchronous External abort
    SyncExtAbort = 0b01000,
    /// TLB conflict abort
    TlbConflictAbort = 0b10000,
    /// IMPLEMENTATION DEFINED fault (Lockdown fault)
    Lockdown = 0b10100,
    /// Co-Processor Abort
    CoprocessorAbort = 0b11010,
    /// Synchronous parity or ECC error on memory access, not on translation table walk
    SyncParErrorOnMemAccess = 0b11001,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u6, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[cfg(arm_architecture = "v8-r")]
pub enum IfsrStatus {
    /// Translation fault
    Translation = 4,
    /// Permission fault
    Permission = 12,
    /// Synchronous External abort
    SyncExtAbort = 16,
    /// Synchronous parity or ECC error on memory access
    SyncParityEccError = 24,
    /// PC alignment fault
    PcAlignment = 33,
    /// Debug exception
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
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
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
