//! Code for managing TTBR0 (*Translation Table Base Register 0*)

use arbitrary_int::u25;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// TTBR0 (*Translation Table Base Register 0*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ttbr0 {
    /// Translation table base 0 address
    ///
    /// The actual width can vary from 7..=31 to 14..=31, depending on the value
    /// of TTBCR.N.
    #[bits(7..=31, rw)]
    addr: u25,

    /// Inner Region
    ///
    /// See docs for the [`Ttbr0::c`] bit.
    #[bit(6, rw)]
    irgn: bool,

    /// Not Outer Shareable
    ///
    /// Ignored when [`Ttbr0::s`] is `false`.
    #[bit(5, rw)]
    nos: bool,

    /// Region bits
    ///
    /// Indicates the Outer cacheability attributes for the memory associated
    /// with the translation table walks.
    #[bits(3..=4, rw)]
    rgn: Region,

    /// Shareable
    #[bit(1, rw)]
    s: bool,

    /// Cacheable
    ///
    /// * c = false => Inner Non-cacheable
    /// * c = true => Inner Cacheable
    ///
    /// On a multi-processor system, set this in conjunction with `irgn` as follows:
    ///
    /// * c = false, irgn = false => Normal memory, Inner Non-cacheable
    /// * c = true, irgn = false => Normal memory, Inner Write-Through Cacheable
    /// * c = false, irgn = true => Normal memory, Inner Write-Back Write-Allocate Cacheable
    /// * c = true, irgn = true => Normal memory, Inner Write-Back no Write-Allocate Cacheable
    #[bit(0, rw)]
    c: bool,
}

impl Ttbr0 {
    /// Get the address
    ///
    /// Returns a full 32-bit memory address.
    pub fn get_address(&self) -> usize {
        (self.raw_value() & 0xFFFF_FF80) as usize
    }

    /// Set the address
    ///
    /// Pass a full 32-bit memory address. It will be shifted before being stored in this value.
    pub fn set_address(&mut self, address: usize) {
        let addr = u25::from_u32((address >> 7) as u32);
        self.set_addr(addr);
    }

    /// Change the address
    ///
    /// Pass a full 32-bit memory address. It will be shifted before being stored in the returned value.
    pub fn with_address(self, address: usize) -> Self {
        let addr = u25::from_u32((address >> 7) as u32);
        self.with_addr(addr)
    }
}

/// Outer cacheability attributes
#[derive(Debug)]
#[bitbybit::bitenum(u2, exhaustive = true)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Region {
    /// Normal memory, Outer Non-cacheable
    NonCacheable = 0b00,
    /// Normal memory, Outer Write-Back Write-Allocate Cacheable
    WriteBackWriteAllocCacheable = 0b01,
    /// Normal memory, Outer Write-Through Cacheable
    WriteThroughCacheable = 0b10,
    /// Normal memory, Outer Write-Back no Write-Allocate Cacheable
    WriteBackNoWriteAllocCacheable = 0b11,
}

impl SysReg for Ttbr0 {
    const CP: u32 = 15;
    const CRN: u32 = 2;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Ttbr0 {}

impl crate::register::SysRegWrite for Ttbr0 {}

impl Ttbr0 {
    #[inline]
    /// Reads TTBR0 (*Translation Table Base Register 0*)
    pub fn read() -> Ttbr0 {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }

    #[inline]
    /// Writes TTBR0 (*Translation Table Base Register 0*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
