//! Support for the PMSAv8-32 EL1 MPU
//!
//! See Part C: Armv8-R Protected Memory System Architecture in [ARM
//! Architecture Reference Manual Supplement - ARMv8, for the ARMv8-R AArch32
//! architecture profile][armv8r]
//!
//! [armv8r]: https://developer.arm.com/documentation/ddi0568/latest/

use arbitrary_int::{u26, u3};

use crate::register;

#[doc(inline)]
pub use register::hprbar::{AccessPerms as El2AccessPerms, Shareability as El2Shareability};
#[doc(inline)]
pub use register::prbar::{AccessPerms as El1AccessPerms, Shareability as El1Shareability};

/// Ways this API can fail
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Found too many regions
    TooManyRegions,
    /// Found an invalid MAIR selector (only 0..=7 is valid)
    InvalidMair(u8),
    /// Found a region with invalid alignment
    UnalignedRegion(core::ops::RangeInclusive<*mut u8>),
}

/// Represents our PMSAv8-32 EL1 MPU
pub struct El1Mpu();

impl El1Mpu {
    /// Create an MPU handle
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El1Mpu {
        El1Mpu()
    }

    /// How many MPU regions are there?
    pub fn num_regions(&self) -> u8 {
        register::Mpuir::read().dregions()
    }

    /// Access the current state of a region
    pub fn get_region(&mut self, idx: u8) -> Option<El1Region> {
        if idx >= self.num_regions() {
            return None;
        }
        register::Prselr::write(register::Prselr(idx as u32));
        let prbar = register::Prbar::read();
        let prlar = register::Prlar::read();
        let start_addr = (prbar.base().value() << 6) as *mut u8;
        let end_addr = ((prlar.limit().value() << 6) | 0x3F) as *mut u8;
        Some(El1Region {
            range: start_addr..=end_addr,
            shareability: prbar.shareability(),
            access: prbar.access_perms(),
            no_exec: prbar.nx(),
            mair: prlar.mair().value(),
            enable: prlar.enabled(),
        })
    }

    /// Write a single region to the EL1 MPU
    ///
    /// ## Arguments
    ///
    /// - `region`: The [El1Region] object containing the configuration for the MPU region.
    /// - `idx`: The index of the region to be configured.
    ///
    /// ## Errors
    ///
    /// Returns:
    /// - [Error::UnalignedRegion] if the region's start address is not 64-byte aligned.
    /// - [Error::UnalignedRegion] if the region's end address is not 63-byte aligned.
    /// - [Error::InvalidMair] if the region's MAIR index is invalid (greater than 7).
    pub fn set_region(&mut self, idx: u8, region: &El1Region) -> Result<(), Error> {
        let start = *(region.range.start()) as usize as u32;
        // Check for 64-byte alignment (0x3F is six bits)
        if start & 0x3F != 0 {
            return Err(Error::UnalignedRegion(region.range.clone()));
        }
        let end = *(region.range.end()) as usize as u32;
        if end & 0x3F != 0x3F {
            return Err(Error::UnalignedRegion(region.range.clone()));
        }
        if region.mair > 7 {
            return Err(Error::InvalidMair(region.mair));
        }
        register::Prselr::write(register::Prselr(idx as u32));
        register::Prbar::write({
            let mut bar = register::Prbar::new_with_raw_value(0);
            bar.set_base(u26::from_u32(start >> 6));
            bar.set_access_perms(region.access);
            bar.set_nx(region.no_exec);
            bar.set_shareability(region.shareability);
            bar
        });
        register::Prlar::write({
            let mut lar = register::Prlar::new_with_raw_value(0);
            lar.set_limit(u26::from_u32(end >> 6));
            lar.set_enabled(region.enable);
            lar.set_mair(u3::from_u8(region.mair));
            lar
        });

        Ok(())
    }

    /// Writes a subset of EL1 MPU regions starting from a specified index.
    ///
    /// ## Arguments
    ///
    /// - `regions_starting_idx`: The starting index for the regions to be reconfigured.
    /// - `regions`: A slice of [El1Region] objects that will overwrite the previous regions starting from `regions_starting_idx`.
    pub fn set_regions(
        &mut self,
        regions_starting_idx: u8,
        regions: &[El1Region],
    ) -> Result<(), Error> {
        if regions.len().saturating_add(regions_starting_idx as usize) > self.num_regions() as usize
        {
            return Err(Error::TooManyRegions);
        }

        for (idx, region) in regions.iter().enumerate() {
            self.set_region(idx as u8 + regions_starting_idx, region)?;
        }

        Ok(())
    }

    /// Set the memory attributes to MAIR0 and MAIR1
    #[allow(clippy::get_first)]
    pub fn set_attributes(&mut self, memattrs: &[MemAttr]) {
        let mem_attr0 = memattrs.get(0).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr1 = memattrs.get(1).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr2 = memattrs.get(2).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr3 = memattrs.get(3).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mair0 = mem_attr3 << 24 | mem_attr2 << 16 | mem_attr1 << 8 | mem_attr0;
        unsafe {
            register::Mair0::write(register::Mair0(mair0));
        }
        let mem_attr0 = memattrs.get(4).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr1 = memattrs.get(5).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr2 = memattrs.get(6).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr3 = memattrs.get(7).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mair1 = mem_attr3 << 24 | mem_attr2 << 16 | mem_attr1 << 8 | mem_attr0;
        unsafe {
            register::Mair1::write(register::Mair1(mair1));
        }
    }

    /// Enable or disable the background region
    pub fn background_region_enable(&mut self, enable: bool) {
        register::Sctlr::modify(|r| {
            r.set_br(enable);
        });
    }

    /// Configure the EL1 MPU
    ///
    /// Write regions, attributes and enable/disable the background region
    /// with a single [El1Config] struct.
    pub fn configure(&mut self, config: &El1Config) -> Result<(), Error> {
        self.set_regions(0, config.regions)?;

        self.set_attributes(config.memory_attributes);

        self.background_region_enable(config.background_config);

        Ok(())
    }

    /// Enable the EL1 MPU
    pub fn enable(&mut self) {
        register::Sctlr::modify(|r| {
            r.set_m(true);
        });
    }

    /// Disable the EL1 MPU
    pub fn disable(&mut self) {
        register::Sctlr::modify(|r| {
            r.set_m(false);
        });
    }
}

/// Represents our PMSAv8-32 EL2 MPU
pub struct El2Mpu();

impl El2Mpu {
    /// Create an EL2 MPU handle
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El2Mpu {
        El2Mpu()
    }

    /// How many EL2 MPU regions are there?
    pub fn num_regions(&self) -> u8 {
        register::Hmpuir::read().region()
    }

    /// Access the current state of a region
    pub fn get_region(&mut self, idx: u8) -> Option<El2Region> {
        if idx >= self.num_regions() {
            return None;
        }
        register::Hprselr::write(register::Hprselr(idx as u32));
        let hprbar = register::Hprbar::read();
        let hprlar = register::Hprlar::read();
        let start_addr = (hprbar.base().value() << 6) as *mut u8;
        let end_addr = ((hprlar.limit().value() << 6) | 0x3F) as *mut u8;
        Some(El2Region {
            range: start_addr..=end_addr,
            shareability: hprbar.shareability(),
            access: hprbar.access_perms(),
            no_exec: hprbar.nx(),
            mair: hprlar.mair().value(),
            enable: hprlar.enabled(),
        })
    }

    /// Write a single region to the EL2 MPU
    ///
    /// ## Arguments
    ///
    /// - `region`: The [El2Region] object containing the configuration for the MPU region.
    /// - `idx`: The index of the region to be configured.
    ///
    /// ## Errors
    ///
    /// Returns:
    /// - [Error::UnalignedRegion] if the region's start address is not 64-byte aligned.
    /// - [Error::UnalignedRegion] if the region's end address is not 63-byte aligned.
    /// - [Error::InvalidMair] if the region's MAIR index is invalid (greater than 7).
    pub fn set_region(&mut self, idx: u8, region: &El2Region) -> Result<(), Error> {
        let start = *(region.range.start()) as usize as u32;
        // Check for 64-byte alignment (0x3F is six bits)
        if start & 0x3F != 0 {
            return Err(Error::UnalignedRegion(region.range.clone()));
        }
        let end = *(region.range.end()) as usize as u32;
        if end & 0x3F != 0x3F {
            return Err(Error::UnalignedRegion(region.range.clone()));
        }
        if region.mair > 7 {
            return Err(Error::InvalidMair(region.mair));
        }
        register::Hprselr::write(register::Hprselr(idx as u32));
        register::Hprbar::write({
            let mut bar = register::Hprbar::new_with_raw_value(0);
            bar.set_base(u26::from_u32(start >> 6));
            bar.set_access_perms(region.access);
            bar.set_nx(region.no_exec);
            bar.set_shareability(region.shareability);
            bar
        });
        register::Hprlar::write({
            let mut lar = register::Hprlar::new_with_raw_value(0);
            lar.set_limit(u26::from_u32(end >> 6));
            lar.set_enabled(region.enable);
            lar.set_mair(u3::from_u8(region.mair));
            lar
        });

        Ok(())
    }

    /// Writes a subset of EL2 MPU regions starting from a specified index.
    ///
    /// ## Arguments
    ///
    /// - `regions_starting_idx`: The starting index for the regions to be reconfigured.
    /// - `regions`: A slice of [El2Region] objects that will overwrite the previous regions starting from `regions_starting_idx`.
    pub fn set_regions(
        &mut self,
        regions_starting_idx: u8,
        regions: &[El2Region],
    ) -> Result<(), Error> {
        if regions.len().saturating_add(regions_starting_idx as usize) > self.num_regions() as usize
        {
            return Err(Error::TooManyRegions);
        }

        for (idx, region) in regions.iter().enumerate() {
            self.set_region(idx as u8 + regions_starting_idx, region)?;
        }

        Ok(())
    }

    /// Set the memory attributes to HMAIR0 and HMAIR1
    #[allow(clippy::get_first)]
    pub fn set_attributes(&mut self, memattrs: &[MemAttr]) {
        let mem_attr0 = memattrs.get(0).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr1 = memattrs.get(1).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr2 = memattrs.get(2).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr3 = memattrs.get(3).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let hmair0 = mem_attr3 << 24 | mem_attr2 << 16 | mem_attr1 << 8 | mem_attr0;
        unsafe {
            register::Hmair0::write(register::Hmair0(hmair0));
        }
        let mem_attr0 = memattrs.get(4).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr1 = memattrs.get(5).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr2 = memattrs.get(6).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let mem_attr3 = memattrs.get(7).map(|m| m.to_bits()).unwrap_or(0) as u32;
        let hmair1 = mem_attr3 << 24 | mem_attr2 << 16 | mem_attr1 << 8 | mem_attr0;
        unsafe {
            register::Hmair1::write(register::Hmair1(hmair1));
        }
    }

    /// Enable or disable the background region
    pub fn background_region_enable(&mut self, enable: bool) {
        register::Hsctlr::modify(|r| {
            r.set_br(enable);
        });
    }

    /// Configure the EL2 MPU
    ///
    /// Write regions, attributes and enable/disable the background region with a single [El2Config] struct.
    pub fn configure(&mut self, config: &El2Config) -> Result<(), Error> {
        self.set_regions(0, config.regions)?;

        self.set_attributes(config.memory_attributes);

        self.background_region_enable(config.background_config);

        Ok(())
    }

    /// Enable the EL2 MPU
    pub fn enable(&mut self) {
        register::Hsctlr::modify(|r| {
            r.set_m(true);
        });
    }

    /// Disable the EL2 MPU
    pub fn disable(&mut self) {
        register::Hsctlr::modify(|r| {
            r.set_m(false);
        });
    }
}

/// Configuration for the PMSAv8-32 EL1 MPU
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct El1Config<'a> {
    /// Background Config Enable
    ///
    /// If true, use the default MMU config if no other region matches an address
    pub background_config: bool,
    /// Information about each Region.
    ///
    /// If you pass more regions than the MPU supports, you get an error.
    pub regions: &'a [El1Region],
    /// Information about each Memory Attribute
    ///
    /// If you pass more MemAttrs than the MPU supports (8), you get an error.
    pub memory_attributes: &'a [MemAttr],
}

/// Configuration for the PMSAv8-32 MPU
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct El1Region {
    /// The range of the region
    ///
    /// * The first address must be a multiple of 32.
    /// * The length must be a multiple of 32.
    pub range: core::ops::RangeInclusive<*mut u8>,
    /// Shareability of the region
    pub shareability: El1Shareability,
    /// Access for the region
    pub access: El1AccessPerms,
    /// Is region no-exec?
    pub no_exec: bool,
    /// Which Memory Attribute applies here?
    ///
    /// Selects from the eight attributes in {MAIR0, MAIR1}.
    ///
    /// Only values 0..=7 are valid here.
    pub mair: u8,
    /// Is this region enabled?
    pub enable: bool,
}

// Creating a static Region is fine - the pointers within it
// only go to the MPU and aren't accessed via Rust code
unsafe impl Sync for El1Region {}

/// Configuration for the PMSAv8-32 EL2 MPU
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct El2Config<'a> {
    /// Background Config Enable
    ///
    /// If true, use the default MMU config if no other region matches an address
    pub background_config: bool,
    /// Information about each Region.
    ///
    /// If you pass more regions than the MPU supports, you get an error.
    pub regions: &'a [El2Region],
    /// Information about each Memory Attribute
    ///
    /// If you pass more MemAttrs than the MPU supports (8), you get an error.
    pub memory_attributes: &'a [MemAttr],
}

/// Configuration for the PMSAv8-32 EL2 MPU
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct El2Region {
    /// The range of the region
    ///
    /// * The first address must be a multiple of 32.
    /// * The length must be a multiple of 32.
    pub range: core::ops::RangeInclusive<*mut u8>,
    /// Shareability of the region
    pub shareability: El2Shareability,
    /// Access for the region
    pub access: El2AccessPerms,
    /// Is region no-exec?
    pub no_exec: bool,
    /// Which Memory Attribute applies here?
    ///
    /// Selects from the eight attributes in {HMAIR0, HMAIR1}.
    ///
    /// Only values 0..=7 are valid here.
    pub mair: u8,
    /// Is this region enabled?
    pub enable: bool,
}

// Creating a static El2Region is fine - the pointers within it
// only go to the MPU and aren't accessed via Rust code
unsafe impl Sync for El2Region {}

/// Describes the memory ordering and cacheability of a region
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemAttr {
    /// Strongly-ordered memory
    StronglyOrdered,
    /// Device memory
    DeviceMemory,
    /// Normal memory
    NormalMemory {
        /// Controls outer access
        outer: Cacheable,
        /// Controls inner access
        inner: Cacheable,
    },
}

impl MemAttr {
    /// Convert this memory attribute to an 8-bit value we can write to MAIRx
    const fn to_bits(&self) -> u8 {
        match self {
            MemAttr::StronglyOrdered => 0b0000_0000,
            MemAttr::DeviceMemory => 0b0000_0100,
            MemAttr::NormalMemory { outer, inner } => {
                let outer_bits = outer.to_bits();
                let inner_bits = inner.to_bits();
                outer_bits << 4 | inner_bits
            }
        }
    }
}

/// Cacheability of a region
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cacheable {
    WriteThroughTransient(RwAllocPolicy),
    WriteBackTransient(RwAllocPolicy),
    WriteThroughNonTransient(RwAllocPolicy),
    WriteBackNonTransient(RwAllocPolicy),
    NonCacheable,
}

impl Cacheable {
    const fn to_bits(&self) -> u8 {
        #[allow(clippy::identity_op)]
        match self {
            Cacheable::WriteThroughTransient(rw_alloc) => 0b0000 | (*rw_alloc as u8),
            Cacheable::WriteBackTransient(rw_alloc) => 0b0100 | (*rw_alloc as u8),
            Cacheable::WriteThroughNonTransient(rw_alloc) => 0b1000 | (*rw_alloc as u8),
            Cacheable::WriteBackNonTransient(rw_alloc) => 0b1100 | (*rw_alloc as u8),
            Cacheable::NonCacheable => 0b0100,
        }
    }
}

/// Cache allocation policy
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum RwAllocPolicy {
    /// Write-allocate
    W = 0b01,
    /// Read-allocate
    R = 0b10,
    /// Read-allocate and Write-allocate
    RW = 0b11,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mem_attr_strong() {
        let mem_attr = MemAttr::StronglyOrdered;
        assert_eq!(mem_attr.to_bits(), 0b0000_0000);
    }

    #[test]
    fn mem_attr_device() {
        let mem_attr = MemAttr::DeviceMemory;
        assert_eq!(mem_attr.to_bits(), 0b0000_0100);
    }

    #[test]
    fn mem_attr_normal() {
        let mem_attr = MemAttr::NormalMemory {
            outer: Cacheable::NonCacheable,
            inner: Cacheable::WriteBackNonTransient(RwAllocPolicy::W),
        };
        assert_eq!(
            mem_attr.to_bits(),
            0b0100_1101,
            "0b{:08b}",
            mem_attr.to_bits()
        );
    }
}
