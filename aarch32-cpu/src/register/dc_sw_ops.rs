use arbitrary_int::u3;

/// Create input data register value for cache maintenance operations by set and way.
///
/// ## Generics
///
/// - A: log2(ASSOCIATIVITY) rounded up to the next integer if necessary. For example, a 4-way
///   associative cache will have a value of 2 and a 8-way associative cache will have a value of
///   4.
/// - N: log2(LINE LENGTH). For example, a 32-byte line length (4 words) will have a value of
///   5.
#[inline]
pub const fn new<const A: usize, const N: usize>(way: u8, set: u16, level: u3) -> u32 {
    if A == 0 {
        ((set as u32) << N) | level.value() as u32
    } else {
        ((way as u32) << (32 - A)) | ((set as u32) << N) | level.value() as u32
    }
}

/// Create input data register value for cache maintenance operations by set and way.
/// Returns [None] on invalid input.
///
/// # Arguments
///
/// - a: log2(ASSOCIATIVITY) rounded up to the next integer if necessary. For example, a 4-way
///   associative cache will have a value of 2 and a 8-way associative cache will have a value of
///   4.
/// - n: log2(LINE LENGTH). For example, a 32-byte line length (4 words) will have a value of
///   5.
#[inline]
pub const fn new_with_offsets(a: usize, way: u8, n: usize, set: u16, level: u3) -> u32 {
    if a == 0 {
        ((set as u32) << n) | level.value() as u32
    } else {
        ((way as u32) << (32 - a)) | ((set as u32) << n) | level.value() as u32
    }
}
