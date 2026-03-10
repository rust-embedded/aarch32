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

use aarch32_cpu::register::{Hactlr, Cpsr, cpsr::ProcessorMode};

use core::sync::atomic::{AtomicBool, Ordering};

/// The PPI for the virutal timer, according to the Cortex-R52 Technical Reference Manual,
/// Table 10-3: PPI assignments.
///
/// This corresponds to Interrupt ID 27.
pub const VIRTUAL_TIMER_PPI: arm_gic::IntId = arm_gic::IntId::ppi(11);

#[cfg(not(arm_architecture = "v8-r"))]
compile_error!("This example is only compatible to the ARMv8-R architecture");

static WANT_PANIC: AtomicBool = AtomicBool::new(false);

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

/// Exit from QEMU with code
pub fn exit(code: i32) -> ! {
    stack_dump();
    semihosting::process::exit(code)
}

/// Print stack using to semihosting output for each stack
///
/// Produces output like:
///
/// ```text
/// Stack usage report:
/// UND1 Stack =      0 used of  16384 bytes (000%) @ 0x10057f00..0x1005bf00
/// UND0 Stack =      0 used of  16384 bytes (000%) @ 0x1005bf00..0x1005ff00
/// SVC1 Stack =      0 used of  16384 bytes (000%) @ 0x1005ff00..0x10063f00
/// SVC0 Stack =      0 used of  16384 bytes (000%) @ 0x10063f00..0x10067f00
/// ABT1 Stack =      0 used of  16384 bytes (000%) @ 0x10067f00..0x1006bf00
/// ABT0 Stack =      0 used of  16384 bytes (000%) @ 0x1006bf00..0x1006ff00
/// HYP1 Stack =      0 used of  16384 bytes (000%) @ 0x1006ff00..0x10073f00
/// HYP0 Stack =      0 used of  16384 bytes (000%) @ 0x10073f00..0x10077f00
/// IRQ1 Stack =      0 used of     64 bytes (000%) @ 0x10077f00..0x10077f40
/// IRQ0 Stack =      0 used of     64 bytes (000%) @ 0x10077f40..0x10077f80
/// FIQ1 Stack =      0 used of     64 bytes (000%) @ 0x10077f80..0x10077fc0
/// FIQ0 Stack =      0 used of     64 bytes (000%) @ 0x10077fc0..0x10078000
/// SYS1 Stack =    808 used of  16384 bytes (004%) @ 0x10078000..0x1007c000
/// SYS0 Stack =   1432 used of  16384 bytes (008%) @ 0x1007c000..0x10080000
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

/// Create the ARM GIC driver
///
/// # Safety
///
/// Only call this function once, from Core 0.
pub unsafe fn make_gic() -> arm_gic::gicv3::GicV3<'static> {
    /// Offset from PERIPHBASE for GIC Distributor
    const GICD_BASE_OFFSET: usize = 0x0000_0000usize;

    /// Offset from PERIPHBASE for the first GIC Redistributor
    const GICR_BASE_OFFSET: usize = 0x0010_0000usize;

    // Get the GIC address by reading CBAR
    let periphbase = aarch32_cpu::register::ImpCbar::read().periphbase();
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
    let mut gic = unsafe { arm_gic::gicv3::GicV3::new(gicd, gicr_base, 2, false) };
    semihosting::println!("Calling git.setup(0)");
    gic.setup(0);
    arm_gic::gicv3::GicCpuInterface::set_priority_mask(0xFF);
    gic
}

/// Release core1 from spin loop
pub fn start_core1() {
    let fpga_led = 0xE020_2000 as *mut u32;
    unsafe {
        // Activate second core by writing to FPGA LEDs.
        // We needed a shared register that wasn't in RAM, and this will do.
        fpga_led.write_volatile(1);
    }
}

// Start-up code for multi-core Armv8-R, as implemented on the MPS3-AN536.
//
// We boot into EL2, set up a stack pointer, init .data on .bss on core0, and
// run `kmain` in EL1 on all cores.
#[cfg(arm_architecture = "v8-r")]
core::arch::global_asm!(
    r#"
    .section .text.startup
    .align 4
    .arm

    .global _start
    .global core1_released
    .type _start, %function
    _start:
        // Read MPIDR into R0
        mrc     p15, 0, r0, c0, c0, 5
        ands    r0, r0, 0xFF
        bne     core1
    core0:
        ldr     pc, =_default_start
    core1:
        // LED GPIO register base address
        ldr     r0, =0xE0202000
        mov     r1, #0
    core1_spin:
        wfe
        // spin until an LED0 is on. We use the LED because unlike RAM this register resets to a known value.
        ldr     r2, [r0]  
        cmp     r1, r2
        beq     core1_spin
    core1_released:
        // now an LED is on, we assume _core1_stack_pointer contains our stack pointer
        // First we must exit EL2...
        // Set the HVBAR (for EL2) to _vector_table
        ldr     r0, =_vector_table
        mcr     p15, 4, r0, c12, c0, 0
        // Configure HACTLR to let us enter EL1
        mrc     p15, 4, r0, c1, c0, 1
        mov     r1, {hactlr_bits}
        orr     r0, r0, r1
        mcr     p15, 4, r0, c1, c0, 1
        // Program the SPSR - enter system mode (0x1F) in Arm mode with IRQ, FIQ masked
        mov		r0, {sys_mode}
        msr		spsr_hyp, r0
        adr		r0, 1f
        msr		elr_hyp, r0
        dsb
        isb
        eret
    1:
        // Allow VFP coprocessor access
        mrc     p15, 0, r0, c1, c0, 2
        orr     r0, r0, #0xF00000
        mcr     p15, 0, r0, c1, c0, 2
        // Enable VFP
        mov     r0, #0x40000000
        vmsr    fpexc, r0
        // Set the VBAR (for EL1) to _vector_table. NB: This isn't required on
        // Armv7-R because that only supports 'low' (default) or 'high'.
        ldr     r0, =_vector_table
        mcr     p15, 0, r0, c12, c0, 0
        // set up our stacks - also switches to SYS mode
        movs    r0, #1
        bl      _stack_setup_preallocated
        // Zero all registers before calling kmain2
        mov     r0, 0
        mov     r1, 0
        mov     r2, 0
        mov     r3, 0
        mov     r4, 0
        mov     r5, 0
        mov     r6, 0
        mov     r7, 0
        mov     r8, 0
        mov     r9, 0
        mov     r10, 0
        mov     r11, 0
        mov     r12, 0
        // call our kmain2 for core 1
        bl      kmain2
    .size _start, . - _start
    "#,
    hactlr_bits = const {
        Hactlr::new_with_raw_value(0)
            .with_cpuactlr(true)
            .with_cdbgdci(true)
            .with_flashifregionr(true)
            .with_periphpregionr(true)
            .with_qosr(true)
            .with_bustimeoutr(true)
            .with_intmonr(true)
            .with_err(true)
            .with_testr1(true)
            .raw_value()
    },
    sys_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Sys)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
);

/// What a second core does when no `kmain2` is supplied.
#[unsafe(no_mangle)]
pub extern "C" fn default_kmain2() {
    loop {
        aarch32_cpu::asm::wfe();
    }
}
