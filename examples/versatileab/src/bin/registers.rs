//! Registers example

#![no_std]
#![no_main]

// pull in our start-up code
use versatileab as _;

use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in the runtime crate.
#[no_mangle]
pub extern "C" fn boot_core(cpu_id: u32) {
    match cpu_id {
        0 => {
            main();
        }
        _ => panic!("unexpected CPU ID {}", cpu_id),
    }
}

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-m-rt`.
#[export_name = "main"]
fn main() -> ! {
    chip_info();
    test_changing_sctlr();
    #[cfg(arm_architecture = "v7-r")]
    mpu_pmsa_v7();
    semihosting::process::exit(0);
}

fn chip_info() {
    println!("{:?}", cortex_ar::register::Midr::read());
    println!("{:?}", cortex_ar::register::Cpsr::read());
    println!("{:?}", cortex_ar::register::Mpidr::read());
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
