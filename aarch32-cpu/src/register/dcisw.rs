//! DCISW (*Invalidate Data or Unified Cache line by Set/Way.*)

use arbitrary_int::u3;

use crate::register::{SysReg, SysRegWrite};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dcisw(pub u32);

impl Dcisw {
    /// Create DCISW value for cache invalidation by set and way.
    ///
    /// ## Generics
    ///
    /// - A: log2(ASSOCIATIVITY) rounded up to the next integer if necessary. For example, a 4-way
    ///   associative cache will have a value of 2 and a 8-way associative cache will have a value of
    ///   3.
    /// - N: log2(LINE LENGTH). For example, a 32-byte line length (4 words) will have a value of
    ///   5.
    #[inline]
    pub const fn new<const A: usize, const N: usize>(way: u8, set: u16, level: u3) -> Self {
        Self(super::dc_sw_ops::new::<A, N>(way, set, level))
    }

    /// Create DCISW value for cache invalidation by set and way.
    /// Returns [None] on invalid input.
    ///
    /// # Arguments
    ///
    /// - a: log2(ASSOCIATIVITY) rounded up to the next integer if necessary. For example, a 4-way
    ///   associative cache will have a value of 2 and a 8-way associative cache will have a value of
    ///   3.
    /// - n: log2(LINE LENGTH). For example, a 32-byte line length (4 words) will have a value of
    ///   5.
    #[inline]
    pub const fn new_with_offsets(a: usize, way: u8, n: usize, set: u16, level: u3) -> Self {
        Self(super::dc_sw_ops::new_with_offsets(a, way, n, set, level))
    }
}
impl SysReg for Dcisw {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 2;
}

impl crate::register::SysRegWrite for Dcisw {}

impl Dcisw {
    #[inline]
    /// Writes DCSW (*Invalidate Data or Unified Cache line by Set/Way.*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
