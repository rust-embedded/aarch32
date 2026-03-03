//! Common code for all examples

#![no_std]

#[cfg(arm_architecture = "v7-a")]
pub mod mmu;

/// The base address of our PL190 interrupt controller
pub const PL190_BASE_ADDRESS: usize = 0x1014_0000;

#[cfg(arm_architecture = "v8-r")]
compile_error!("This example/board is not compatible with the ARMv8-R architecture");

static WANT_PANIC: portable_atomic::AtomicBool = portable_atomic::AtomicBool::new(false);

/// Track if we're already in the exit routine.
///
/// Stops us doing infinite recursion if we panic whilst doing the stack reporting.
static IN_EXIT: portable_atomic::AtomicBool = portable_atomic::AtomicBool::new(false);

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
#[cfg(target_os = "none")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::println!("PANIC: {:#?}", info);
    if WANT_PANIC.load(portable_atomic::Ordering::Relaxed) {
        exit(0);
    } else {
        exit(1);
    }
}

/// Set the panic function as no longer returning a failure code via semihosting
pub fn want_panic() {
    WANT_PANIC.store(true, portable_atomic::Ordering::Relaxed);
}

/// Init the hardware
///
/// Includes enabling the MMU (if we have one)
pub fn init() {
    #[cfg(arm_architecture = "v7-a")]
    mmu::set_mmu();

    #[cfg(arm_architecture = "v7-a")]
    mmu::enable_mmu_and_cache();

    #[cfg(arm_architecture = "v7-r")]
    aarch32_cpu::register::Sctlr::modify(|s| {
        // Enable Cache
        s.set_c(true);
    });
}

/// Exit from QEMU with code
pub fn exit(code: i32) -> ! {
    if !IN_EXIT.swap(true, portable_atomic::Ordering::Relaxed) {
        stack_dump();
    }
    semihosting::process::exit(code)
}

/// Print stack using to semihosting output for each stack
///
/// Produces output like:
///
/// ```text
/// Stack usage report:
/// UND0 Stack =      0 used of  16384 bytes (000%) @ 0x1006bf80..0x1006ff80
/// SVC0 Stack =      0 used of  16384 bytes (000%) @ 0x1006ff80..0x10073f80
/// ABT0 Stack =      0 used of  16384 bytes (000%) @ 0x10073f80..0x10077f80
/// HYP0 Stack =      0 used of  16384 bytes (000%) @ 0x10077f80..0x1007bf80
/// IRQ0 Stack =      0 used of     64 bytes (000%) @ 0x1007bf80..0x1007bfc0
/// FIQ0 Stack =      0 used of     64 bytes (000%) @ 0x1007bfc0..0x1007c000
/// SYS0 Stack =   2416 used of  16384 bytes (014%) @ 0x1007c000..0x10080000
/// ```
fn stack_dump() {
    use aarch32_cpu::stacks::stack_used_bytes;
    use aarch32_rt::stacks::Stack;

    semihosting::eprintln!("Stack usage report:");

    unsafe {
        for stack in Stack::iter() {
            for core in (0..Stack::num_cores()).rev() {
                let core_range = stack.range(core).unwrap();
                let (total, used) = stack_used_bytes(core_range.clone());
                let percent = used * 100 / total;
                // Send to stderr, so it doesn't mix with expected output on stdout
                semihosting::eprintln!(
                    "{}{} Stack = {:6} used of {:6} bytes ({:03}%) @ {:08x?}",
                    stack,
                    core,
                    used,
                    total,
                    percent,
                    core_range
                );
            }
        }
    }
}
