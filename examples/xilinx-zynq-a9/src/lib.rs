#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
pub mod mmu;

static WANT_PANIC: AtomicBool = AtomicBool::new(false);

/// Track if we're already in the exit routine.
///
/// Stops us doing infinite recursion if we panic whilst doing the stack reporting.
static IN_EXIT: AtomicBool = AtomicBool::new(false);

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
#[cfg(target_os = "none")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::println!("PANIC: {:#?}", info);
    if WANT_PANIC.load(Ordering::Relaxed) {
        exit(0);
    } else {
        exit(1);
    }
}

/// Set the panic function as no longer returning a failure code via semihosting
pub fn want_panic() {
    WANT_PANIC.store(true, Ordering::Relaxed);
}

/// Init the hardware
///
/// Includes enabling the MMU. Each core must call this for itself, because the
/// MMU control registers (`TTBR0`, `SCTLR`) are per-core; they all point at the
/// same shared L1 page table.
pub fn init() {
    mmu::set_mmu();
    mmu::enable_mmu_and_cache();
}

static CORE1_RELEASE: AtomicU32 = AtomicU32::new(0);

/// Release core1 from spin loop
pub fn start_core1() {
    CORE1_RELEASE.store(1, Ordering::SeqCst);
    unsafe { core::arch::asm!("sev") };
}

/// Park function for secondary cores
///
/// We sleep the cores with a `WFE` and check a register in the FPGA to see if
/// it is time to boot.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn _asm_secondary_core_park() {
    core::arch::naked_asm!(
        r#"
        // Address of the release flag
        ldr     r0, ={release}
    1:
        // Wait until Core 0 does a 'sev'
        wfe
        // Spin until the flag is non-zero
        ldr     r1, [r0]
        cmp     r1, 0
        beq     1b
        // return to start-up
        bx      lr
    "#,
        release = sym CORE1_RELEASE,
    )
}

/// Exit from QEMU with code
pub fn exit(code: i32) -> ! {
    if !IN_EXIT.swap(true, Ordering::Relaxed) {
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

/// Represents the hardware we drive in our Zynq-7000 system.
pub struct Board {
    /// The Arm Generic Interrupt Controller (memory-mapped / GICv2 model)
    pub gic: arm_gic::gicv2::GicV2<'static>,
}

impl Board {
    /// Create a new board structure.
    ///
    /// Returns `Some(board)` the first time you call it, and `None` thereafter,
    /// so you cannot have two copies of the [`Board`] structure.
    pub fn new() -> Option<Board> {
        static TAKEN: AtomicBool = AtomicBool::new(false);
        if TAKEN.swap(true, Ordering::SeqCst) {
            // they already took the peripherals
            return None;
        }
        Some(Board {
            // SAFETY: This is the first and only call to `make_gic()`, as
            // guaranteed by the atomic flag check above.
            gic: unsafe { make_gic() },
        })
    }
}

/// The Cortex-A9 MPCore private peripheral base on the Zynq-7000.
///
/// This is fixed by the SoC (and matches the QEMU `xilinx-zynq-a9` machine). We
/// use a constant rather than reading `CBAR`, because `CBAR` on the Cortex-A9
/// uses a different encoding to the one `aarch32_cpu::register::ImpCbar` issues.
const PERIPHBASE: usize = 0xF8F0_0000;

/// Create the Arm GIC driver for the Cortex-A9 MPCore.
///
/// The Cortex-A9 uses the memory-mapped GIC (the GICv2 programming model). The
/// Distributor sits at `PERIPHBASE + 0x1000` and the CPU interface at
/// `PERIPHBASE + 0x100`. Both regions fall inside the device memory mapped by
/// [`mmu`].
///
/// # Safety
///
/// Only call this function once.
pub unsafe fn make_gic() -> arm_gic::gicv2::GicV2<'static> {
    use arm_gic::gicv2::registers::{Gicc, Gicd};

    /// Offset from PERIPHBASE for the GIC Distributor
    const GICD_BASE_OFFSET: usize = 0x0000_1000;

    /// Offset from PERIPHBASE for the GIC CPU interface
    const GICC_BASE_OFFSET: usize = 0x0000_0100;

    let gicd_base = (PERIPHBASE + GICD_BASE_OFFSET) as *mut Gicd;
    let gicc_base = (PERIPHBASE + GICC_BASE_OFFSET) as *mut Gicc;
    semihosting::println!(
        "Creating GIC driver @ {:010p} / {:010p}",
        gicd_base,
        gicc_base
    );
    // SAFETY: `gicd_base` and `gicc_base` point at the GIC Distributor and CPU
    // interface MMIO regions for this SoC, and this function is only called
    // once, so the driver has exclusive ownership.
    let mut gic = unsafe { arm_gic::gicv2::GicV2::new(gicd_base, gicc_base) };
    gic.setup();
    gic
}
