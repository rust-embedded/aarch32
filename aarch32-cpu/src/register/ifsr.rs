//! Code for managing IFSR (*Instruction Fault Status Register*)

use arbitrary_int::{prelude::*, u4, u5};

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IFSR (*Instruction Fault Status Register*)
#[bitbybit::bitfield(u32, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ifsr {
    /// External abort qualifier
    #[bit(12, rw)]
    ext: bool,
    #[bits(4..=7, rw)]
    domain: u4,
    /// Status bitfield.
    #[bits([0..=3, 10], rw)]
    status_raw: u5,
}

/// Fault status register enumeration for IFSR, which is also part of the DFSR
#[derive(Debug, Copy, Clone, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum FsrStatus {
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

impl Ifsr {
    pub fn status(&self) -> Result<FsrStatus, u8> {
        let status = self.status_raw().as_u8();
        FsrStatus::try_from(status).map_err(|_| status)
    }
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

impl core::fmt::Debug for Ifsr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "IFSR {{ ext={} Domain={:#06b} Status={:#07b} }}",
            self.ext(),
            self.domain(),
            self.status_raw()
        )
    }
}
