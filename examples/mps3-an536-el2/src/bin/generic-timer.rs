//! Hyp Timer Test Arm Cortex-R52 running in EL2 (Hyp Mode)

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

use aarch32_cpu::generic_timer::GenericTimer;
use aarch32_rt::{entry, exception, irq};
use arm_gic::gicv3::{GicCpuInterface, Group, InterruptGroup};
use semihosting::println;

use mps3_an536_el2::HYP_TIMER_PPI;

static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code at the bottom of this file.
#[entry]
fn main() -> ! {
    let mut board = mps3_an536_el2::Board::new().unwrap();

    println!("{:?}", aarch32_cpu::register::Hcr::read());

    println!("Configure Timer Interrupt...");
    board
        .gic
        .set_interrupt_priority(HYP_TIMER_PPI, Some(0), 0x31)
        .unwrap();
    board
        .gic
        .set_group(HYP_TIMER_PPI, Some(0), Group::Group1NS)
        .unwrap();
    board
        .gic
        .enable_interrupt(HYP_TIMER_PPI, Some(0), true)
        .unwrap();

    let mut hyp_timer = board.hyp_timer;

    println!("Timer Hz = {}", hyp_timer.frequency_hz());
    hyp_timer.interrupt_mask(false);
    hyp_timer.countdown_set(hyp_timer.frequency_hz() / 5);
    hyp_timer.enable(true);
    // used in interrupt handler
    drop(hyp_timer);

    println!("Enabling interrupts...");
    dump_sctlr();
    unsafe {
        aarch32_cpu::interrupt::enable();
    }
    dump_sctlr();

    loop {
        aarch32_cpu::asm::wfi();
        let tick_count = TICK_COUNT.load(Relaxed);
        // println!("Main loop wake up {}", tick_count);
        if tick_count >= 10 {
            break;
        }
    }

    println!("EL2 timer test completed OK");

    mps3_an536_el2::exit(0);
}

fn dump_sctlr() {
    let sctlr = aarch32_cpu::register::Sctlr::read();
    println!("{:?}", sctlr);
}

#[irq]
fn irq_handler() {
    println!("> IRQ");
    while let Some(int_id) = GicCpuInterface::get_and_acknowledge_interrupt(InterruptGroup::Group1)
    {
        match int_id {
            HYP_TIMER_PPI => {
                println!("Hyp timer tick!");
                handle_timer_irq();
            }
            _ => {
                println!("Interrupt {:?}?", int_id);
            }
        }
        GicCpuInterface::end_interrupt(int_id, InterruptGroup::Group1);
    }
    println!("< IRQ");
}

/// Run when the timer IRQ fires
fn handle_timer_irq() {
    // SAFETY: We drop en other time handle in main, this is the only active handle.
    let mut el2_timer = unsafe { aarch32_cpu::generic_timer::El2HypPhysicalTimer::new() };
    // trigger a timer in 0.2 seconds
    el2_timer.countdown_set(el2_timer.frequency_hz() / 5);
    // tell the main loop the timer went tick
    TICK_COUNT.fetch_add(1, Relaxed);
}

/// This is our HVC exception handler
#[exception(HypervisorCall)]
fn hvc_handler(hsr: u32, frame: &aarch32_rt::Frame) -> u32 {
    let hsr = aarch32_cpu::register::Hsr::new_with_raw_value(hsr);
    println!(
        "In hvc_handler, with {:08x?}, {:x?}, {:08x?}",
        hsr,
        hsr.get_iss(),
        frame
    );
    return frame.r0;
}
