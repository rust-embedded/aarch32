use arbitrary_int::{u12, u2, u3, u4};

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("invalid L1 entry type {0:?}")]
pub struct InvalidL1EntryType(pub L1EntryType);

#[bitbybit::bitenum(u3, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
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
    #[inline]
    const fn ap(&self) -> u8 {
        (*self as u8) & 0b11
    }

    #[inline]
    const fn apx(&self) -> bool {
        (*self as u8) > (AccessPermissions::FullAccess as u8)
    }
}

#[bitbybit::bitenum(u2, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MemoryRegionAttributesRaw {
    /// TEX bits
    type_extensions: u3,
    c: bool,
    b: bool,
}

impl MemoryRegionAttributesRaw {
    #[inline]
    pub const fn new(type_extensions: u3, c: bool, b: bool) -> Self {
        Self {
            type_extensions,
            c,
            b,
        }
    }
}

#[bitbybit::bitenum(u2, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum CacheableMemoryAttribute {
    NonCacheable = 0b00,
    WriteBackWriteAlloc = 0b01,
    WriteThroughNoWriteAlloc = 0b10,
    WriteBackNoWriteAlloc = 0b11,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
                MemoryRegionAttributesRaw::new(u3::new(0b000), false, false)
            }
            MemoryRegionAttributes::ShareableDevice => {
                MemoryRegionAttributesRaw::new(u3::new(0b000), false, true)
            }
            MemoryRegionAttributes::OuterAndInnerWriteThroughNoWriteAlloc => {
                MemoryRegionAttributesRaw::new(u3::new(0b000), true, false)
            }
            MemoryRegionAttributes::OuterAndInnerWriteBackNoWriteAlloc => {
                MemoryRegionAttributesRaw::new(u3::new(0b000), true, true)
            }
            MemoryRegionAttributes::OuterAndInnerNonCacheable => {
                MemoryRegionAttributesRaw::new(u3::new(0b001), false, false)
            }
            MemoryRegionAttributes::OuterAndInnerWriteBackWriteAlloc => {
                MemoryRegionAttributesRaw::new(u3::new(0b001), true, true)
            }
            MemoryRegionAttributes::NonShareableDevice => {
                MemoryRegionAttributesRaw::new(u3::new(0b010), false, false)
            }
            MemoryRegionAttributes::CacheableMemory { inner, outer } => {
                MemoryRegionAttributesRaw::new(
                    u3::new((1 << 2) | (outer.raw_value().value())),
                    (*inner as u8 & 0b10) != 0,
                    (*inner as u8 & 0b01) != 0,
                )
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SectionAttributes {
    /// NG bit
    pub non_global: bool,
    /// Implementation defined bit.
    pub p_bit: bool,
    pub shareable: bool,
    /// AP bits
    pub access: AccessPermissions,
    pub memory_attrs: MemoryRegionAttributesRaw,
    pub domain: u4,
    /// xN bit.
    pub execute_never: bool,
}

impl SectionAttributes {
    /// Lower 18 bits of the L1 section entry.
    #[inline]
    pub const fn raw(&self) -> u32 {
        ((self.non_global as u32) << 17)
            | ((self.shareable as u32) << 16)
            | ((self.access.apx() as u32) << 15)
            | ((self.memory_attrs.type_extensions.value() as u32) << 12)
            | ((self.access.ap() as u32) << 10)
            | ((self.p_bit as u32) << 9)
            | ((self.domain.value() as u32) << 5)
            | ((self.execute_never as u32) << 4)
            | ((self.memory_attrs.c as u32) << 3)
            | ((self.memory_attrs.b as u32) << 2)
            | L1EntryType::Section as u32
    }

    /// Extract the section attributes from a raw L1 section entry.
    #[inline]
    pub fn from_raw(raw: u32) -> Result<Self, InvalidL1EntryType> {
        let section_type = L1EntryType::new_with_raw_value(u2::new((raw & 0b11) as u8));
        if section_type != L1EntryType::Section {
            return Err(InvalidL1EntryType(section_type));
        }
        Ok(Self::from_raw_unchecked(raw))
    }

    /// Extract the section attributes without checking the entry type bits.
    #[inline]
    pub const fn from_raw_unchecked(raw: u32) -> Self {
        Self {
            non_global: (raw >> 17) & 0x1 != 0,
            shareable: (raw >> 16) & 0x1 != 0,
            p_bit: (raw >> 9) & 0x1 != 0,
            access: AccessPermissions::new_with_raw_value(u3::new(
                ((((raw >> 15) & 0b1) as u8) << 2) | (((raw >> 10) & 0b11) as u8),
            )),
            memory_attrs: MemoryRegionAttributesRaw::new(
                u3::new(((raw >> 12) & 0b111) as u8),
                ((raw >> 3) & 0b1) != 0,
                ((raw >> 2) & 0b1) as u8 != 0,
            ),
            domain: u4::new(((raw >> 5) & 0b1111) as u8),
            execute_never: ((raw >> 4) & 0b1) != 0,
        }
    }
}

/// 1 MB section translation entry, mapping a 1 MB region to a physical address.
///
/// The ARM Cortex-A architecture programmers manual chapter 9.4 (p.163) specifies these attributes
/// in more detail.
#[bitbybit::bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct L1Section {
    /// Section base address.
    #[bits(20..=31, rw)]
    base_addr: u12,
    /// Non-global bit.
    #[bit(16, rw)]
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
        Self::new_with_raw_value((higher_bits << 20) | section_attrs.raw())
    }
}
