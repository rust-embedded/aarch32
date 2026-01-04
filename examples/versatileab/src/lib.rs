//! Common code for all examples

#![no_std]

/// The base address of our PL190 interrupt controller
pub const PL190_BASE_ADDRESS: usize = 0x1014_0000;

#[cfg(arm_architecture = "v8-r")]
compile_error!("This example/board is not compatible with the ARMv8-R architecture");

static WANT_PANIC: portable_atomic::AtomicBool = portable_atomic::AtomicBool::new(false);

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
#[cfg(target_os = "none")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::println!("PANIC: {:#?}", info);
    if WANT_PANIC.load(portable_atomic::Ordering::Relaxed) {
        semihosting::process::exit(0);
    } else {
        semihosting::process::abort();
    }
}

/// Set the panic function as no longer returning a failure code via semihosting
pub fn want_panic() {
    WANT_PANIC.store(true, portable_atomic::Ordering::Relaxed);
}
