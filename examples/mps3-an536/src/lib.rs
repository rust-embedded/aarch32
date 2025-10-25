//! Common code for all examples
//!
//! ## Interrupt Map
//!
//! | Interrupt ID | Description                  |
//! |--------------|------------------------------|
//! | `EXTPPI0[0]` | UART 0 Receive Interrupt    |
//! | `EXTPPI0[1]` | UART 0 Transmit Interrupt   |
//! | `EXTPPI0[2]` | UART 0 Combined Interrupt   |
//! | `EXTPPI0[3]` | UART 0 Overflow             |
//! | `EXTPPI1[0]` | UART 1 Receive Interrupt    |
//! | `EXTPPI1[1]` | UART 1 Transmit Interrupt   |
//! | `EXTPPI1[2]` | UART 1 Combined Interrupt   |
//! | `EXTPPI1[3]` | UART 1 Overflow             |
//! | `SP[0]`      | WDG                         |
//! | `SP[1]`      | DualTimer 1                 |
//! | `SP[2]`      | DualTimer 2                 |
//! | `SP[3]`      | DualTimer Combined          |
//! | `SP[4]`      | RTC                         |
//! | `SP[5]`      | UART 2 Receive Interrupt    |
//! | `SP[6]`      | UART 2 Transmit Interrupt   |
//! | `SP[7]`      | UART 3 Receive Interrupt    |
//! | `SP[8]`      | UART 3 Transmit Interrupt   |
//! | `SP[9]`      | UART 4 Receive Interrupt    |
//! | `SP[10]`     | UART 4 Transmit Interrupt   |
//! | `SP[11]`     | UART 5 Receive Interrupt    |
//! | `SP[12]`     | UART 5 Transmit Interrupt   |
//! | `SP[13]`     | UART 2 Combined Interrupt   |
//! | `SP[14]`     | UART 3 Combined Interrupt   |
//! | `SP[15]`     | UART 4 Combined Interrupt   |
//! | `SP[16]`     | UART 5 Combined Interrupt   |
//! | `SP[17]`     | UART Overflow (2, 3, 4 & 5) |
//! | `SP[18]`     | Ethernet                    |
//! | `SP[19]`     | USB                         |
//! | `SP[20]`     | FPGA Audio I2S              |
//! | `SP[21]`     | Touch Screen                |
//! | `SP[22]`     | SPI ADC                     |
//! | `SP[23]`     | SPI Shield 0                |
//! | `SP[24]`     | SPI Shield 1                |
//! | `SP[25]`     | HDCLCD Interrupt            |
//! | `SP[26]`     | GPIO 0 Combined Interrupt   |
//! | `SP[27]`     | GPIO 1 Combined Interrupt   |
//! | `SP[28]`     | GPIO 2 Combined Interrupt   |
//! | `SP[29]`     | GPIO 3 Combined Interrupt   |
//! | `SP[30..=45]`| GPIO 0.x Interrupt          |
//! | `SP[46..=61]`| GPIO 1.x Interrupt          |
//! | `SP[62..=77]`| GPIO 2.x Interrupt          |
//! | `SP[78..=93]`| GPIO 3.x Interrupt          |
//!
//! * Interrupt ID `SP[x]` are shared across cores
//! * Interrupt ID `EXTPPI0[x]` is only available on Core 0
//! * Interrupt ID `EXTPPI1[x]` is only available on Core 1

#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

/// The PPI for the virutal timer, according to the Cortex-R52 Technical Reference Manual,
/// Table 10-3: PPI assignments.
///
/// This corresponds to Interrupt ID 27.
#[cfg(feature = "gic")]
pub const VIRTUAL_TIMER_PPI: arm_gic::IntId = arm_gic::IntId::ppi(11);

#[cfg(not(arm_architecture = "v8-r"))]
compile_error!("This example is only compatible to the ARMv8-R architecture");

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

#[cfg(feature = "gic")]
#[derive(Clone, Debug)]
/// Represents a handler for an interrupt
pub struct InterruptHandler {
    int_id: arm_gic::IntId,
    function: fn(arm_gic::IntId),
}

#[cfg(feature = "gic")]
impl InterruptHandler {
    /// Create a new `InterruptHandler`, associating an `IntId` with a function to call
    pub const fn new(int_id: arm_gic::IntId, function: fn(arm_gic::IntId)) -> InterruptHandler {
        InterruptHandler { int_id, function }
    }

    /// Get the [`arm_gic::IntId`] this handler is for
    pub const fn int_id(&self) -> arm_gic::IntId {
        self.int_id
    }

    /// Is this handler for this [`arm_gic::IntId`]?
    pub fn matches(&self, int_id: arm_gic::IntId) -> bool {
        self.int_id == int_id
    }

    /// Execute the handler
    pub fn execute(&self) {
        (self.function)(self.int_id);
    }
}

    /// Represents all the hardware we support in our MPS3-AN536 system
pub struct Board {
    /// The Arm Generic Interrupt Controller (v3)
    #[cfg(feature = "gic")]
    pub gic: arm_gic::gicv3::GicV3<'static>,
    /// The Arm Virtual Generic Timer
    pub virtual_timer: cortex_ar::generic_timer::El1VirtualTimer,
    /// The Arm Physical Generic Timer
    pub physical_timer: cortex_ar::generic_timer::El1PhysicalTimer,
}

impl Board {
    /// Create a new board structure.
    ///
    /// Returns `Some(board)` the first time you call it, and None thereafter,
    /// so you cannot have two copies of the [`Board`] structure.
    pub fn new() -> Option<Board> {
        static TAKEN: AtomicBool = AtomicBool::new(false);
        if TAKEN
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            Some(Board {
                #[cfg(feature = "gic")]
                // SAFETY: This is the first and only call to `make_gic()` as guaranteed by
                // the atomic flag check above, ensuring no aliasing of GIC register access.
                gic: unsafe { make_gic() },
                // SAFETY: This is the first and only time we create the virtual timer instance
                // as guaranteed by the atomic flag check above, ensuring exclusive access.
                virtual_timer: unsafe { cortex_ar::generic_timer::El1VirtualTimer::new() },
                // SAFETY: This is the first and only time we create the physical timer instance
                // as guaranteed by the atomic flag check above, ensuring exclusive access.
                physical_timer: unsafe { cortex_ar::generic_timer::El1PhysicalTimer::new() },
            })
        } else {
            None
        }
    }
}

/// Create the ARM GIC driver
///
/// # Safety
///
/// Only call this function once.
#[cfg(feature = "gic")]
unsafe fn make_gic() -> arm_gic::gicv3::GicV3<'static> {
    /// Offset from PERIPHBASE for GIC Distributor
    const GICD_BASE_OFFSET: usize = 0x0000_0000usize;

    /// Offset from PERIPHBASE for the first GIC Redistributor
    const GICR_BASE_OFFSET: usize = 0x0010_0000usize;

    // Get the GIC address by reading CBAR
    let periphbase = cortex_ar::register::ImpCbar::read().periphbase();
    semihosting::println!("Found PERIPHBASE {:010p}", periphbase);
    let gicd_base = periphbase.wrapping_byte_add(GICD_BASE_OFFSET);
    let gicr_base = periphbase.wrapping_byte_add(GICR_BASE_OFFSET);

    // Initialise the GIC.
    semihosting::println!(
        "Creating GIC driver @ {:010p} / {:010p}",
        gicd_base,
        gicr_base
    );
    // SAFETY: `gicd_base` points to the valid GICD MMIO region as obtained from the
    // hardware CBAR register. This pointer is used exclusively by this GIC instance.
    let gicd = unsafe {
        arm_gic::UniqueMmioPointer::new(core::ptr::NonNull::new(gicd_base.cast()).unwrap())
    };
    let gicr_base = core::ptr::NonNull::new(gicr_base.cast()).unwrap();
    // SAFETY: The GICD and GICR base addresses point to valid GICv3 MMIO regions as
    // obtained from the hardware CBAR register. This function is only called once
    // (via Board::new()'s atomic guard), ensuring exclusive ownership of the GIC.
    let mut gic = unsafe { arm_gic::gicv3::GicV3::new(gicd, gicr_base, 1, false) };
    semihosting::println!("Calling git.setup(0)");
    gic.setup(0);
    arm_gic::gicv3::GicCpuInterface::set_priority_mask(0x80);
    gic
}
