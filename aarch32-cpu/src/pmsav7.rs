//! Support for the PMSAv7 MPU
//!
//! See Chapter 14: Protected Memory System Architecture in [Arm
//! Architecture Reference Manual ARMv7-A and ARMv7-R edition][armv7]
//!
//! [armv7]: https://developer.arm.com/documentation/ddi0406/latest

use crate::register;

use arbitrary_int::{u2, u3};
#[doc(inline)]
pub use register::drsr::RegionSize;

/// Ways this API can fail
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Found too many regions
    TooManyRegions,
    /// Found a region with invalid alignment
    UnalignedRegion(*mut u8),
}

/// Represents our PMSAv7 MPU
pub struct Mpu();

impl Mpu {
    /// Create an MPU handle
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> Mpu {
        Mpu()
    }

    /// How many MPU instruction regions are there?
    pub fn num_iregions(&self) -> u8 {
        register::Mpuir::read().iregions()
    }

    /// How many MPU data/unified regions are there?
    pub fn num_dregions(&self) -> u8 {
        register::Mpuir::read().dregions()
    }

    /// Do we have a unified MPU?
    pub fn is_unified(&self) -> bool {
        !register::Mpuir::read().non_unified()
    }

    /// Get an instruction region
    pub fn get_iregion(&mut self, idx: u8) -> Option<Region> {
        if idx >= self.num_iregions() {
            return None;
        }
        register::Rgnr::write(register::Rgnr(idx as u32));
        let base = register::Irbar::read().0 as *mut u8;
        let rsr = register::Irsr::read();
        let racr = register::Iracr::read();

        let mem_attr_bits = MemAttrBits {
            tex: racr.tex(),
            c: racr.c(),
            b: racr.b(),
            s: racr.s(),
        };

        let mem_attr = mem_attr_bits.decode()?;

        Some(Region {
            base,
            size: rsr.region_size(),
            subregion_mask: rsr.subregion_mask(),
            enabled: rsr.enabled(),
            no_exec: racr.nx(),
            mem_attr,
        })
    }

    /// Get a data/unified region
    pub fn get_dregion(&mut self, idx: u8) -> Option<Region> {
        if idx >= self.num_dregions() {
            return None;
        }
        register::Rgnr::write(register::Rgnr(idx as u32));
        let base = register::Drbar::read().0 as *mut u8;
        let rsr = register::Drsr::read();
        let racr = register::Dracr::read();

        let mem_attr_bits = MemAttrBits {
            tex: racr.tex(),
            c: racr.c(),
            b: racr.b(),
            s: racr.s(),
        };
        let mem_attr = mem_attr_bits.decode()?;

        Some(Region {
            base,
            size: rsr.region_size(),
            subregion_mask: rsr.subregion_mask(),
            enabled: rsr.enabled(),
            no_exec: racr.nx(),
            mem_attr,
        })
    }

    /// Configure the EL1 MPU
    pub fn configure(&mut self, config: &Config) -> Result<(), Error> {
        if config.iregions.len() > self.num_iregions() as usize {
            return Err(Error::TooManyRegions);
        }
        if config.dregions.len() > self.num_dregions() as usize {
            return Err(Error::TooManyRegions);
        }
        for (idx, region) in config.iregions.iter().enumerate() {
            register::Rgnr::write(register::Rgnr(idx as u32));
            if !region.size.is_aligned(region.base) {
                return Err(Error::UnalignedRegion(region.base));
            }
            register::Irbar::write(register::Irbar(region.base as u32));
            register::Irsr::write({
                let mut out = register::Irsr::new_with_raw_value(0);
                out.set_enabled(region.enabled);
                out.set_region_size(region.size);
                out.set_subregion_mask(region.subregion_mask);
                out
            });
            register::Iracr::write({
                let mut out = register::Iracr::new_with_raw_value(0);
                let mem_attr_bits = region.mem_attr.to_bits();
                out.set_tex(mem_attr_bits.tex);
                out.set_c(mem_attr_bits.c);
                out.set_b(mem_attr_bits.b);
                out.set_s(mem_attr_bits.s);
                out.set_nx(region.no_exec);
                // out.with_ap(region.access_perms);
                out
            });
        }
        for (idx, region) in config.dregions.iter().enumerate() {
            if !region.size.is_aligned(region.base) {
                return Err(Error::UnalignedRegion(region.base));
            }
            register::Rgnr::write(register::Rgnr(idx as u32));
            register::Drbar::write(register::Drbar(region.base as u32));
            register::Drsr::write({
                let mut out = register::Drsr::new_with_raw_value(0);
                out.set_enabled(region.enabled);
                out.set_region_size(region.size);
                out.set_subregion_mask(region.subregion_mask);
                out
            });
            register::Dracr::write({
                let mut out = register::Dracr::new_with_raw_value(0);
                let mem_attr_bits = region.mem_attr.to_bits();
                out.set_tex(mem_attr_bits.tex);
                out.set_c(mem_attr_bits.c);
                out.set_b(mem_attr_bits.b);
                out.set_s(mem_attr_bits.s);
                out.set_nx(region.no_exec);
                // out.with_ap(region.access_perms);
                out
            });
        }
        register::Sctlr::modify(|r| {
            r.set_br(config.background_config);
        });
        Ok(())
    }

    /// Enable the MPU
    pub fn enable(&mut self) {
        register::Sctlr::modify(|r| {
            r.set_m(true);
        });
    }

    /// Disable the MPU
    pub fn disable(&mut self) {
        register::Sctlr::modify(|r| {
            r.set_m(false);
        });
    }
}

/// Configuration for the PMSAv7 MPU
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config<'a> {
    /// Background Config Enable
    ///
    /// If true, use the default MMU config if no other region matches an address
    pub background_config: bool,
    /// List of instruction regions
    pub iregions: &'a [Region],
    /// List of data/unified regions
    pub dregions: &'a [Region],
}

/// Configuration for a region in the PMSAv7 MPU
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Region {
    /// The base address of this region.
    ///
    /// Must be aligned to the size of the region.
    pub base: *mut u8,
    /// The size of this region
    pub size: RegionSize,
    /// Sub-region bitmask
    ///
    /// The region is divided into exactly eight equal sized subregions.
    /// Subregion 0 is the subregion at the least significant address.
    ///
    /// A 1 bit means that sub-region is disabled.
    ///
    /// Only applies to regions sized 256 bytes or larger.
    pub subregion_mask: u8,
    /// Is this region enabled?
    pub enabled: bool,
    /// No-Execute in this region
    pub no_exec: bool,
    /// Attributes for this region
    pub mem_attr: MemAttr,
}

// Creating a static Region is fine - the pointers within it
// only go to the MPU and aren't accessed via Rust code
unsafe impl Sync for Region {}

/// Describes the memory ordering and cacheability of a region
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemAttr {
    /// Strongly-ordered memory
    StronglyOrdered,
    /// Device (shareable or non-shareable)
    Device { shareable: bool },
    /// Outer and Inner Write-Through, no Write-Allocate
    WriteThroughNoWriteAllocate { shareable: bool },
    /// Outer and Inner Write-Back, no Write-Allocate
    WriteBackNoWriteAllocate { shareable: bool },
    /// Outer and Inner Non-cacheable
    NonCacheable { shareable: bool },
    /// Implementation Defined
    ImplementationDefined { shareable: bool },
    /// Outer and Inner Write-Back, Write-Allocate
    WriteBackWriteAllocate { shareable: bool },
    /// Cacheable memory
    Cacheable {
        outer: CacheablePolicy,
        inner: CacheablePolicy,
        shareable: bool,
    },
}

impl MemAttr {
    /// Convert this memory attribute to an 8-bit value we can write to MAIRx
    const fn to_bits(&self) -> MemAttrBits {
        match self {
            MemAttr::StronglyOrdered => MemAttrBits {
                tex: u3::from_u8(0b000),
                c: false,
                b: false,
                s: true,
            },
            MemAttr::Device { shareable: true } => MemAttrBits {
                tex: u3::from_u8(0b000),
                c: false,
                b: true,
                s: true,
            },
            MemAttr::Device { shareable: false } => MemAttrBits {
                tex: u3::from_u8(0b010),
                c: false,
                b: false,
                s: false,
            },
            MemAttr::WriteThroughNoWriteAllocate { shareable } => MemAttrBits {
                tex: u3::from_u8(0b000),
                c: true,
                b: false,
                s: *shareable,
            },
            MemAttr::WriteBackNoWriteAllocate { shareable } => MemAttrBits {
                tex: u3::from_u8(0b000),
                c: true,
                b: true,
                s: *shareable,
            },
            MemAttr::NonCacheable { shareable } => MemAttrBits {
                tex: u3::from_u8(0b001),
                c: false,
                b: false,
                s: *shareable,
            },
            MemAttr::ImplementationDefined { shareable } => MemAttrBits {
                tex: u3::from_u8(0b001),
                c: true,
                b: false,
                s: *shareable,
            },
            MemAttr::WriteBackWriteAllocate { shareable } => MemAttrBits {
                tex: u3::from_u8(0b000),
                c: true,
                b: true,
                s: *shareable,
            },
            MemAttr::Cacheable {
                outer,
                inner,
                shareable,
            } => {
                let outer = *outer as u8;
                let inner = *inner as u8;
                MemAttrBits {
                    tex: u3::from_u8(0b100 | outer),
                    c: (inner & 0b10) != 0,
                    b: (inner & 0b01) != 0,
                    s: *shareable,
                }
            }
        }
    }
}

/// A representation of Memory Attributes suitable for sticking into the RACR register
#[derive(Debug, Clone, PartialEq, Eq)]
struct MemAttrBits {
    tex: u3,
    c: bool,
    b: bool,
    s: bool,
}

impl MemAttrBits {
    const fn decode(&self) -> Option<MemAttr> {
        match (self.tex.value(), self.c, self.b) {
            (0b000, false, false) => Some(MemAttr::StronglyOrdered),
            (0b000, false, true) => Some(MemAttr::Device { shareable: true }),
            (0b000, true, false) => {
                Some(MemAttr::WriteThroughNoWriteAllocate { shareable: self.s })
            }
            (0b000, true, true) => Some(MemAttr::WriteBackNoWriteAllocate { shareable: self.s }),
            (0b001, false, false) => Some(MemAttr::NonCacheable { shareable: self.s }),
            (0b001, true, false) => Some(MemAttr::ImplementationDefined { shareable: self.s }),
            (0b001, true, true) => Some(MemAttr::WriteBackWriteAllocate { shareable: self.s }),
            (0b010, false, false) => Some(MemAttr::Device { shareable: false }),
            (tex, c, b) if tex >= 0b100 => {
                let outer = tex & 0b11;
                let inner = ((c as u8) << 1) | (b as u8);
                Some(MemAttr::Cacheable {
                    outer: CacheablePolicy::new_with_raw_value(u2::from_u8(outer)),
                    inner: CacheablePolicy::new_with_raw_value(u2::from_u8(inner)),
                    shareable: self.s,
                })
            }
            _ => {
                // failed to decode
                None
            }
        }
    }
}

/// Describes the cache policy of a region
#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u2, exhaustive = true)]
pub enum CacheablePolicy {
    NonCacheable = 0b00,
    WriteBackWriteAllocate = 0b01,
    WriteThroughNoWriteAllocate = 0b10,
    WriteBackNoWriteAllocate = 0b11,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mem_attr_strong() {
        let mem_attr = MemAttr::StronglyOrdered;
        let mem_attr_bits = mem_attr.to_bits();
        assert_eq!(
            mem_attr_bits,
            MemAttrBits {
                tex: u3::from_u8(0),
                c: false,
                b: false,
                s: true
            }
        );
        let mem_attr2 = mem_attr_bits.decode();
        assert_eq!(Some(mem_attr), mem_attr2);
    }

    #[test]
    fn mem_attr_complex() {
        let mem_attr = MemAttr::Cacheable {
            // 0b01
            outer: CacheablePolicy::WriteBackWriteAllocate,
            // 0b10
            inner: CacheablePolicy::WriteThroughNoWriteAllocate,
            shareable: true,
        };
        let mem_attr_bits = mem_attr.to_bits();
        assert_eq!(
            mem_attr_bits,
            MemAttrBits {
                tex: u3::from_u8(0b101),
                c: true,
                b: false,
                s: true
            }
        );
        let mem_attr2 = mem_attr_bits.decode();
        assert_eq!(Some(mem_attr), mem_attr2);
    }
}
