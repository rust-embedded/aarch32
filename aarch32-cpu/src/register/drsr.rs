//! Code for managing DRSR (*Data Region Size and Enable Register*)

use arbitrary_int::prelude::*;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// The size of a region
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[bitbybit::bitenum(u5, exhaustive = true)]
pub enum RegionSize {
    /// A value of 0 is not permitted, this value is reserved and unpredictable.
    Invalid = 0,
    /// N = 1 => 4 bytes
    _4B = 1,
    /// N = 2 => 8 bytes
    _8B = 2,
    /// N = 3 => 16 bytes
    _16B = 3,
    /// N = 4 => 32 BYTES
    _32B = 4,
    /// N = 5 => 64 bytes
    _64B = 5,
    /// N = 6 => 128 bytes
    _128B = 6,
    /// N = 7 => 256 bytes
    _256B = 7,
    /// N = 8 => 512 bytes
    _512b = 8,
    /// N = 9 => 1 KiB
    _1K = 9,
    /// N = 10 => 2 KiB
    _2K = 10,
    /// N = 11 => 4 KiB
    _4K = 11,
    /// N = 12 => 8 KiB
    _8K = 12,
    /// N = 13 => 16 KiB
    _16K = 13,
    /// N = 14 => 32 KiB
    _32K = 14,
    /// N = 15 => 64 KiB
    _64K = 15,
    /// N = 16 => 128 KiB
    _128K = 16,
    /// N = 17 => 256 KiB
    _256K = 17,
    /// N = 18 => 512 KiB
    _512K = 18,
    /// N = 19 => 1 MiB
    _1M = 19,
    /// N = 20 => 2 MiB
    _2M = 20,
    /// N = 21 => 4 MiB
    _4M = 21,
    /// N = 22 => 8 MiB
    _8M = 22,
    /// N = 23 => 16 MiB
    _16M = 23,
    /// N = 24 => 32 MiB
    _32M = 24,
    /// N = 25 => 64 MiB
    _64M = 25,
    /// N = 26 => 128 MiB
    _128M = 26,
    /// N = 27 => 256 MiB
    _256M = 27,
    /// N = 28 => 512 MiB
    _512M = 28,
    /// N = 29 => 1 GiB
    _1G = 29,
    /// N = 30 => 2 GiB
    _2G = 30,
    /// N = 31 => 4 GiB
    _4G = 31,
}

impl RegionSize {
    pub fn is_aligned(&self, addr: *const u8) -> bool {
        let addr = addr as usize;
        if *self == RegionSize::_4G {
            // only one address allowed for 4GB region size
            addr == 0
        } else {
            let n = self.raw_value().as_u8();
            let mask = (1 << (n + 1)) - 1;
            (addr & mask) == 0
        }
    }
}

/// DRSR (*Data Region Size and Enable Register*)
#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
pub struct Drsr {
    /// Sub-region bitmask
    ///
    /// The region is divided into exactly eight equal sized subregions.
    /// Subregion 0 is the subregion at the least significant address.
    ///
    /// A 1 bit means that sub-region is disabled.
    ///
    /// Only applies to regions sized 256 bytes or larger.
    #[bits(8..=15, rw)]
    subregion_mask: u8,
    /// Region Size
    #[bits(1..=5, rw)]
    region_size: RegionSize,
    /// Is region enabled?
    #[bits(0..=0, rw)]
    enabled: bool,
}

impl SysReg for Drsr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for Drsr {}
impl Drsr {
    #[inline]
    /// Reads DRSR (*Data Region Size and Enable Register*)
    ///
    /// Set RGNR to control which region this reads.
    pub fn read() -> Drsr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Drsr {}
impl Drsr {
    #[inline]
    /// Writes DRSR (*Data Region Size and Enable Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Drsr) {
        unsafe { <Self as SysRegWrite>::write_raw(value.raw_value()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn aligned_4g() {
        let addr: *const u8 = core::ptr::null();
        assert!(RegionSize::_4G.is_aligned(addr));
    }

    #[test]
    fn aligned_1g() {
        let addr = 0x4000_0000 as *const u8;
        assert!(RegionSize::_1G.is_aligned(addr));
        let addr = 0x4000_0001 as *const u8;
        assert!(!RegionSize::_1G.is_aligned(addr));
    }

    #[test]
    fn aligned_256b() {
        let addr = 0x100 as *const u8;
        assert!(RegionSize::_256B.is_aligned(addr));
        let addr = 0x80 as *const u8;
        assert!(!RegionSize::_256B.is_aligned(addr));
    }
}
