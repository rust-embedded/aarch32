use arbitrary_int::u3;

use crate::register::{Dccimvac, Dccisw, Dccmvac, Dccsw, Dcimvac, Dcisw, SysRegWrite};

/// Invalidate the full L1 data cache.
///
/// ## Generics
///
/// - A: log2(ASSOCIATIVITY) rounded up to the next integer if necessary. For example, a 4-way
///   associative cache will have a value of 2 and a 8-way associative cache will have a value of
///   3.
/// - N: log2(LINE LENGTH). For example, a 32-byte line length (4 words) will have a value of
///   5.
/// - S: log2(NUM OF SETS). For systems with a fixed cache size, the number of sets can be
///   calculated using `CACHE SIZE / (LINE LENGTH * ASSOCIATIVITY)`.
///   For example, a 4-way associative 32kB L1 cache with a 32-byte line length (4 words) will
///   have 32768 / (32 * 4) == 256 sets and S will have a value of 8.
#[inline]
pub fn invalidate_l1_data_cache<const A: usize, const N: usize, const S: usize>() {
    let ways = 1 << A;
    let sets = 1 << S;

    for set in 0..sets {
        for way in 0..ways {
            unsafe { Dcisw::write(Dcisw::new::<A, N>(way, set, u3::new(0))) };
        }
    }
}

/// Clean the full L1 data cache.
///
/// ## Generics
///
/// - A: log2(ASSOCIATIVITY) rounded up to the next integer if necessary. For example, a 4-way
///   associative cache will have a value of 2 and a 8-way associative cache will have a value of
///   3.
/// - N: log2(LINE LENGTH). For example, a 32-byte line length (4 words) will have a value of
///   5.
/// - S: log2(NUM OF SETS). For systems with a fixed cache size, the number of sets can be
///   calculated using `CACHE SIZE / (LINE LENGTH * ASSOCIATIVITY)`.
///   For example, a 4-way associative 32kB L1 cache with a 32-byte line length (4 words) will
///   have 32768 / (32 * 4) == 256 sets and S will have a value of 8.
#[inline]
pub fn clean_l1_data_cache<const A: usize, const N: usize, const S: usize>() {
    let ways = 1 << A;
    let sets = 1 << S;

    for set in 0..sets {
        for way in 0..ways {
            unsafe { Dccsw::write(Dccsw::new::<A, N>(way, set, u3::new(0))) };
        }
    }
}

/// Clean and Invalidate the full L1 data cache.
///
/// ## Generics
///
/// - A: log2(ASSOCIATIVITY) rounded up to the next integer if necessary. For example, a 4-way
///   associative cache will have a value of 2 and a 8-way associative cache will have a value of
///   3.
/// - N: log2(LINE LENGTH). For example, a 32-byte line length (4 words) will have a value of
///   5.
/// - S: log2(NUM OF SETS). For systems with a fixed cache size, the number of sets can be
///   calculated using `CACHE SIZE / (LINE LENGTH * ASSOCIATIVITY)`.
///   For example, a 4-way associative 32kB L1 cache with a 32-byte line length (4 words) will
///   have 32768 / (32 * 4) == 256 sets and S will have a value of 8.
#[inline]
pub fn clean_and_invalidate_l1_data_cache<const A: usize, const N: usize, const S: usize>() {
    let ways = 1 << A;
    let sets = 1 << S;

    for set in 0..sets {
        for way in 0..ways {
            unsafe { Dccisw::write(Dccisw::new::<A, N>(way, set, u3::new(0))) };
        }
    }
}

/// Invalidates a data cache line to the point of coherence.
///
/// See p.1735 of the ARMv7-A Architecture Reference Manual for more information.
#[inline]
pub fn invalidate_data_cache_line_to_poc(addr: u32) {
    unsafe {
        Dcimvac::write_raw(addr);
    }
}

/// Cleans a data cache line to the point of coherence.
///
/// See p.1735 of the ARMv7-A Architecture Reference Manual for more information.
#[inline]
pub fn clean_data_cache_line_to_poc(addr: u32) {
    unsafe {
        Dccmvac::write_raw(addr);
    }
}

/// Cleans and invalidates a data cache line to the point of coherence.
///
/// See p.1735 of the ARMv7-A Architecture Reference Manual for more information.
#[inline]
pub fn clean_and_invalidate_data_cache_line_to_poc(addr: u32) {
    unsafe {
        Dccimvac::write_raw(addr);
    }
}
