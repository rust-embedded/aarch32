use arbitrary_int::{u12, u2, u3, u4};

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum AccessPermissions {
    PermissionFault = 0b000,
    PrivilegedOnly = 0b001,
    NoUserWrite = 0b010,
    FullAccess = 0b011,
    _Reserved1 = 0b100,
    PrivilegedReadOnly = 0b101,
    ReadOnly = 0b110,
    _Reserved2 = 0b111,
}

impl AccessPermissions {
    const fn ap(&self) -> u8 {
        (*self as u8) & 0b11
    }

    const fn apx(&self) -> bool {
        (*self as u8) > (AccessPermissions::FullAccess as u8)
    }
}

#[derive(Debug)]
#[repr(u8)]
#[bitbybit::bitenum(u2, exhaustive = true)]
pub enum L1EntryType {
    /// Access generates an abort exception. Indicates an unmapped virtual address.
    Fault = 0b00,
    /// Entry points to a L2 translation table, allowing 1 MB of memory to be further divided
    PageTable = 0b01,
    /// Maps a 1 MB region to a physical address.
    Section = 0b10,
    /// Special 1MB section entry which requires 16 entries in the translation table.
    Supersection = 0b11,
}

/// The ARM Cortex-A architecture reference manual p.1363 specifies these attributes in more detail.
///
/// The B (Bufferable), C (Cacheable), and TEX (Type extension) bit names are inherited from
/// earlier versions of the architecture. These names no longer adequately describe the function
/// of the B, C, and TEX bits.
#[derive(Debug, Copy, Clone)]
pub struct MemoryRegionAttributesRaw {
    /// TEX bits
    type_extensions: u8,
    c: bool,
    b: bool,
}

impl MemoryRegionAttributesRaw {
    pub const fn new(type_extensions: u8, c: bool, b: bool) -> Self {
        Self {
            type_extensions,
            c,
            b,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CacheableMemoryAttribute {
    NonCacheable = 0b00,
    WriteBackWriteAlloc = 0b01,
    WriteThroughNoWriteAlloc = 0b10,
    WriteBackNoWriteAlloc = 0b11,
}

#[derive(Debug, Copy, Clone)]
pub enum MemoryRegionAttributes {
    StronglyOrdered,
    ShareableDevice,
    OuterAndInnerWriteThroughNoWriteAlloc,
    OuterAndInnerWriteBackNoWriteAlloc,
    OuterAndInnerNonCacheable,
    OuterAndInnerWriteBackWriteAlloc,
    NonShareableDevice,
    CacheableMemory {
        inner: CacheableMemoryAttribute,
        outer: CacheableMemoryAttribute,
    },
}

impl MemoryRegionAttributes {
    pub const fn as_raw(&self) -> MemoryRegionAttributesRaw {
        match self {
            MemoryRegionAttributes::StronglyOrdered => {
                MemoryRegionAttributesRaw::new(0b000, false, false)
            }
            MemoryRegionAttributes::ShareableDevice => {
                MemoryRegionAttributesRaw::new(0b000, false, true)
            }
            MemoryRegionAttributes::OuterAndInnerWriteThroughNoWriteAlloc => {
                MemoryRegionAttributesRaw::new(0b000, true, false)
            }
            MemoryRegionAttributes::OuterAndInnerWriteBackNoWriteAlloc => {
                MemoryRegionAttributesRaw::new(0b000, true, true)
            }
            MemoryRegionAttributes::OuterAndInnerNonCacheable => {
                MemoryRegionAttributesRaw::new(0b001, false, false)
            }
            MemoryRegionAttributes::OuterAndInnerWriteBackWriteAlloc => {
                MemoryRegionAttributesRaw::new(0b001, true, true)
            }
            MemoryRegionAttributes::NonShareableDevice => {
                MemoryRegionAttributesRaw::new(0b010, false, false)
            }
            MemoryRegionAttributes::CacheableMemory { inner, outer } => {
                MemoryRegionAttributesRaw::new(
                    (1 << 2) | (*outer as u8),
                    (*inner as u8 & 0b10) != 0,
                    (*inner as u8 & 0b01) != 0,
                )
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SectionAttributes {
    /// NG bit
    pub non_global: bool,
    /// Implementation defined bit.
    pub p_bit: bool,
    pub shareable: bool,
    /// AP bits
    pub access: AccessPermissions,
    pub memory_attrs: MemoryRegionAttributesRaw,
    pub domain: u8,
    /// xN bit.
    pub execute_never: bool,
}

/// 1 MB section translation entry, mapping a 1 MB region to a physical address.
///
/// The ARM Cortex-A architecture programmers manual chapter 9.4 (p.163) specifies these attributes
/// in more detail.
#[bitbybit::bitfield(u32)]
pub struct L1Section {
    /// Section base address.
    #[bits(20..=31, rw)]
    base_addr: u12,
    /// Non-global bit.
    #[bit(17, rw)]
    ng: bool,
    /// Shareable bit.
    #[bit(16, rw)]
    s: bool,
    #[bit(15, rw)]
    apx: bool,
    /// Type extension bits.
    #[bits(12..=14, rw)]
    tex: u3,
    #[bits(10..=11, rw)]
    ap: u2,
    #[bit(9, rw)]
    p_bit: bool,
    #[bits(5..=8, rw)]
    domain: u4,
    #[bit(4, rw)]
    xn: bool,
    #[bit(3, rw)]
    c: bool,
    #[bit(2, rw)]
    b: bool,
    #[bits(0..=1, rw)]
    entry_type: L1EntryType,
}

impl core::fmt::Debug for L1Section {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "L1Section {{ base_addr={:#x} ng={} s={} apx={} tex={:#b} ap={:#b} domain={:#b} xn={} c={} b={} }}",
            self.base_addr(),
            self.ng() as u8,
            self.s() as u8,
            self.apx() as u8,
            self.tex(),
            self.ap(),
            self.domain(),
            self.xn() as u8,
            self.c() as u8,
            self.b() as u8,
        )
    }
}

impl L1Section {
    /// Generates a new L1 section from a physical address and section attributes.
    ///
    /// The uppermost 12 bits of the physical address define which 1 MB of virtual address space
    /// are being accessed. They will be stored in the L1 section table. This address MUST be
    /// aligned to 1 MB.
    ///
    /// # Panics
    ///
    /// Physcal address not aligned to 1 MB.
    pub const fn new(phys_addr: u32, section_attrs: SectionAttributes) -> Self {
        // Must be aligned to 1 MB
        if phys_addr & 0x000F_FFFF != 0 {
            panic!("physical base address for L1 section must be aligned to 1 MB");
        }
        let higher_bits = phys_addr >> 20;
        let raw = (higher_bits << 20)
            | ((section_attrs.non_global as u32) << 17)
            | ((section_attrs.shareable as u32) << 16)
            | ((section_attrs.access.apx() as u32) << 15)
            | ((section_attrs.memory_attrs.type_extensions as u32) << 12)
            | ((section_attrs.access.ap() as u32) << 10)
            | ((section_attrs.p_bit as u32) << 9)
            | ((section_attrs.domain as u32) << 5)
            | ((section_attrs.execute_never as u32) << 4)
            | ((section_attrs.memory_attrs.c as u32) << 3)
            | ((section_attrs.memory_attrs.b as u32) << 2)
            | L1EntryType::Section as u32;
        Self::new_with_raw_value(raw)
    }
}
