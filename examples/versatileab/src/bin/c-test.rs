//! EABI checking example program

#![no_std]
#![no_main]

use aarch32_rt::entry;
use semihosting::println;
use versatileab as _;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn my_main() -> ! {
    versatileab::init();
    println!("c_code_char_check(p) -> {}", unsafe {
        c_code::c_code_char_check(1)
    });
    println!("c_code_unsigned_char_check(p) -> {}", unsafe {
        c_code::c_code_unsigned_char_check(1)
    });
    println!("c_code_signed_char_check(p) -> {}", unsafe {
        c_code::c_code_signed_char_check(1)
    });
    println!("c_code_short_check(p) -> {}", unsafe {
        c_code::c_code_short_check(1)
    });
    println!("c_code_unsigned_short_check(p) -> {}", unsafe {
        c_code::c_code_unsigned_short_check(1)
    });
    println!("c_code_int_check(p) -> {}", unsafe {
        c_code::c_code_int_check(1)
    });
    println!("c_code_unsigned_int_check(p) -> {}", unsafe {
        c_code::c_code_unsigned_int_check(1)
    });
    println!("c_code_long_check(p) -> {}", unsafe {
        c_code::c_code_long_check(1)
    });
    println!("c_code_unsigned_long_check(p) -> {}", unsafe {
        c_code::c_code_unsigned_long_check(1)
    });
    let mut integers = [1, 2];
    println!("c_code_int_ptr_check(p) -> {}", unsafe {
        *c_code::c_code_int_ptr_check(integers.as_mut_ptr())
    });
    println!("c_code_float_check(p) -> {}", unsafe {
        c_code::c_code_float_check(1.0)
    });
    println!("c_code_double_check(p) -> {}", unsafe {
        c_code::c_code_double_check(1.0)
    });

    let mut data_in = c_code::c_code_data_t {
        c: 1,
        s: 1,
        i: 1,
        l: 1,
        f: 1.0,
        d: 1.0,
        e: c_code::c_code_sample_enum_t_ITEM_TWO,
    };
    println!("data_in: {:x?}", data_in);

    println!("c_code_struct_check(data_in) -> {:x?}", unsafe {
        c_code::c_code_struct_check(data_in)
    });

    unsafe { c_code::c_code_struct_ref_check(&mut data_in) };
    println!("called c_code_struct_ref_check, data_in is {:x?}", data_in);

    versatileab::exit(0);
}
