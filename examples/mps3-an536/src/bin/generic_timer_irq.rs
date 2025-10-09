//! Generic-timer example for Arm Cortex-R52, with interrupts firing.

#![no_std]
#![no_main]

use arm_gic::{
    gicv3::{GicCpuInterface, Group, InterruptGroup},
    IntId,
};
use cortex_ar::generic_timer::{El1VirtualTimer, GenericTimer};
use cortex_r_rt::{entry, irq};
use mps3_an536 as _;
use semihosting::println;

/// The PPI for the virutal timer, according to the Cortex-R52 Technical Reference Manual,
/// Table 10-3: PPI assignments.
///
/// This corresponds to Interrupt ID 27.
const VIRTUAL_TIMER_PPI: IntId = IntId::ppi(11);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-r-rt`.
#[entry]
fn main() -> ! {
    // Initialise the GIC.
    let mut gic = mps3_an536::init_gic();

    println!("Configure Timer Interrupt...");
    gic.set_interrupt_priority(VIRTUAL_TIMER_PPI, Some(0), 0x31)
        .unwrap();
    gic.set_group(VIRTUAL_TIMER_PPI, Some(0), Group::Group1NS)
        .unwrap();
    gic.enable_interrupt(VIRTUAL_TIMER_PPI, Some(0), true)
        .unwrap();

    // Create virtual timer, run as up-counter.
    let mut vgt = unsafe { El1VirtualTimer::new() };
    vgt.enable(true);
    vgt.interrupt_mask(false);
    vgt.counter_compare_set(vgt.counter().wrapping_add(vgt.frequency_hz() as u64));

    drop(vgt); // Drop to free the timer handle.

    println!("Enabling interrupts...");
    dump_cpsr();
    unsafe {
        cortex_ar::interrupt::enable();
    }
    dump_cpsr();

    let mut count: u32 = 0;
    loop {
        cortex_ar::asm::wfi();
        println!("Main loop wake up {}", count);
        count = count.wrapping_add(1);

        if count == 10 {
            println!("Timer IRQ test completed OK");
            semihosting::process::exit(0);
        }
    }
}

fn dump_cpsr() {
    let cpsr = cortex_ar::register::Cpsr::read();
    println!("CPSR: {:?}", cpsr);
}

#[irq]
fn irq_handler() {
    println!("  > IRQ");
    while let Some(int_id) = GicCpuInterface::get_and_acknowledge_interrupt(InterruptGroup::Group1)
    {
        match int_id {
            VIRTUAL_TIMER_PPI => handle_timer_irq(),
            _ => unreachable!("We handle all enabled IRQs"),
        }
        GicCpuInterface::end_interrupt(int_id, InterruptGroup::Group1);
    }
    println!("  < IRQ");
}

/// Run when the timer IRQ fires
fn handle_timer_irq() {
    // SAFETY: We drop en other time handle in main, this is the only active handle.
    let mut vgt = unsafe { El1VirtualTimer::new() };
    // trigger a timer in 0.2 seconds
    vgt.counter_compare_set(
        vgt.counter_compare()
            .wrapping_add(vgt.frequency_hz() as u64 / 5),
    );

    println!("    - Timer fired, resetting");
}
