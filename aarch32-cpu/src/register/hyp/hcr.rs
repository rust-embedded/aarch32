//! Code for managing HCR (*Hyp Configuration Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HCR (*Hyp Configuration Register*)
#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hcr {
    /// TCPAC - Traps EL1 accesses to the CPACR to Hyp mode
    #[bit(31, rw)]
    tcpac: bool,
    /// TRVM - Trap Reads of Memory controls
    #[bit(30, rw)]
    trvm: bool,
    /// HCD - HVC instruction disable
    #[bit(29, rw)]
    hcd: bool,
    /// TGE - Trap General Exceptions from EL0
    #[bit(27, rw)]
    tge: bool,
    /// TVM - Trap Memory controls
    #[bit(26, rw)]
    tvm: bool,
    /// TPU - Trap cache maintenance instructions that operate to the Point of Unification
    #[bit(24, rw)]
    tpu: bool,
    /// TPC - Trap data or unified cache maintenance instructions that operate to the Point of Coherency
    #[bit(23, rw)]
    tpc: bool,
    /// TSW - Trap data or unified cache maintenance instructions that operate by Set/Way
    #[bit(22, rw)]
    tsw: bool,
    /// TAC - Trap Auxiliary Control Registers
    #[bit(21, rw)]
    tac: bool,
    /// TIDCP - Trap IMPLEMENTATION DEFINED functionality
    #[bit(20, rw)]
    tidcp: bool,
    /// TID3 - Trap ID group 3
    #[bit(18, rw)]
    tid3: bool,
    /// TID2 - Trap ID group 2
    #[bit(17, rw)]
    tid2: bool,
    /// TID1 - Trap ID group 1
    #[bit(16, rw)]
    tid1: bool,
    /// TID0 - Trap ID group 0
    #[bit(15, rw)]
    tid0: bool,
    /// TWE - Traps EL0 and EL1 execution of WFE instructions to Hyp mode
    #[bit(14, rw)]
    twe: bool,
    /// TWI - Traps EL0 and EL1 execution of WFI instructions to Hyp mode
    #[bit(13, rw)]
    twi: bool,
    /// DC - Default Cacheability
    #[bit(12, rw)]
    dc: bool,
    /// BSU - Barrier Shareability upgrade.
    #[bits(10..=11, rw)]
    bsu: Bsu,
    /// FB - Force broadcast
    #[bit(9, rw)]
    fb: bool,
    /// VA - Virtual SError interrupt exception
    #[bit(8, rw)]
    va: bool,
    /// VI - Virtual IRQ exception
    #[bit(7, rw)]
    vi: bool,
    /// VF - Virtual FIQ exception
    #[bit(6, rw)]
    vf: bool,
    /// AMO - SError interrupt Mask Override
    #[bit(5, rw)]
    amo: bool,
    /// IMO - IRQ Mask Override
    #[bit(4, rw)]
    imo: bool,
    /// FMO - FIQ Mask Override
    #[bit(3, rw)]
    fmo: bool,
    /// SWIO - Set/Way Invalidation Override
    #[bit(1, rw)]
    swio: bool,
    /// VM - Virtualization enable
    #[bit(0, rw)]
    vm: bool,
}

/// Barrier Shareability upgrade
///
/// This field determines the minimum Shareability domain that is applied to any
/// barrier instruction executed from EL1 or EL0
#[bitbybit::bitenum(u2, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum Bsu {
    NoEffect = 0b00,
    InnerShareable = 0b01,
    OuterShareable = 0b10,
    FullSystem = 0b11,
}

impl SysReg for Hcr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Hcr {}

impl Hcr {
    #[inline]
    /// Reads HCR (*Hyp Configuration Register*)
    pub fn read() -> Hcr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Hcr {}

impl Hcr {
    #[inline]
    /// Writes HCR (*Hyp Configuration Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }

    #[inline]
    /// Modify HCR (*Hyp Configuration Register*)
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }
}
