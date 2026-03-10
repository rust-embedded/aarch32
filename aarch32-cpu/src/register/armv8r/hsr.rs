//! Code for managing HSR (*Hyp Syndrome Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

use arbitrary_int::{u2, u25, u3, u4, u6};

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

impl Hsr {
    /// Get the ISS value from the HSR
    pub fn get_iss(&self) -> Option<Iss> {
        if let Ok(ec) = self.ec() {
            Some(ec.decode_iss(self.iss()))
        } else {
            None
        }
    }
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

/// A decoded ISS
///
/// ISS is a 25 bit field whose meaning varies depending on the value of the EC field.
#[derive(Debug, Clone)]
pub enum Iss {
    Unknown(IssUnknown),
    TrappedWfiWfe(IssTrappedWfiWfe),
    TrappedCp15McrMrc(IssTrappedMcrMrc),
    TrappedCp15McrrMrrc(IssTrappedMcrrMrrc),
    TrappedCp14McrMrc(IssTrappedMcrMrc),
    TrappedLdcStc(IssTrappedLdcStc),
    TrappedFpu(IssTrappedFpu),
    TrappedVmrs(IssTrappedVmrs),
    TrappedCp14McrrMrrc(IssTrappedMcrrMrrc),
    IllegalAArch32Eret,
    Svc(IssCall),
    Hvc(IssCall),
    Smc(IssSmc),
    PrefetchAbortFromLower(IssPrefetchAbort),
    PrefetchAbortFromCurrent(IssPrefetchAbort),
    PcAlignment,
    DataAbortFromLower(IssDataAbort),
    DataAbortFromCurrent(IssDataAbort),
}

impl ExceptionClass {
    pub fn decode_iss(&self, iss: u25) -> Iss {
        match self {
            ExceptionClass::Unknown => Iss::Unknown(IssUnknown(iss.value())),
            ExceptionClass::TrappedWfiWfe => {
                Iss::TrappedWfiWfe(IssTrappedWfiWfe::new_with_raw_value(iss))
            }
            ExceptionClass::TrappedCp15McrMrc => {
                Iss::TrappedCp15McrMrc(IssTrappedMcrMrc::new_with_raw_value(iss))
            }
            ExceptionClass::TrappedCp15McrrMrrc => {
                Iss::TrappedCp15McrrMrrc(IssTrappedMcrrMrrc::new_with_raw_value(iss))
            }
            ExceptionClass::TrappedCp14McrMrc => {
                Iss::TrappedCp14McrMrc(IssTrappedMcrMrc::new_with_raw_value(iss))
            }
            ExceptionClass::TrappedLdcStc => {
                Iss::TrappedLdcStc(IssTrappedLdcStc::new_with_raw_value(iss))
            }
            ExceptionClass::TrappedFpu => Iss::TrappedFpu(IssTrappedFpu::new_with_raw_value(iss)),
            ExceptionClass::TrappedVmrs => Iss::TrappedVmrs(IssTrappedVmrs(iss.value())),
            ExceptionClass::TrappedCp14McrrMrrc => {
                Iss::TrappedCp14McrrMrrc(IssTrappedMcrrMrrc::new_with_raw_value(iss))
            }
            ExceptionClass::IllegalAArch32Eret => Iss::IllegalAArch32Eret,
            ExceptionClass::Svc => Iss::Svc(IssCall::new_with_raw_value(iss)),
            ExceptionClass::Hvc => Iss::Hvc(IssCall::new_with_raw_value(iss)),
            ExceptionClass::Smc => Iss::Smc(IssSmc(iss.value())),
            ExceptionClass::PrefetchAbortFromLower => {
                Iss::PrefetchAbortFromLower(IssPrefetchAbort::new_with_raw_value(iss))
            }
            ExceptionClass::PrefetchAbortFromCurrent => {
                Iss::PrefetchAbortFromCurrent(IssPrefetchAbort::new_with_raw_value(iss))
            }
            ExceptionClass::PcAlignment => Iss::PcAlignment,
            ExceptionClass::DataAbortFromLower => {
                Iss::DataAbortFromLower(IssDataAbort::new_with_raw_value(iss))
            }
            ExceptionClass::DataAbortFromCurrent => {
                Iss::DataAbortFromCurrent(IssDataAbort::new_with_raw_value(iss))
            }
        }
    }
}

/// The ISS field when EC = ExceptionClass::Unknown
///
/// All bits are reserved
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IssUnknown(pub u32);

/// The ISS field when EC = ExceptionClass::TrappedWfiWfe
#[bitbybit::bitfield(u25, debug, defmt_bitfields(feature = "defmt"))]
pub struct IssTrappedWfiWfe {
    /// Condition code valid
    #[bit(24, r)]
    cv: bool,
    /// Condition code
    #[bits(20..=23, r)]
    cc: u4,
    /// Trapped Instruction is WFE
    #[bit(0, r)]
    ti: bool,
}

/// The ISS field when EC = ExceptionClass::TrappedCp15McrMrc or ExceptionClass::TrappedCp14McrMrc
#[bitbybit::bitfield(u25, debug, defmt_bitfields(feature = "defmt"))]
pub struct IssTrappedMcrMrc {
    /// Condition code valid
    #[bit(24, r)]
    cv: bool,
    /// Condition code
    #[bits(20..=23, r)]
    cc: u4,
    /// OPC2 value from instruction
    #[bits(17..=19, r)]
    opc2: u3,
    /// OPC1 value from instruction
    #[bits(14..=16, r)]
    opc1: u3,
    /// CRn value from instruction
    #[bits(10..=13, r)]
    crn: u4,
    /// Rt value from instruction
    #[bits(5..=8, r)]
    rt: u4,
    /// CRm value from instruction
    #[bits(1..=4, r)]
    crm: u4,
    /// Direction (true = read, false = write)
    #[bit(0, r)]
    is_read: bool,
}

/// The ISS field when EC = ExceptionClass::TrappedCp15McrrMrrc or ExceptionClass::TrappedCp14McrrMrrc
#[bitbybit::bitfield(u25, debug, defmt_bitfields(feature = "defmt"))]
pub struct IssTrappedMcrrMrrc {
    /// Condition code valid
    #[bit(24, r)]
    cv: bool,
    /// Condition code
    #[bits(20..=23, r)]
    cc: u4,
    /// OPC2 value from instruction
    #[bits(16..=19, r)]
    opc2: u4,
    /// Rt2 value from instruction
    #[bits(10..=13, r)]
    rt2: u4,
    /// Rt value from instruction
    #[bits(5..=8, r)]
    rt: u4,
    /// CRm value from instruction
    #[bits(1..=4, r)]
    crm: u4,
    /// Direction (true = read, false = write)
    #[bit(0, r)]
    is_read: bool,
}

/// The ISS field when EC = ExceptionClass::TrappedLdcStc
#[bitbybit::bitfield(u25, debug, defmt_bitfields(feature = "defmt"))]
pub struct IssTrappedLdcStc {
    /// Condition code valid
    #[bit(24, r)]
    cv: bool,
    /// Condition code
    #[bits(20..=23, r)]
    cc: u4,
    /// The immediate value from the instruction
    #[bits(12..=19, r)]
    imm8: u8,
    /// Rn value from instruction
    #[bits(5..=8, r)]
    rn: u4,
    /// Whether offset is added (true) or subtracted (false)
    #[bit(4, r)]
    offset: bool,
    /// Addressing Mode
    #[bits(1..=3, r)]
    am: u3,
    /// Direction (true = read, false = write)
    #[bit(0, r)]
    is_read: bool,
}

/// The ISS field when EC = ExceptionClass::TrappedFpu
#[bitbybit::bitfield(u25, debug, defmt_bitfields(feature = "defmt"))]
pub struct IssTrappedFpu {
    /// Condition code valid
    #[bit(24, r)]
    cv: bool,
    /// Condition code
    #[bits(20..=23, r)]
    cc: u4,
    /// Trapped Advanced SIMD
    #[bit(5, r)]
    ta: bool,
    /// CoProc Bits
    #[bits(0..=3, r)]
    coproc: u4,
}

/// The ISS field when EC = ExceptionClass::TrappedVmrs
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IssTrappedVmrs(pub u32);

/// The ISS field when EC = ExceptionClass::Svc
#[bitbybit::bitfield(u25, debug, defmt_bitfields(feature = "defmt"))]
pub struct IssCall {
    /// Immediate value from instruction
    #[bits(0..=15, r)]
    imm16: u16,
}

/// The ISS field when EC = ExceptionClass::Smc
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IssSmc(pub u32);

/// The ISS field when EC = ExceptionClass::PrefetchAbortFromLower or ExceptionClass::PrefetchAbortFromCurrent
#[bitbybit::bitfield(u25, debug, defmt_bitfields(feature = "defmt"))]
pub struct IssPrefetchAbort {
    /// FAR not Valid, for a Synchronous External abort.
    #[bit(10, r)]
    fnv: bool,
    /// External Abort Type.
    ///
    /// External = true, anything else = false
    #[bit(9, r)]
    ea: bool,
    /// Instruction Fault Status Code
    #[bits(0..=5, r)]
    ifsc: u6,
}

/// The ISS field when EC = ExceptionClass::DataAbortFromLower or ExceptionClass::DataAbortFromCurrent
#[bitbybit::bitfield(u25, debug, defmt_bitfields(feature = "defmt"))]
pub struct IssDataAbort {
    /// Instruction Syndrome Valid
    #[bit(24, r)]
    isv: bool,
    /// Syndrome Access Size
    #[bits(22..=23, r)]
    sas: u2,
    /// Syndrome Sign Extend
    #[bit(21, r)]
    sae: bool,
    /// Syndrome Register transfer
    #[bits(16..=19, r)]
    srt: u4,
    /// Acquire/Release
    #[bit(14, r)]
    ar: bool,
    /// FAR not Valid
    #[bit(10, r)]
    fnv: bool,
    /// External Abort Type.
    ///
    /// External = true, anything else = false
    #[bit(9, r)]
    ea: bool,
    /// Cache maintenance
    #[bit(8, r)]
    cm: bool,
    /// Write not Read
    #[bit(6, r)]
    wnr: bool,
    /// Data Fault Status Code
    #[bits(0..=5, r)]
    dfsc: u6,
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
