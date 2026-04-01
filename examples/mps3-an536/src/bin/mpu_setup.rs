//! Stack-overflow check for Arm Cortex-R
//!
//! Uses the MPU to protect each stack

#![no_std]
#![no_main]

use aarch32_cpu::pmsav8::{
    Cacheable, El1AccessPerms, El1Mpu, El1Region, El1Shareability, MemAttr, RwAllocPolicy,
};
use aarch32_rt::{entry, sections::Section, stacks::Stack};
use semihosting::println;

use mps3_an536 as _;

unsafe extern "C" {
    static __stext: u8;
    static __etext: u8;
    static __srodata: u8;
    static __erodata: u8;
    static __sdata: u8;
    static __edata: u8;
    static __sbss: u8;
    static __ebss: u8;
}

const MAIR_READ_ONLY: u8 = 0;
const MAIR_READ_WRITE: u8 = 1;
const MAIR_DEVICE: u8 = 2;

static MEM_ATTRS: [MemAttr; 8] = [
    // Read-only Code RAM
    MemAttr::NormalMemory {
        outer: Cacheable::WriteThroughNonTransient(RwAllocPolicy::R),
        inner: Cacheable::WriteThroughNonTransient(RwAllocPolicy::R),
    },
    // Read-write RAM
    MemAttr::NormalMemory {
        outer: Cacheable::WriteBackNonTransient(RwAllocPolicy::W),
        inner: Cacheable::WriteBackNonTransient(RwAllocPolicy::W),
    },
    // Device Memory
    MemAttr::DeviceMemory,
    // Spare entries
    MemAttr::DeviceMemory,
    MemAttr::DeviceMemory,
    MemAttr::DeviceMemory,
    MemAttr::DeviceMemory,
    MemAttr::DeviceMemory,
];

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch32-rt`.
#[entry]
fn main() -> ! {
    let mut mpu = unsafe { El1Mpu::new() };

    mpu.set_attributes(&MEM_ATTRS);

    let mut last = 0;
    for (idx, stack) in Stack::iter().enumerate() {
        let range = stack.mpu_range(0).unwrap();
        println!("{:>13} @ {:010x?}", stack, range);

        let region = El1Region {
            range,
            shareability: El1Shareability::NonShareable,
            access: El1AccessPerms::ReadWrite,
            no_exec: true,
            mair: MAIR_READ_WRITE,
            enable: true,
        };
        mpu.set_region(idx as u8, &region).unwrap();
        last = idx as u8;
    }

    let mut next = last + 1;

    if let Some(range) = Section::VectorTable.mpu_range() {
        println!("{:>13} @ {:010x?}", Section::VectorTable, range);
        mpu.set_region(
            next,
            &El1Region {
                range,
                shareability: El1Shareability::NonShareable,
                access: El1AccessPerms::ReadOnly,
                no_exec: false,
                mair: MAIR_READ_ONLY,
                enable: true,
            },
        )
        .unwrap();
        next += 1;
    }

    if let Some(range) = Section::Text.mpu_range() {
        println!("{:>13} @ {:010x?}", Section::Text, range);
        mpu.set_region(
            next,
            &El1Region {
                range,
                shareability: El1Shareability::NonShareable,
                access: El1AccessPerms::ReadOnly,
                no_exec: false,
                mair: MAIR_READ_ONLY,
                enable: true,
            },
        )
        .unwrap();
        next += 1;
    }

    if let Some(range) = Section::Rodata.mpu_range() {
        println!("{:>13} @ {:010x?}", Section::Rodata, range);
        mpu.set_region(
            next,
            &El1Region {
                range,
                shareability: El1Shareability::NonShareable,
                access: El1AccessPerms::ReadOnly,
                no_exec: true,
                mair: MAIR_READ_ONLY,
                enable: true,
            },
        )
        .unwrap();
        next += 1;
    }

    if let Some(range) = Section::Data.mpu_range() {
        println!("{:>13} @ {:010x?}", Section::Data, range);
        mpu.set_region(
            next,
            &El1Region {
                range,
                shareability: El1Shareability::NonShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: MAIR_READ_WRITE,
                enable: true,
            },
        )
        .unwrap();
        next += 1;
    }

    if let Some(range) = Section::Bss.mpu_range() {
        println!("{:>13} @ {:010x?}", Section::Bss, range);
        mpu.set_region(
            next,
            &El1Region {
                range,
                shareability: El1Shareability::NonShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: MAIR_READ_WRITE,
                enable: true,
            },
        )
        .unwrap();
        next += 1;
    }

    if let Some(range) = Section::Uninit.mpu_range() {
        println!("{:>13} @ {:010x?}", Section::Uninit, range);
        mpu.set_region(
            next,
            &El1Region {
                range,
                shareability: El1Shareability::NonShareable,
                access: El1AccessPerms::ReadWrite,
                no_exec: true,
                mair: MAIR_READ_WRITE,
                enable: true,
            },
        )
        .unwrap();
        next += 1;
    }

    mpu.set_region(
        next,
        &El1Region {
            range: 0xE000_0000 as *const u8..=0xFFFF_FFFF as *const u8,
            shareability: El1Shareability::NonShareable,
            access: El1AccessPerms::ReadWrite,
            no_exec: true,
            mair: MAIR_DEVICE,
            enable: true,
        },
    )
    .unwrap();

    mpu.enable();

    for idx in 0..mpu.num_regions() {
        if let Some(region) = mpu.get_region(idx)
            && region.enable
        {
            println!("Region {:02}: {:010x?}", idx, region);
        }
    }

    mps3_an536::exit(0);
}
