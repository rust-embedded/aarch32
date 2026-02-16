//! Code for managing HSR (*Hyp Syndrome Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

use arbitrary_int::u25;

/// HSR (*Hyp Syndrome Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct Hsr {
    /// Exception Class.
    ///
    /// Indicates the reason for the exception that this register holds
    /// information about.
    #[bits(26..=31, rw)]
    ec: Option<ExceptionClass>,
    /// Instruction length bit.
    ///
    /// Indicates the size of the instruction that has been trapped to Hyp mode.
    #[bit(25, rw)]
    il: InstructionLength,
    /// Instruction Specific Syndrome.
    ///
    /// Architecturally, this field can be defined independently for each
    /// defined Exception class. However, in practice, some ISS encodings are
    /// used for more than one Exception class.
    #[bits(0..=24, rw)]
    iss: u25,
}

#[bitbybit::bitenum(u6, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum ExceptionClass {
    Unknown = 0b00_0000,
    TrappedWfiWfe = 0b00_0001,
    TrappedCp15McrMrc = 0b00_0011,
    TrappedCp15McrrMrrc = 0b00_0100,
    TrappedCp14McrMrc = 0b00_0101,
    TrappedLdcStc = 0b00_0110,
    TrappedFpu = 0b00_0111,
    TrappedVmrs = 0b00_1000,
    TrappedCp14McrrMrrc = 0b00_1100,
    IllegalAArch32Eret = 0b00_1110,
    Svc = 0b01_0001,
    Hvc = 0b01_0010,
    Smc = 0b01_0011,
    PrefetchAbortFromLower = 0b10_0000,
    PrefetchAbortFromCurrent = 0b10_0001,
    PcAlignment = 0b10_0010,
    DataAbortFromLower = 0b10_0100,
    DataAbortFromCurrent = 0b10_0101,
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum InstructionLength {
    SixteenBit = 0b0,
    ThirtyTwoBit = 0b1,
}

impl SysReg for Hsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 4;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Hsr {}

impl Hsr {
    #[inline]
    /// Reads HSR (*Hyp Syndrome Register*)
    pub fn read() -> Hsr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hsr {}

impl Hsr {
    #[inline]
    /// Writes HSR (*Hyp Syndrome Register*)
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
