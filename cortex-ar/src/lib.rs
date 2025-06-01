//! CPU/peripheral support for Arm Cortex-R

#![no_std]

mod critical_section;

#[cfg(target_arch = "arm")]
pub mod asm;

pub mod cache;
pub mod interrupt;
pub mod mmu;
pub mod register;

#[cfg(any(test, arm_architecture = "v7-r"))]
pub mod pmsav7;

#[cfg(any(test, arm_architecture = "v8-r"))]
pub mod generic_timer;

#[cfg(any(test, arm_architecture = "v8-r"))]
pub mod pmsav8;

/// Generate an SVC call with the given argument.
///
/// Safe to call even in Supervisor (SupervisorCall) mode, as long as your Svc handler
/// saves and restores SPSR_svc correctly.
#[macro_export]
macro_rules! svc {
    ($r0:expr) => {
        unsafe {
            core::arch::asm!("svc {arg}", arg = const $r0, out("lr") _);
        }
    }
}
