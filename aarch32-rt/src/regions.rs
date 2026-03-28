//! Code to get the memory regions from the linker script
//!
//! This is useful if you want to set up the MPU

/// Represents one of the memory regions in the linker script
#[derive(Debug, Copy, Clone)]
pub enum Region {
    /// The .vector_table section
    ///
    /// Contains the reset and exception vectors
    VectorTable,
    /// The .text section
    ///
    /// Contains the executable code
    Text,
    /// The .rodata section
    ///
    /// Contains read-only static data
    Rodata,
    /// The .bss section
    ///
    /// Contains zero-initialised static data
    Bss,
    /// The .data section
    ///
    /// Contains non-zero-initialised static data
    Data,
    /// The .uninit section
    ///
    /// Contains non-initialised static data
    Uninit,
}

impl core::fmt::Display for Region {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.pad(match self {
            Region::VectorTable => ".vector_table",
            Region::Text => ".text",
            Region::Rodata => ".rodata",
            Region::Bss => ".bss",
            Region::Data => ".data",
            Region::Uninit => ".uninit",
        })
    }
}

impl Region {
    /// Create an iterator over all the regions
    pub fn iter() -> impl Iterator<Item = Region> {
        RegionIter::new()
    }

    /// Get the highest address of this region
    ///
    /// Technically it gets *one past the end* of the region.
    pub fn top(&self) -> *const u8 {
        use core::ptr::addr_of;
        unsafe extern "C" {
            static __evector: u8;
            static __etext: u8;
            static __erodata: u8;
            static __ebss: u8;
            static __edata: u8;
            static __euninit: u8;
        }
        match self {
            Region::VectorTable => addr_of!(__evector),
            Region::Text => addr_of!(__etext),
            Region::Rodata => addr_of!(__erodata),
            Region::Bss => addr_of!(__ebss),
            Region::Data => addr_of!(__edata),
            Region::Uninit => addr_of!(__euninit),
        }
    }

    /// Get the lowest address of this region
    pub fn bottom(&self) -> *const u8 {
        use core::ptr::addr_of;
        unsafe extern "C" {
            static __svector: u8;
            static __stext: u8;
            static __srodata: u8;
            static __sbss: u8;
            static __sdata: u8;
            static __suninit: u8;
        }
        match self {
            Region::VectorTable => addr_of!(__svector),
            Region::Text => addr_of!(__stext),
            Region::Rodata => addr_of!(__srodata),
            Region::Bss => addr_of!(__sbss),
            Region::Data => addr_of!(__sdata),
            Region::Uninit => addr_of!(__suninit),
        }
    }

    /// Get the range of this region.
    pub fn range(&self) -> Option<core::ops::Range<*const u8>> {
        let bottom = self.bottom();
        let top = self.top();
        if bottom != top {
            Some(bottom..top)
        } else {
            None
        }
    }

    /// Get the inclusive range of this region.
    ///
    /// This is the range you need to give to the PMSAv8 MPU code.
    pub fn mpu_range(&self) -> Option<core::ops::RangeInclusive<*const u8>> {
        let bottom = self.bottom();
        let top = self.top();
        let top_under = unsafe { top.offset(-1) };
        if bottom != top {
            Some(bottom..=top_under)
        } else {
            None
        }
    }
}

/// Iterator over all the [`Region`] variants
pub struct RegionIter {
    next: Option<Region>,
}

impl RegionIter {
    /// Create a new [`RegionIter`]
    pub fn new() -> Self {
        Self {
            next: Some(Region::VectorTable),
        }
    }
}

impl Default for RegionIter {
    fn default() -> Self {
        RegionIter::new()
    }
}

impl Iterator for RegionIter {
    type Item = Region;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        self.next = match self.next {
            Some(Region::VectorTable) => Some(Region::Text),
            Some(Region::Text) => Some(Region::Rodata),
            Some(Region::Rodata) => Some(Region::Bss),
            Some(Region::Bss) => Some(Region::Data),
            Some(Region::Data) => Some(Region::Uninit),
            Some(Region::Uninit) | None => None,
        };
        current
    }
}
