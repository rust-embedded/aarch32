//! Code for checking stack usage

/// Reports stack usage as a count of bytes
///
/// It starts at the lower bound, and looks for values that are set to 0,
/// concluding that those values have never been used. It returns `(total,
/// used)` in bytes.
///
/// # Safety
///
/// Pass a range of valid, readable, memory with 32-bit aligned addresses.
pub unsafe fn stack_used_bytes(stack: core::ops::Range<*const u32>) -> (usize, usize) {
    let size_words = unsafe { stack.end.offset_from(stack.start) } as usize;
    let unused_words = unsafe { stack_unused_bytes_asm(stack.start, size_words) };
    let used_words = size_words - unused_words;
    (size_words * size_of::<u32>(), used_words * size_of::<u32>())
}

/// Counts number of words that are equal to zero
///
/// Written in Arm assembly to avoid any issues with pointing at things that are
/// not validly initialised integers (as far as Rust is concerned).
///
/// Returns a count of the number of contiguous words equal to 0x0 at `start`, with a
/// maximum of `size` words
///
/// # Safety
///
/// The address `start` must be correctly aligned, and point to a region of memory
/// of at least `size` words in length.
unsafe fn stack_unused_bytes_asm(start: *const u32, size: usize) -> usize {
    let result: usize;
    unsafe {
        core::arch::asm!(
            r#"
            // skip out if size is zero
            movs    {result}, #0
            cmp     {size}, #0
            beq     3f
    2:      // loop
            ldr     {scratch}, [{start}]
            cmp     {scratch}, #0
            // break out if value is non-zero
            bne     3f
            // otherwise increment counter
            adds    {result}, {result}, #1
            adds    {start}, {start}, #4
            // loop if not finished yet
            cmp     {result}, {size}
            bne     2b
            // all finished
    3:
            "#,
            size = in(reg) size,
            start = inout(reg) start => _,
            result = out(reg) result,
            scratch = out(reg) _,
        );
    }
    result
}
