//! # GIC example for Arm Cortex-R52 on an MPS2-AN536
//!
//! Uses a run-time map of interrupt handlers.

#![no_std]
#![no_main]

use core::cell::RefCell;

use aarch32_rt::{entry, irq};
use arm_gic::{
    IntId,
    gicv3::{GicCpuInterface, Group, InterruptGroup, SgiTarget, SgiTargetGroup},
};
use heapless::linear_map::LinearMap;
use mps3_an536::InterruptHandler;
use semihosting::println;

const SGI_INTID_LO: IntId = IntId::sgi(3);
const SGI_INTID_HI: IntId = IntId::sgi(4);

static INTERRUPT_HANDLERS: critical_section::Mutex<RefCell<LinearMap<IntId, InterruptHandler, 8>>> =
    critical_section::Mutex::new(RefCell::new(LinearMap::new()));

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch32-rt`.
#[entry]
fn main() -> ! {
    let mut board = mps3_an536::Board::new().unwrap();

    // Configure two Software Generated Interrupts for Core 0
    println!("Configure low-prio SGI...");
    board
        .gic
        .set_interrupt_priority(SGI_INTID_LO, Some(0), 0x31)
        .unwrap();
    board
        .gic
        .set_group(SGI_INTID_LO, Some(0), Group::Group1NS)
        .unwrap();

    println!("Configure high-prio SGI...");
    board
        .gic
        .set_interrupt_priority(SGI_INTID_HI, Some(0), 0x10)
        .unwrap();
    board
        .gic
        .set_group(SGI_INTID_HI, Some(0), Group::Group1NS)
        .unwrap();

    println!("gic.enable_interrupt()");
    board
        .gic
        .enable_interrupt(SGI_INTID_LO, Some(0), true)
        .unwrap();
    board
        .gic
        .enable_interrupt(SGI_INTID_HI, Some(0), true)
        .unwrap();

    critical_section::with(|cs| {
        let mut handlers = INTERRUPT_HANDLERS.borrow_ref_mut(cs);
        handlers
            .insert(
                SGI_INTID_LO,
                InterruptHandler::new(SGI_INTID_LO, handle_sgi_lo),
            )
            .unwrap();
        handlers
            .insert(
                SGI_INTID_HI,
                InterruptHandler::new(SGI_INTID_HI, handle_sgi_hi),
            )
            .unwrap();
    });

    println!("Enabling interrupts...");
    dump_sctlr();
    unsafe {
        aarch32_cpu::interrupt::enable();
    }
    dump_sctlr();

    // Send it
    println!("Send lo-prio SGI");
    GicCpuInterface::send_sgi(
        SGI_INTID_LO,
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
        SgiTargetGroup::CurrentGroup1,
    )
    .unwrap();

    for _ in 0..1_000_000 {
        aarch32_cpu::asm::nop();
    }

    println!("IRQ test completed OK");

    mps3_an536::exit(0);
}

fn dump_sctlr() {
    let sctlr = aarch32_cpu::register::Sctlr::read();
    println!("{:?}", sctlr);
}

/// Handles the low-prio SGI
fn handle_sgi_lo(int_id: IntId) {
    println!("- got {:?}, sending hi-prio {:?}", int_id, SGI_INTID_HI);
    GicCpuInterface::send_sgi(
        SGI_INTID_HI,
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
        SgiTargetGroup::CurrentGroup1,
    )
    .unwrap();
    println!("- finished sending hi-prio!");
}

/// Handles the high-prio SGI
fn handle_sgi_hi(int_id: IntId) {
    println!("- got hi-prio {:?}!", int_id);
}

/// Called when the Arm CPU gets an IRQ
///
/// Talks to the GICv3 to find out which interrupts are pending and calls
/// [`handle_interrupt_with_id`] for each of them, with interrupts re-enabled.
#[irq]
fn irq_handler() {
    println!("> IRQ");
    while let Some(next_int_id) =
        GicCpuInterface::get_and_acknowledge_interrupt(InterruptGroup::Group1)
    {
        // handle the interrupt
        println!("- handle_interrupt_with_id({:?})", next_int_id);
        let handler = critical_section::with(|cs| {
            let handlers_map = INTERRUPT_HANDLERS.borrow_ref(cs);
            handlers_map.get(&next_int_id).cloned()
        });
        if let Some(irq_entry) = handler {
            // let's go re-entrant
            unsafe {
                aarch32_cpu::interrupt::enable();
            }
            irq_entry.execute();
            // turn interrupts off again
            aarch32_cpu::interrupt::disable();
        }
        GicCpuInterface::end_interrupt(next_int_id, InterruptGroup::Group1);
    }
    println!("< IRQ");
}
