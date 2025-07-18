use arbitrary_int::{u12, u2, u3, u4};

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("invalid L1 entry type {0:?}")]
pub struct InvalidL1EntryType(pub L1EntryType);

#[bitbybit::bitenum(u3, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq)]
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
    pub const fn new(apx: bool, ap: u2) -> Self {
        Self::new_with_raw_value(u3::new(((apx as u8) << 2) | ap.value()))
    }

    /// AP bit for the given access permission.
    #[inline]
    pub const fn ap(&self) -> u2 {
        u2::new((*self as u8) & 0b11)
    }

    /// APX bit for the given access permission.
    #[inline]
    pub const fn apx(&self) -> bool {
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

/// Individual section attributes for a L1 section.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    /// Extract the section attributes from a raw L1 section entry.
    #[inline]
    fn from_raw(raw: u32) -> Result<Self, InvalidL1EntryType> {
        let section_type = L1EntryType::new_with_raw_value(u2::new((raw & 0b11) as u8));
        if section_type != L1EntryType::Section {
            return Err(InvalidL1EntryType(section_type));
        }
        Ok(Self::from_raw_unchecked(raw))
    }

    /// Retrieves the corresponding L1 section part without the section base address being set.
    const fn l1_section_part(&self) -> L1Section {
        L1Section::builder()
            .with_base_addr_upper_bits(u12::new(0))
            .with_ng(self.non_global)
            .with_s(self.shareable)
            .with_apx(self.access.apx())
            .with_tex(self.memory_attrs.type_extensions)
            .with_ap(self.access.ap())
            .with_p_bit(self.p_bit)
            .with_domain(self.domain)
            .with_xn(self.execute_never)
            .with_c(self.memory_attrs.c)
            .with_b(self.memory_attrs.b)
            .with_entry_type(L1EntryType::Section)
            .build()
    }

    /// Extract the section attributes without checking the entry type bits.
    #[inline]
    const fn from_raw_unchecked(raw: u32) -> Self {
        let l1 = L1Section::new_with_raw_value(raw);
        Self {
            non_global: l1.ng(),
            shareable: l1.s(),
            p_bit: l1.p_bit(),
            access: AccessPermissions::new(l1.apx(), l1.ap()),
            memory_attrs: MemoryRegionAttributesRaw::new(l1.tex(), l1.c(), l1.b()),
            domain: l1.domain(),
            execute_never: l1.xn(),
        }
    }
}

/// 1 MB section translation entry, mapping a 1 MB region to a physical address.
///
/// The ARM Cortex-A architecture programmers manual chapter 9.4 (p.163) or the ARMv7-A and ArmV7-R
/// architecture reference manual p.1323 specify these attributes in more detail.
#[bitbybit::bitfield(u32, default = 0x00)]
#[derive(PartialEq, Eq)]
pub struct L1Section {
    /// Section base address upper bits.
    #[bits(20..=31, rw)]
    base_addr_upper_bits: u12,
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
            self.base_addr_upper_bits(),
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
    pub const fn new_with_addr_and_attrs(phys_addr: u32, section_attrs: SectionAttributes) -> Self {
        // Must be aligned to 1 MB
        if phys_addr & 0x000F_FFFF != 0 {
            panic!("physical base address for L1 section must be aligned to 1 MB");
        }
        Self::new_with_addr_upper_bits_and_attrs(u12::new((phys_addr >> 20) as u16), section_attrs)
    }

    /// Retrieve the section attributes.
    #[inline]
    pub fn section_attrs(&self) -> Result<SectionAttributes, InvalidL1EntryType> {
        SectionAttributes::from_raw(self.raw_value())
    }

    /// Set the section attributes without changing the address.
    #[inline]
    pub fn set_section_attrs(&mut self, section_attrs: SectionAttributes) {
        *self = Self::new_with_addr_upper_bits_and_attrs(self.base_addr_upper_bits(), section_attrs)
    }

    /// Create a new L1 section with the given upper 12 bits of the address and section attributes.
    #[inline]
    pub const fn new_with_addr_upper_bits_and_attrs(
        addr_upper_twelve_bits: u12,
        section_attrs: SectionAttributes,
    ) -> Self {
        let attrs = section_attrs.l1_section_part();
        L1Section::builder()
            .with_base_addr_upper_bits(addr_upper_twelve_bits)
            .with_ng(attrs.ng())
            .with_s(attrs.s())
            .with_apx(attrs.apx())
            .with_tex(attrs.tex())
            .with_ap(attrs.ap())
            .with_p_bit(attrs.p_bit())
            .with_domain(attrs.domain())
            .with_xn(attrs.xn())
            .with_c(attrs.c())
            .with_b(attrs.b())
            .with_entry_type(attrs.entry_type())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECTION_ATTRS_DEVICE_PERIPHERAL: SectionAttributes = SectionAttributes {
        non_global: false,
        p_bit: false,
        shareable: false,
        // APX false, AP 0b11
        access: AccessPermissions::FullAccess,
        // TEX 0b000, c false, b true
        memory_attrs: MemoryRegionAttributes::ShareableDevice.as_raw(),
        domain: u4::new(0b1111),
        execute_never: false,
    };
    /// Address upper 12 bits: 0b1
    const L1_SECTION_PERIPHERAL: L1Section =
        L1Section::new_with_addr_and_attrs(0x100000, SECTION_ATTRS_DEVICE_PERIPHERAL);

    // Fully cacheable normal memory (for example DDR with L1 and L2 cache)
    const SECTION_ATTRS_FULL_CACHEABLE: SectionAttributes = SectionAttributes {
        non_global: false,
        p_bit: false,
        shareable: true,
        // APX false, AP 0b11
        access: AccessPermissions::FullAccess,
        // TEX 0b101, c false, b true
        memory_attrs: MemoryRegionAttributes::CacheableMemory {
            inner: CacheableMemoryAttribute::WriteBackWriteAlloc,
            outer: CacheableMemoryAttribute::WriteBackWriteAlloc,
        }
        .as_raw(),
        domain: u4::new(0b1010),
        execute_never: false,
    };
    /// Address upper 12 bits: 0b10
    const L1_SECTION_MEMORY: L1Section =
        L1Section::new_with_addr_and_attrs(0x200000, SECTION_ATTRS_FULL_CACHEABLE);

    #[test]
    pub fn basic_test_peripheral_memory() {
        assert_eq!(L1_SECTION_PERIPHERAL.raw_value(), 0x100DE6);
        assert_eq!(L1_SECTION_PERIPHERAL.base_addr_upper_bits(), u12::new(0b1));
        assert_eq!(
            L1_SECTION_PERIPHERAL
                .section_attrs()
                .expect("invalid type field"),
            SECTION_ATTRS_DEVICE_PERIPHERAL
        );
        assert!(!L1_SECTION_PERIPHERAL.ng());
        assert!(!L1_SECTION_PERIPHERAL.p_bit());
        assert!(!L1_SECTION_PERIPHERAL.s());
        assert!(!L1_SECTION_PERIPHERAL.apx());
        assert_eq!(L1_SECTION_PERIPHERAL.ap(), u2::new(0b11));
        assert_eq!(L1_SECTION_PERIPHERAL.tex(), u3::new(0b000));
        assert!(!L1_SECTION_PERIPHERAL.c());
        assert!(L1_SECTION_PERIPHERAL.b());
        assert_eq!(L1_SECTION_PERIPHERAL.domain(), u4::new(0b1111));
        assert!(!L1_SECTION_PERIPHERAL.xn());
    }

    #[test]
    pub fn basic_test_normal_memory() {
        assert_eq!(L1_SECTION_MEMORY.raw_value(), 0x215D46);
        assert_eq!(L1_SECTION_MEMORY.base_addr_upper_bits(), u12::new(0b10));
        assert_eq!(
            L1_SECTION_MEMORY
                .section_attrs()
                .expect("invalid type field"),
            SECTION_ATTRS_FULL_CACHEABLE
        );
        assert!(!L1_SECTION_MEMORY.ng());
        assert!(!L1_SECTION_MEMORY.p_bit());
        assert!(L1_SECTION_MEMORY.s());
        assert!(!L1_SECTION_MEMORY.apx());
        assert_eq!(L1_SECTION_MEMORY.ap(), u2::new(0b11));
        assert_eq!(L1_SECTION_MEMORY.tex(), u3::new(0b101));
        assert!(!L1_SECTION_MEMORY.c());
        assert!(L1_SECTION_MEMORY.b());
        assert_eq!(L1_SECTION_MEMORY.domain(), u4::new(0b1010));
        assert!(!L1_SECTION_MEMORY.xn());
    }

    #[test]
    pub fn update_fields() {
        let mut l1 = L1_SECTION_MEMORY;
        let new_attrs = SectionAttributes {
            non_global: true,
            p_bit: true,
            shareable: false,
            // APX true, AP 0b10
            access: AccessPermissions::ReadOnly,
            // TEX 0b000, c false, b false
            memory_attrs: MemoryRegionAttributes::StronglyOrdered.as_raw(),
            domain: u4::new(0b1001),
            execute_never: true,
        };
        l1.set_section_attrs(new_attrs);
        assert_eq!(l1.raw_value(), 0x228B32);
        assert_eq!(l1.base_addr_upper_bits(), u12::new(0b10));
        assert_eq!(l1.section_attrs().unwrap(), new_attrs);
        assert!(l1.ng());
        assert!(l1.p_bit());
        assert!(!l1.s());
        assert!(l1.apx());
        assert_eq!(l1.ap(), u2::new(0b10));
        assert_eq!(l1.tex(), u3::new(0b000));
        assert!(!l1.c());
        assert!(!l1.b());
        assert_eq!(l1.domain(), u4::new(0b1001));
        assert!(l1.xn());
    }

    #[test]
    #[should_panic(expected = "physical base address for L1 section must be aligned to 1 MB")]
    pub fn unaligned_section_address() {
        L1Section::new_with_addr_and_attrs(0x100001, SECTION_ATTRS_DEVICE_PERIPHERAL);
    }
}
