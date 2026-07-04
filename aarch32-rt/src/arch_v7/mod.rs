//! ASM routines for for Armv7 and higher

mod abort;
mod hvc;
mod interrupt;
mod svc;
mod undefined;

#[cfg(any(
    arm_architecture = "v7-a",
    all(arm_architecture = "v8-r", not(feature = "el2-mode")),
))]
mod boot_from_el2;

#[cfg(arm_architecture = "v7-r")]
mod boot_from_el1;
