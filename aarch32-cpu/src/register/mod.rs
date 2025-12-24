//! Access registers in Armv7-R and Armv8-R

pub mod actlr;
pub mod actlr2;
pub mod adfsr;
pub mod aidr;
pub mod aifsr;
pub mod amair0;
pub mod amair1;
pub mod bpiall;
pub mod ccsidr;
pub mod clidr;
pub mod contextidr;
pub mod cpacr;
pub mod cpsr;
pub mod csselr;
pub mod ctr;
pub mod dc_sw_ops;
pub mod dccimvac;
pub mod dccisw;
pub mod dccmvac;
pub mod dccmvau;
pub mod dccsw;
pub mod dcimvac;
pub mod dcisw;
pub mod dfar;
pub mod dfsr;
pub mod dlr;
pub mod dracr;
pub mod drbar;
pub mod drsr;
pub mod dspsr;
pub mod fcseidr;
pub mod icc_pmr;
pub mod id_afr0;
pub mod id_dfr0;
pub mod id_isar0;
pub mod id_isar1;
pub mod id_isar2;
pub mod id_isar3;
pub mod id_isar4;
pub mod id_isar5;
pub mod id_mmfr0;
pub mod id_mmfr1;
pub mod id_mmfr2;
pub mod id_mmfr3;
pub mod id_mmfr4;
pub mod id_pfr0;
pub mod id_pfr1;
pub mod ifar;
pub mod ifsr;
pub mod imp;
pub mod iracr;
pub mod irbar;
pub mod irsr;
pub mod mair0;
pub mod mair1;
pub mod midr;
pub mod mpidr;
pub mod mpuir;
pub mod nsacr;
pub mod par;
pub mod pmccfiltr;
pub mod pmccntr;
pub mod pmceid0;
pub mod pmceid1;
pub mod pmcntenclr;
pub mod pmcntenset;
pub mod pmcr;
pub mod pmevcntr0;
pub mod pmevcntr1;
pub mod pmevcntr2;
pub mod pmevcntr3;
pub mod pmevtyper0;
pub mod pmevtyper1;
pub mod pmevtyper2;
pub mod pmevtyper3;
pub mod pmintenclr;
pub mod pmintenset;
pub mod pmovsr;
pub mod pmovsset;
pub mod pmselr;
pub mod pmswinc;
pub mod pmuserenr;
pub mod pmxevcntr;
pub mod pmxevtyper;
pub mod revidr;
pub mod rgnr;
pub mod rvbar;
pub mod sctlr;
pub mod tcmtr;
pub mod tlbiall;
pub mod tlbtr;
pub mod tpidrprw;
pub mod tpidruro;
pub mod tpidrurw;
pub mod vmpidr;
pub mod vpidr;
pub mod vsctlr;

pub use actlr::Actlr;
pub use actlr2::Actlr2;
pub use adfsr::Adfsr;
pub use aidr::Aidr;
pub use aifsr::Aifsr;
pub use amair0::Amair0;
pub use amair1::Amair1;
pub use bpiall::BpIAll;
pub use ccsidr::Ccsidr;
pub use clidr::Clidr;
pub use contextidr::Contextidr;
pub use cpacr::Cpacr;
pub use cpsr::Cpsr;
pub use csselr::Csselr;
pub use ctr::Ctr;
pub use dccimvac::Dccimvac;
pub use dccisw::Dccisw;
pub use dccmvac::Dccmvac;
pub use dccmvau::Dccmvau;
pub use dccsw::Dccsw;
pub use dcimvac::Dcimvac;
pub use dcisw::Dcisw;
pub use dfar::Dfar;
pub use dfsr::Dfsr;
pub use dlr::Dlr;
pub use dracr::Dracr;
pub use drbar::Drbar;
pub use drsr::Drsr;
pub use dspsr::Dspsr;
pub use fcseidr::Fcseidr;
pub use icc_pmr::IccPmr;
pub use id_afr0::IdAfr0;
pub use id_dfr0::IdDfr0;
pub use id_isar0::IdIsar0;
pub use id_isar1::IdIsar1;
pub use id_isar2::IdIsar2;
pub use id_isar3::IdIsar3;
pub use id_isar4::IdIsar4;
pub use id_isar5::IdIsar5;
pub use id_mmfr0::IdMmfr0;
pub use id_mmfr1::IdMmfr1;
pub use id_mmfr2::IdMmfr2;
pub use id_mmfr3::IdMmfr3;
pub use id_mmfr4::IdMmfr4;
pub use id_pfr0::IdPfr0;
pub use id_pfr1::IdPfr1;
pub use ifar::Ifar;
pub use ifsr::Ifsr;
pub use iracr::Iracr;
pub use irbar::Irbar;
pub use irsr::Irsr;
pub use mair0::Mair0;
pub use mair1::Mair1;
pub use midr::Midr;
pub use mpidr::Mpidr;
pub use mpuir::Mpuir;
pub use nsacr::Nsacr;
pub use par::Par;
pub use pmccfiltr::Pmccfiltr;
pub use pmccntr::Pmccntr;
pub use pmceid0::Pmceid0;
pub use pmceid1::Pmceid1;
pub use pmcntenclr::Pmcntenclr;
pub use pmcntenset::Pmcntenset;
pub use pmcr::Pmcr;
pub use pmevcntr0::Pmevcntr0;
pub use pmevcntr1::Pmevcntr1;
pub use pmevcntr2::Pmevcntr2;
pub use pmevcntr3::Pmevcntr3;
pub use pmevtyper0::Pmevtyper0;
pub use pmevtyper1::Pmevtyper1;
pub use pmevtyper2::Pmevtyper2;
pub use pmevtyper3::Pmevtyper3;
pub use pmintenclr::Pmintenclr;
pub use pmintenset::Pmintenset;
pub use pmovsr::Pmovsr;
pub use pmovsset::Pmovsset;
pub use pmselr::Pmselr;
pub use pmswinc::Pmswinc;
pub use pmuserenr::Pmuserenr;
pub use pmxevcntr::Pmxevcntr;
pub use pmxevtyper::Pmxevtyper;
pub use revidr::Revidr;
pub use rgnr::Rgnr;
pub use rvbar::Rvbar;
pub use sctlr::Sctlr;
pub use tcmtr::Tcmtr;
pub use tlbiall::TlbIAll;
pub use tlbtr::Tlbtr;
pub use tpidrprw::Tpidrprw;
pub use tpidruro::Tpidruro;
pub use tpidrurw::Tpidrurw;
pub use vmpidr::Vmpidr;
pub use vpidr::Vpidr;
pub use vsctlr::Vsctlr;

#[cfg(any(test, doc, arm_architecture = "v8-r"))]
pub mod armv8r;
#[cfg(any(test, doc, arm_architecture = "v8-r"))]
pub use armv8r::*;

pub use imp::*;

/// Describes a 32-bit System Register
pub trait SysReg {
    /// Which Co-Processor (e.g. 15 for CP15) is this register in?
    const CP: u32;
    /// Which CRn argument (e.g. 0 for c0) accesses this register
    const CRN: u32;
    /// Which OP1 argument accesses this register
    const OP1: u32;
    /// Which CRm argument (e.g. 1 for c1) accesses this register
    const CRM: u32;
    /// Which OP2 argument accesses this register
    const OP2: u32;
}

/// 32-bit Readable System Registers
pub trait SysRegRead: SysReg {
    /// Read a value from this 32-bit register
    ///
    /// # Safety
    ///
    /// You need to read the Architecture Reference Manual because this read
    /// may have side-effects.
    #[cfg_attr(not(feature = "check-asm"), inline)]
    #[cfg_attr(
        any(
            arm_architecture = "v4t",
            arm_architecture = "v5te",
            arm_architecture = "v6"
        ),
        instruction_set(arm::a32)
    )]
    unsafe fn read_raw() -> u32 {
        let r: u32;
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!(
                "mrc p{cp}, {op1}, {reg}, c{crn}, c{crm}, {op2}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                reg = out(reg) r,
                crn = const Self::CRN,
                crm = const Self::CRM,
                op2 = const Self::OP2,
                options(nomem, nostack, preserves_flags)
            );
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        r
    }
}

/// Writable 32-bit System Registers
pub trait SysRegWrite: SysReg {
    /// Write a value to this 32-bit register
    ///
    /// # Safety
    ///
    /// You need to read the Architecture Reference Manual to verify that you are
    /// writing valid data here.
    #[cfg_attr(not(feature = "check-asm"), inline)]
    #[cfg_attr(
        any(
            arm_architecture = "v4t",
            arm_architecture = "v5te",
            arm_architecture = "v6"
        ),
        instruction_set(arm::a32)
    )]
    unsafe fn write_raw(_value: u32) {
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!(
                "mcr p{cp}, {op1}, {reg}, c{crn}, c{crm}, {op2}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                reg = in(reg) _value,
                crn = const Self::CRN,
                crm = const Self::CRM,
                op2 = const Self::OP2,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}

/// Describes a 64-bit System Register
pub trait SysReg64 {
    /// Which Co-Processor (e.g. 15 for CP15) is this register in?
    const CP: u32;
    /// Which OP1 argument accesses this register
    const OP1: u32;
    /// Which CRm argument (e.g. 1 for c1) accesses this register
    const CRM: u32;
}

/// 64-bit Readable System Registers
pub trait SysRegRead64: SysReg64 {
    /// Read a value from this 64-bit register
    ///
    /// # Safety
    ///
    /// You need to read the Architecture Reference Manual because this read
    /// may have side-effects.
    #[inline]
    unsafe fn read_raw() -> u64 {
        let r_lo: u32;
        let r_hi: u32;
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!(
                "mrrc p{cp}, {op1}, {rt}, {rt2}, c{crm}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                rt = out(reg) r_lo,
                rt2 = out(reg) r_hi,
                crm = const Self::CRM,
                options(nomem, nostack, preserves_flags)
            );
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r_lo = 0;
            r_hi = 0;
        }
        ((r_hi as u64) << 32) | (r_lo as u64)
    }
}

/// Writable 64-bit System Registers
pub trait SysRegWrite64: SysReg64 {
    /// Write a value to this 64-bit register
    ///
    /// # Safety
    ///
    /// You need to read the Architecture Reference Manual to verify that you are
    /// writing valid data here.
    #[inline]
    unsafe fn write_raw(_value: u64) {
        #[cfg(target_arch = "arm")]
        unsafe {
            let r_lo = _value as u32;
            let r_hi = (_value >> 32) as u32;
            core::arch::asm!(
                "mcrr p{cp}, {op1}, {rt}, {rt2}, c{crm}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                rt = in(reg) r_lo,
                rt2 = in(reg) r_hi,
                crm = const Self::CRM,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}
