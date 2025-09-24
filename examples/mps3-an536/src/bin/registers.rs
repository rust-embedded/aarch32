//! Registers example for Arm Cortex-R

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r_rt::entry;

// pull in our library
use mps3_an536 as _;

use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-r-rt`.
#[entry]
fn main() -> ! {
    chip_info();
    #[cfg(arm_architecture = "v7-r")]
    mpu_pmsa_v7();
    #[cfg(arm_architecture = "v8-r")]
    mpu_pmsa_v8();
    test_changing_sctlr();
    semihosting::process::exit(0);
}

fn chip_info() {
    println!("{:?}", cortex_ar::register::Midr::read());
    println!("{:?}", cortex_ar::register::Cpsr::read());
    #[cfg(arm_architecture = "v8-r")]
    {
        println!("{:?}", cortex_ar::register::ImpCbar::read());
        println!("{:?}", cortex_ar::register::Vbar::read());
        // This only works in EL2 and start-up put us in EL1
        // println!("{:?}", cortex_ar::register::Hvbar::read());
    }
}

#[cfg(arm_architecture = "v7-r")]
fn mpu_pmsa_v7() {
    use cortex_ar::{
        pmsav7::{CacheablePolicy, Config, MemAttr, Mpu, Region, RegionSize},
        register::Mpuir,
    };

    // How many regions?
    let mpuir = Mpuir::read();
    println!("PMSA-v7 MPUIR: {:?}", mpuir);

    // Make an MPU driver
    let mut mpu = unsafe { Mpu::new() };

    // Look at the existing config
    for idx in 0..mpu.num_iregions() {
        if let Some(region) = mpu.get_iregion(idx) {
            println!("IRegion {}: {:?}", idx, region);
        }
    }
    for idx in 0..mpu.num_dregions() {
        if let Some(region) = mpu.get_dregion(idx) {
            println!("DRegion {}: {:?}", idx, region);
        }
    }

    // Load a config (but don't enable it)
    mpu.configure(&Config {
        background_config: true,
        dregions: &[Region {
            base: 0x2000_0000 as *mut u8,
            size: RegionSize::_16M,
            subregion_mask: 0x00,
            enabled: true,
            no_exec: false,
            mem_attr: MemAttr::Cacheable {
                inner: CacheablePolicy::WriteThroughNoWriteAllocate,
                outer: CacheablePolicy::NonCacheable,
                shareable: true,
            },
        }],
        iregions: &[],
    })
    .unwrap();

    // Look at the new config
    for idx in 0..mpu.num_dregions() {
        if let Some(region) = mpu.get_dregion(idx) {
            println!("DRegion {}: {:?}", idx, region);
        }
    }
}

#[cfg(arm_architecture = "v8-r")]
fn mpu_pmsa_v8() {
    use cortex_ar::{
        pmsav8::{
            Cacheable, El1AccessPerms, El1Config, El1Mpu, El1Region, El1Shareability, MemAttr,
            RwAllocPolicy,
        },
        register::Mpuir,
    };

    // How many regions?
    let mpuir = Mpuir::read();
    println!("PMSA-v8 MPUIR: {:?}", mpuir);

    // Make an MPU driver
    let mut mpu = unsafe { El1Mpu::new() };

    // Look at the existing config
    for idx in 0..mpu.num_regions() {
        if let Some(region) = mpu.get_region(idx) {
            println!("Region {}: {:?}", idx, region);
        }
    }

    // Load a config (but don't enable it)
    mpu.configure(&El1Config {
        background_config: true,
        regions: &[El1Region {
            range: 0x0000_0000 as *mut u8..=0x3FFF_FFFF as *mut u8,
            shareability: El1Shareability::OuterShareable,
            access: El1AccessPerms::ReadWrite,
            no_exec: true,
            mair: 0,
            enable: true,
        }],
        memory_attributes: &[MemAttr::NormalMemory {
            outer: Cacheable::WriteThroughNonTransient(RwAllocPolicy::RW),
            inner: Cacheable::WriteThroughNonTransient(RwAllocPolicy::RW),
        }],
    })
    .unwrap();

    // Look at the new config
    for idx in 0..mpu.num_regions() {
        if let Some(region) = mpu.get_region(idx) {
            println!("Region {}: {:?}", idx, region);
        }
    }
}

fn test_changing_sctlr() {
    println!(
        "{:?} before setting C, I and Z",
        cortex_ar::register::Sctlr::read()
    );
    cortex_ar::register::Sctlr::modify(|w| {
        w.set_c(true);
        w.set_i(true);
        w.set_z(true);
    });
    println!("{:?} after", cortex_ar::register::Sctlr::read());
}
