//! GICv3 + Generic Timer example

#![no_std]
#![no_main]

use core::ptr::NonNull;

// pull in our start-up code
use cortex_r_rt::{entry, irq};

// pull in our library
use mps3_an536 as _;

use arm_gic::{
    gicv3::{GicCpuInterface, GicV3, Group, InterruptGroup},
    IntId, UniqueMmioPointer,
};
use cortex_ar::generic_timer::{El1PhysicalTimer, El1VirtualTimer, GenericTimer};
use semihosting::println;

/// Offset from PERIPHBASE for GIC Distributor
const GICD_BASE_OFFSET: usize = 0x0000_0000usize;

/// Offset from PERIPHBASE for the first GIC Redistributor
const GICR_BASE_OFFSET: usize = 0x0010_0000usize;

/// The PPI for the physical timer, according to the Cortex-R52 Reference Manual
///
/// This corresponds to Interrupt ID 30.
const PHYSICAL_TIMER_PPI: IntId = IntId::ppi(14);

/// The PPI for the virutal timer, according to the Cortex-R52 Reference Manual
///
/// This corresponds to Interrupt ID 27.
const VIRTUAL_TIMER_PPI: IntId = IntId::ppi(11);

/// How many tick interrupts per second
const TICKS_PER_SECOND: u32 = 20;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-r-rt`.
#[entry]
fn main() -> ! {
    // Get the GIC address by reading CBAR
    let periphbase = cortex_ar::register::ImpCbar::read().periphbase();
    println!("Found PERIPHBASE {:010p}", periphbase);
    let gicd_base = periphbase.wrapping_byte_add(GICD_BASE_OFFSET);
    let gicr_base = periphbase.wrapping_byte_add(GICR_BASE_OFFSET);

    // Initialise the GIC.
    println!(
        "Creating GIC driver @ {:010p} / {:010p}",
        gicd_base, gicr_base
    );
    let gicd = unsafe { UniqueMmioPointer::new(NonNull::new(gicd_base.cast()).unwrap()) };
    let gicr_base = NonNull::new(gicr_base.cast()).unwrap();
    let mut gic: GicV3 = unsafe { GicV3::new(gicd, gicr_base, 1, false) };
    println!("Calling git.setup(0)");
    gic.setup(0);
    GicCpuInterface::set_priority_mask(0x80);

    println!("Configure Physical Timer Interrupt...");
    gic.set_interrupt_priority(PHYSICAL_TIMER_PPI, Some(0), 0x30)
        .expect("Timer set_interrupt_priority");
    gic.set_group(PHYSICAL_TIMER_PPI, Some(0), Group::Group1NS)
        .expect("Timer set_group");
    gic.enable_interrupt(PHYSICAL_TIMER_PPI, Some(0), true)
        .expect("Timer enable_interrupt");

    println!("Configure Virtual Timer Interrupt...");
    gic.set_interrupt_priority(VIRTUAL_TIMER_PPI, Some(0), 0x31)
        .expect("Timer set_interrupt_priority");
    gic.set_group(VIRTUAL_TIMER_PPI, Some(0), Group::Group1NS)
        .expect("Timer set_group");
    gic.enable_interrupt(VIRTUAL_TIMER_PPI, Some(0), true)
        .expect("Timer enable_interrupt");

    let mut pgt = unsafe { El1PhysicalTimer::new() };
    pgt.enable(true);
    pgt.interrupt_mask(false);
    pgt.counter_compare_set(u64::MAX);
    println!(
        "Physical timer frequency {} Hz, interrupt every {}",
        pgt.frequency_hz(),
        pgt.frequency_hz() / TICKS_PER_SECOND
    );
    pgt.countdown_set(pgt.frequency_hz() / TICKS_PER_SECOND);

    let mut vgt = unsafe { El1VirtualTimer::new() };
    vgt.enable(true);
    vgt.interrupt_mask(false);
    vgt.counter_compare_set(u64::MAX);
    println!(
        "Virtual timer frequency {} Hz, interrupt every {}",
        vgt.frequency_hz(),
        vgt.frequency_hz() / TICKS_PER_SECOND
    );
    vgt.countdown_set(vgt.frequency_hz() / TICKS_PER_SECOND);

    println!("Enabling interrupts...");
    dump_cpsr();
    unsafe {
        cortex_ar::interrupt::enable();
    }
    dump_cpsr();

    for count in 1..=10 {
        cortex_ar::asm::wfi();
        println!("Main loop wake up {}", count);
    }

    semihosting::process::exit(0);
}

fn dump_cpsr() {
    let cpsr = cortex_ar::register::Cpsr::read();
    println!("CPSR: {:?}", cpsr);
}

/// Called when the Arm core gets an IRQ
#[irq]
fn irq_handler() {
    println!("> IRQ");
    while let Some(int_id) = GicCpuInterface::get_and_acknowledge_interrupt(InterruptGroup::Group1)
    {
        println!("- IRQ handle {:?}", int_id);
        if int_id == PHYSICAL_TIMER_PPI {
            handle_physical_timer_irq();
        } else if int_id == VIRTUAL_TIMER_PPI {
            handle_virtual_timer_irq();
        }
        GicCpuInterface::end_interrupt(int_id, InterruptGroup::Group1);
    }
    println!("< IRQ");
}

/// Run when the virtual timer IRQ fires
fn handle_virtual_timer_irq() {
    // SAFETY: No other code writes to this peripheral, so this won't race
    let mut vgt = unsafe { El1VirtualTimer::new() };
    if vgt.interrupt_status() {
        println!("- Virtual Timer handled");
        let delta = vgt.frequency_hz() / TICKS_PER_SECOND;
        vgt.countdown_set(vgt.countdown().wrapping_add(delta));
    }
}

/// Run when the physical timer IRQ fires
fn handle_physical_timer_irq() {
    // SAFETY: No other code writes to this peripheral, so this won't race
    let mut pgt = unsafe { El1PhysicalTimer::new() };
    if pgt.interrupt_status() {
        println!("- Physical Timer handled");
        let delta = pgt.frequency_hz() / TICKS_PER_SECOND;
        pgt.countdown_set(pgt.countdown().wrapping_add(delta));
    }
}
