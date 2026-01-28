//! Generic-timer example for Arm Cortex-R52, with interrupts firing.

#![no_std]
#![no_main]

use aarch32_cpu::generic_timer::{El1VirtualTimer, GenericTimer};
use aarch32_rt::{entry, irq};
use arm_gic::gicv3::{GicCpuInterface, Group, InterruptGroup};
use mps3_an536::VIRTUAL_TIMER_PPI;
use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch32-rt`.
#[entry]
fn main() -> ! {
    let mut board = mps3_an536::Board::new().unwrap();

    // Only interrupts with a higher priority (numerically lower) will be signalled.
    GicCpuInterface::set_priority_mask(0x80);

    println!("Configure Timer Interrupt...");
    board
        .gic
        .set_interrupt_priority(VIRTUAL_TIMER_PPI, Some(0), 0x31)
        .unwrap();
    board
        .gic
        .set_group(VIRTUAL_TIMER_PPI, Some(0), Group::Group1NS)
        .unwrap();
    board
        .gic
        .enable_interrupt(VIRTUAL_TIMER_PPI, Some(0), true)
        .unwrap();

    // Setup virtual timer
    board.virtual_timer.enable(true);
    board.virtual_timer.interrupt_mask(false);
    board.virtual_timer.counter_compare_set(
        board
            .virtual_timer
            .counter()
            .wrapping_add(board.virtual_timer.frequency_hz() as u64 / 5),
    );

    println!("Enabling interrupts...");
    dump_sctlr();
    unsafe {
        aarch32_cpu::interrupt::enable();
    }
    dump_sctlr();

    let mut count: u32 = 0;
    loop {
        aarch32_cpu::asm::wfi();
        println!("Main loop wake up {}", count);
        count = count.wrapping_add(1);

        if count == 10 {
            println!("Timer IRQ test completed OK");
            semihosting::process::exit(0);
        }
    }
}

fn dump_sctlr() {
    let sctlr = aarch32_cpu::register::Sctlr::read();
    println!("{:?}", sctlr);
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
    let mut virtual_timer = unsafe { El1VirtualTimer::new() };
    // trigger a timer in 0.2 seconds
    virtual_timer.counter_compare_set(
        virtual_timer
            .counter_compare()
            .wrapping_add(virtual_timer.frequency_hz() as u64 / 5),
    );

    println!("    - Timer fired, resetting");
}
