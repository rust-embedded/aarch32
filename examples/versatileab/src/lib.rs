//! Common code for all examples

#![no_std]

/// The base address of our PL190 interrupt controller
pub const PL190_BASE_ADDRESS: usize = 0x1014_0000;

#[cfg(arm_architecture = "v8-r")]
compile_error!("This example/board is not compatible with the ARMv8-R architecture");

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
#[cfg(target_os = "none")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::println!("PANIC: {:#?}", info);
    semihosting::process::abort();
}
