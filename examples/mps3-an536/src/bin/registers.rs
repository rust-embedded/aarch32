//! Registers example for Arm Cortex-R

#![no_std]
#![no_main]

use aarch32_rt::entry;
use mps3_an536 as _;
use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch32-rt`.
#[entry]
fn main() -> ! {
    chip_info();
    #[cfg(arm_architecture = "v7-r")]
    mpu_pmsa_v7();
    #[cfg(arm_architecture = "v8-r")]
    mpu_pmsa_v8();
    test_changing_sctlr();
    mps3_an536::exit(0);
}

fn chip_info() {
    println!("{:x?}", aarch32_cpu::register::Midr::read());
    println!("{:x?}", aarch32_cpu::register::Cpsr::read());
    #[cfg(arm_architecture = "v8-r")]
    {
        println!("{:x?}", aarch32_cpu::register::ImpCbar::read());
        println!("{:x?}", aarch32_cpu::register::Vbar::read());
        // This only works in EL2 and start-up put us in EL1
        // println!("{:?}", aarch32_cpu::register::Hvbar::read());
    }
}

#[cfg(arm_architecture = "v7-r")]
fn mpu_pmsa_v7() {
    use aarch32_cpu::{
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
    use aarch32_cpu::{
        pmsav8::{
            Cacheable, El1AccessPerms, El1Config, El1Mpu, El1Region, El1Shareability, MemAttr,
            RwAllocPolicy,
        },
        register::{Mpuir, armv8r::*},
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
    println!("Region 0: {:08x?}", Prbar0::read());
    println!("Region 1: {:08x?}", Prbar1::read());
    println!("Region 2: {:08x?}", Prbar2::read());
    println!("Region 3: {:08x?}", Prbar3::read());
    println!("Region 4: {:08x?}", Prbar4::read());
    println!("Region 5: {:08x?}", Prbar5::read());
    println!("Region 6: {:08x?}", Prbar6::read());
    println!("Region 7: {:08x?}", Prbar7::read());
    println!("Region 8: {:08x?}", Prbar8::read());
    println!("Region 9: {:08x?}", Prbar9::read());
    println!("Region 10: {:08x?}", Prbar10::read());
    println!("Region 11: {:08x?}", Prbar11::read());
    println!("Region 12: {:08x?}", Prbar12::read());
    println!("Region 13: {:08x?}", Prbar13::read());
    println!("Region 14: {:08x?}", Prbar14::read());
    println!("Region 15: {:08x?}", Prbar15::read());

    // Load a config (but don't enable it)
    #[allow(clippy::zero_ptr)]
    mpu.configure(&El1Config {
        background_config: true,
        regions: &[
            El1Region {
                range: 0x0000_0000 as *mut u8..=0x0FFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0x1000_0000 as *mut u8..=0x1FFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0x2000_0000 as *mut u8..=0x2FFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0x3000_0000 as *mut u8..=0x3FFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0x4000_0000 as *mut u8..=0x4FFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0x5000_0000 as *mut u8..=0x5FFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0x6000_0000 as *mut u8..=0x6FFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0x7000_0000 as *mut u8..=0x7FFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0x8000_0000 as *mut u8..=0x8FFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0x9000_0000 as *mut u8..=0x9FFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0xA000_0000 as *mut u8..=0xAFFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0xB000_0000 as *mut u8..=0xBFFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0xC000_0000 as *mut u8..=0xCFFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0xD000_0000 as *mut u8..=0xDFFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0xE000_0000 as *mut u8..=0xEFFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
            El1Region {
                range: 0xF000_0000 as *mut u8..=0xFFFF_FFFF as *mut u8,
                shareability: El1Shareability::OuterShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: 0,
                enable: true,
            },
        ],
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
    println!("Region 0: {:08x?}", Prbar0::read());
    println!("Region 1: {:08x?}", Prbar1::read());
    println!("Region 2: {:08x?}", Prbar2::read());
    println!("Region 3: {:08x?}", Prbar3::read());
    println!("Region 4: {:08x?}", Prbar4::read());
    println!("Region 5: {:08x?}", Prbar5::read());
    println!("Region 6: {:08x?}", Prbar6::read());
    println!("Region 7: {:08x?}", Prbar7::read());
    println!("Region 8: {:08x?}", Prbar8::read());
    println!("Region 9: {:08x?}", Prbar9::read());
    println!("Region 10: {:08x?}", Prbar10::read());
    println!("Region 11: {:08x?}", Prbar11::read());
    println!("Region 12: {:08x?}", Prbar12::read());
    println!("Region 13: {:08x?}", Prbar13::read());
    println!("Region 14: {:08x?}", Prbar14::read());
    println!("Region 15: {:08x?}", Prbar15::read());
}

fn test_changing_sctlr() {
    println!(
        "{:?} before setting C, I and Z",
        aarch32_cpu::register::Sctlr::read()
    );
    aarch32_cpu::register::Sctlr::modify(|w| {
        w.set_c(true);
        w.set_i(true);
        w.set_z(true);
    });
    println!("{:?} after", aarch32_cpu::register::Sctlr::read());
}
