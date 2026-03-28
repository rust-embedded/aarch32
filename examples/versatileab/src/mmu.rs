//! MMU Initialisation Code for Versatile AB
//!
//! Required when running with a VMSA processor, such as the Cortex-A8
//!
//! Without MMU programming, all memory is strongly-ordered and that means
//! unaligned loads will cause data aborts.
//!
//! ## Memory Map
//!
//! | Content     | Range                         | Attributes                  |
//! |-------------|-------------------------------|-----------------------------|
//! | SDRAM       | `0x0000_0000 .. 0x07FF_FFFF`  | Normal write-back cacheable |
//! | Reserved    | `0x0800_0000 .. 0x0FFF_FFFF`  | Unassigned                  |
//! | Peripherals | `0x1000_0000 .. 0x101F_FFFF`  | Device Memory               |
//! | Reserved    | `0x1020_0000 .. 0xFFFF_FFFF`  | Unassigned                  |
//!
//! Note that this table is enough to run the examples, but it does not describe
//! all the hardware either on the real board or emulated by QEMU.

use aarch32_cpu::mmu::{
    AccessPermissions, CachePolicy, L1Section, L1Table, MemoryRegionAttributes,
    NUM_L1_PAGE_TABLE_ENTRIES, SectionAttributes,
};
use arbitrary_int::u4;

/// Our MMU page table
static MMU_L1_PAGE_TABLE: L1Table = make_mmu_table();

const SDRAM_ATTRS: SectionAttributes = SectionAttributes {
    non_global: false,
    p_bit: false,
    shareable: true,
    access: AccessPermissions::FullAccess,
    memory_attrs: MemoryRegionAttributes::CacheableMemory {
        inner: CachePolicy::WriteBackWriteAlloc,
        outer: CachePolicy::WriteBackWriteAlloc,
    }
    .as_raw(),
    domain: u4::new(0b0),
    execute_never: false,
};

const DEVICE_ATTRS: SectionAttributes = SectionAttributes {
    non_global: false,
    p_bit: false,
    shareable: false,
    access: AccessPermissions::FullAccess,
    memory_attrs: MemoryRegionAttributes::ShareableDevice.as_raw(),
    domain: u4::new(0b0),
    execute_never: false,
};

/// The number of bytes in 1 MiB
const ONE_MB: u32 = 1024 * 1024;

const fn make_mmu_table() -> L1Table {
    let mut temp: [L1Section; NUM_L1_PAGE_TABLE_ENTRIES] =
        [L1Section::ZERO; NUM_L1_PAGE_TABLE_ENTRIES];
    let mut page = 0;
    // Map 128 MiB of RAM @ 0x0000_0000
    while page < 128 {
        let section =
            L1Section::new_with_addr_and_attrs(0x0000_0000 + (page * ONE_MB), SDRAM_ATTRS);
        temp[0x000 + (page as usize)] = section;
        page += 1;
    }
    // Map 2 MiB of peripherals @ 0x1000_0000
    page = 0;
    while page < 2 {
        let section =
            L1Section::new_with_addr_and_attrs(0x1000_0000 + (page * ONE_MB), DEVICE_ATTRS);
        temp[0x100 + (page as usize)] = section;
        page += 1;
    }

    L1Table {
        entries: core::cell::UnsafeCell::new(temp),
    }
}

/// Set the MMU base register to `MMU_L1_PAGE_TABLE`
pub fn set_mmu() {
    let ttbr0 = aarch32_cpu::register::Ttbr0::new_with_raw_value(0)
        .with_address(core::ptr::addr_of!(MMU_L1_PAGE_TABLE) as usize)
        .with_irgn(false)
        .with_nos(false)
        .with_rgn(aarch32_cpu::register::ttbr0::Region::WriteBackWriteAllocCacheable)
        .with_s(true)
        .with_c(true);
    unsafe { aarch32_cpu::register::Ttbr0::write(ttbr0) }
}

/// Enable the MMU and the cache
pub fn enable_mmu_and_cache() {
    // Enable Manager access to Domain 0
    aarch32_cpu::register::Dacr::modify(|d| {
        d.set_d(0, aarch32_cpu::register::dacr::DomainAccess::Manager);
    });
    // This function contains the barrier we need to flush the pipeline
    aarch32_cpu::register::Sctlr::modify(|s| {
        // Enable Cache
        s.set_c(true);
        // Enable MMU
        s.set_m(true);
    });
}
