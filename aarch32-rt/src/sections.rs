//! Code to get the memory regions from the linker script
//!
//! This is useful if you want to set up the MPU

/// Represents one of the memory regions in the linker script
#[derive(Debug, Copy, Clone)]
pub enum Section {
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

impl core::fmt::Display for Section {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(match self {
            Section::VectorTable => ".vector_table",
            Section::Text => ".text",
            Section::Rodata => ".rodata",
            Section::Bss => ".bss",
            Section::Data => ".data",
            Section::Uninit => ".uninit",
        })
    }
}

impl Section {
    /// Create an iterator over all the regions
    pub fn iter() -> impl Iterator<Item = Section> {
        SectionIter::new()
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
            Section::VectorTable => addr_of!(__evector),
            Section::Text => addr_of!(__etext),
            Section::Rodata => addr_of!(__erodata),
            Section::Bss => addr_of!(__ebss),
            Section::Data => addr_of!(__edata),
            Section::Uninit => addr_of!(__euninit),
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
            Section::VectorTable => addr_of!(__svector),
            Section::Text => addr_of!(__stext),
            Section::Rodata => addr_of!(__srodata),
            Section::Bss => addr_of!(__sbss),
            Section::Data => addr_of!(__sdata),
            Section::Uninit => addr_of!(__suninit),
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
pub struct SectionIter {
    next: Option<Section>,
}

impl SectionIter {
    /// Create a new [`RegionIter`]
    pub fn new() -> Self {
        Self {
            next: Some(Section::VectorTable),
        }
    }
}

impl Default for SectionIter {
    fn default() -> Self {
        SectionIter::new()
    }
}

impl Iterator for SectionIter {
    type Item = Section;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        self.next = match self.next {
            Some(Section::VectorTable) => Some(Section::Text),
            Some(Section::Text) => Some(Section::Rodata),
            Some(Section::Rodata) => Some(Section::Bss),
            Some(Section::Bss) => Some(Section::Data),
            Some(Section::Data) => Some(Section::Uninit),
            Some(Section::Uninit) | None => None,
        };
        current
    }
}
