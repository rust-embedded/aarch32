//! Code for managing DFSR (*Data Fault Status Register*)

use arbitrary_int::{prelude::*, u4, u5};

use crate::register::{SysReg, SysRegRead, SysRegWrite};

use super::ifsr::FsrStatus;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum DfsrStatus {
    AlignmentFault = 0b00001,
    FaultOnInstructionCacheMaintenance = 0b00100,
    AsyncExternalAbort = 0b10110,
    AsyncParityErrorOnMemAccess = 0b11000,
    CommonFsr(FsrStatus),
}

impl TryFrom<u8> for DfsrStatus {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00001 => Ok(DfsrStatus::AlignmentFault),
            0b00100 => Ok(DfsrStatus::FaultOnInstructionCacheMaintenance),
            0b10110 => Ok(DfsrStatus::AsyncExternalAbort),
            0b11000 => Ok(DfsrStatus::AsyncParityErrorOnMemAccess),
            _ => FsrStatus::try_from(value)
                .map(DfsrStatus::CommonFsr)
                .map_err(|_| value),
        }
    }
}

/// DFSR (*Data Fault Status Register*)
#[bitbybit::bitfield(u32, defmt_bitfields(feature = "defmt"))]
pub struct Dfsr {
    /// External abort qualifier
    #[bit(12, rw)]
    ext: bool,
    /// Write Not Read bit.
    #[bit(11, rw)]
    wnr: bool,
    #[bits(4..=7, rw)]
    domain: u4,
    /// Status bitfield.
    #[bits([0..=3, 10], rw)]
    status_raw: u5,
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
    pub fn status(&self) -> Result<DfsrStatus, u8> {
        let status = self.status_raw().as_u8();
        DfsrStatus::try_from(status).map_err(|_| status)
    }

    #[inline]
    /// Reads DFSR (*Data Fault Status Register*)
    pub fn read() -> Dfsr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
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

impl core::fmt::Debug for Dfsr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "DFSR {{ ext={} wnr={} Domain={:#06b} Status={:#07b} }}",
            self.ext(),
            self.wnr(),
            self.domain(),
            self.status_raw()
        )
    }
}
