//! Code for managing DFSR (*Data Fault Status Register*)

#[cfg(arm_architecture = "v6")]
use arbitrary_int::u4;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DFSR (*Data Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v5te")]
pub struct Dfsr {
    /// Status
    #[bits([0..=3], rw)]
    status: Option<DfsrStatus>,
}

/// DFSR (*Data Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v6")]
pub struct Dfsr {
    /// Write not Read
    #[bit(11, rw)]
    wnr: bool,
    /// Domain
    #[bits(4..=7, rw)]
    domain: u4,
    /// Status bitfield.
    #[bits([0..=3, 10], rw)]
    status: Option<DfsrStatus>,
}

/// DFSR (*Data Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v7-r")]
pub struct Dfsr {
    /// Status bitfield.
    #[bits([0..=3, 10], rw)]
    status: Option<DfsrStatus>,
}

/// DFSR (*Instruction Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v7-a")]
pub struct Dfsr {
    /// FAR not Valid
    #[bit(16, rw)]
    fnv: bool,
    /// Cache manintenance fault
    #[bit(13, rw)]
    cm: bool,
    /// External Abort type
    #[bit(12, rw)]
    ext: bool,
    /// Write not Read
    #[bit(11, rw)]
    wnr: bool,
    /// Status bitfield.
    #[bits([0..=5], rw)]
    status: Option<DfsrStatus>,
}

/// DFSR (*Data Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg(arm_architecture = "v8-r")]
pub struct Dfsr {
    /// FAR not Valid
    #[bit(16, rw)]
    fnv: bool,
    /// Cache manintenance fault
    #[bit(13, rw)]
    cm: bool,
    /// External Abort type
    #[bit(12, rw)]
    ext: bool,
    /// Write not Read
    #[bit(11, rw)]
    wnr: bool,
    /// Status bitfield.
    #[bits([0..=5], rw)]
    status: Option<DfsrStatus>,
}

/// Fault status register enumeration for DFSR
#[bitbybit::bitenum(u4, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v5te")]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DfsrStatus {
    /// Alignment fault
    AlignmentFault = 1,
    /// Debug Exception
    Debug = 2,
    /// Alternate value for Alignment fault
    AlignmentAlt = 3,
    /// Translation fault, level 1
    TranslationFaultFirstLevel = 5,
    /// Translation fault, level 2
    TranslationFaultSecondLevel = 7,
    /// Synchronous External Abort
    SyncExtAbort = 8,
    /// Domain fault, level 1
    DomainFaultFirstLevel = 9,
    /// Alternate value for Synchronous External Abort
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

/// Fault status register enumeration for DFSR
#[bitbybit::bitenum(u5, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v6")]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DfsrStatus {
    /// Alignment fault
    AlignmentFault = 0b00001,
    /// Debug event fault
    Debug = 0b00010,
    /// Access Flag fault on Section
    AccessFlagFaultFirstLevel = 0b00011,
    /// Cache maintenance operation fault[2]
    CacheMaintenance = 0b00100,
    /// Translation fault on Section
    TranslationFaultFirstLevel = 0b00101,
    /// Access Flag fault on Page
    AccessFlagFaultSecondLevel = 0b00110,
    /// Translation fault on Page
    TranslationFaultSecondLevel = 0b00111,
    /// Precise External Abort
    PreciseExternalAbort = 0b01000,
    /// Domain fault on Section
    DomainFaultFirstLevel = 0b01001,
    /// Domain fault on Page
    DomainFaultSecondLevel = 0b01011,
    /// External abort on translation, first level
    SyncExtAbortOnTranslationTableWalkFirstLevel = 0b01100,
    /// Permission fault on Section
    PermissionFaultFirstLevel = 0b01101,
    /// External abort on translation, second level
    SyncExtAbortOnTranslationTableWalkSecondLevel = 0b01110,
    /// Permission fault on Page
    PermissionFaultSecondLevel = 0b01111,
    /// Imprecise External Abort
    ImpreciseExtAbort = 0b10110,
}

/// Fault status register enumeration for DFSR
#[bitbybit::bitenum(u5, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v7-r")]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DfsrStatus {
    /// Alignment fault
    AlignmentFault = 1,
    /// Debug exception
    Debug = 2,
    /// Translation fault
    Translation = 4,
    /// Permission fault
    Permission = 12,
    /// Synchronous external abort, other than synchronous parity or ECC error
    SError = 16,
    /// SError interrupt
    SErrorInterrupt = 17,
    /// Synchronous parity or ECC error on memory access
    SyncParErrorOnMemAccess = 24,
    /// SError parity or ECC error on memory access
    SErrorParityEccError = 25,
}

/// Fault status register enumeration for DFSR
#[bitbybit::bitenum(u6, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v7-a")]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DfsrStatus {
    /// Alignment fault.
    AlignmentFault = 0b00001,
    /// Debug exception.
    Debug = 0b00010,
    /// Access flag fault, level 1
    AccessFlagFaultFirstLevel = 0b00011,
    /// Fault on instruction cache maintenance.
    CacheMaintenance = 0b00100,
    /// Translation fault, level 1
    TranslationFaultFirstLevel = 0b00101,
    /// Access flag fault, level 2.
    AccessFlagFaultSecondLevel = 0b00110,
    /// Translation fault, level 2.
    TranslationFaultSecondLevel = 0b00111,
    /// Synchronous External abort, not on translation table walk.
    SyncExtAbort = 0b01000,
    /// Domain fault, level 1
    DomainFaultFirstLevel = 0b01001,
    /// Domain fault, level 2.
    DomainFaultSecondLevel = 0b01011,
    /// Synchronous External abort, on translation table walk, level 1
    SyncExtAbortOnTranslationTableWalkFirstLevel = 0b01100,
    /// Permission fault, level 1
    PermissionFaultFirstLevel = 0b01101,
    /// Synchronous External abort, on translation table walk, level 2.
    SyncExtAbortOnTranslationTableWalkSecondLevel = 0b01110,
    /// Permission fault, level 2.
    PermissionFaultSecondLevel = 0b01111,
    /// TLB conflict abort.
    TldConflictAbort = 0b10000,
    /// SError exception.
    SError = 0b10110,
    /// SError exception, from a parity or ECC error on memory access.
    SErrorParityEccError = 0b11000,
    /// Synchronous parity or ECC error on memory access, not on translation table walk.
    SyncParErrorOnMemAccess = 0b11001,
    /// Synchronous parity or ECC error on translation table walk, level 1
    SyncParErrorOnTranslationTableWalkFirstLevel = 0b11100,
    /// Synchronous parity or ECC error on translation table walk, level 2.
    SyncParErrorOnTranslationTableWalkSecondLevel = 0b11110,
}

/// Fault status register enumeration for DFSR
#[bitbybit::bitenum(u6, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(arm_architecture = "v8-r")]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DfsrStatus {
    /// Translation fault
    Translation = 4,
    /// Permission fault
    Permission = 12,
    /// Synchronous external abort, other than synchronous parity or ECC error
    SyncExtAbort = 16,
    /// SError interrupt
    SErrorInterrupt = 17,
    /// Synchronous parity or ECC error on memory access
    SyncParityEccError = 24,
    /// SError parity or ECC error on memory access
    SErrorParityEccError = 25,
    /// Alignment fault
    AlignmentFault = 33,
    /// Debug exception
    Debug = 34,
}

impl SysReg for Dfsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Dfsr {}

impl Dfsr {
    #[inline]
    /// Reads DFSR (*Data Fault Status Register*)
    pub fn read() -> Dfsr {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Dfsr {}

impl Dfsr {
    #[inline]
    /// Writes DFSR (*Data Fault Status Register*)
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
