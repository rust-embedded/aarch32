//! # GIC example for Arm Cortex-R52 on an MPS2-AN536
//!
//! Uses a linker section to store InterruptHandler objects.

#![no_std]
#![no_main]

use aarch32_rt::{entry, irq};
use arm_gic::{
    IntId,
    gicv3::{GicCpuInterface, Group, InterruptGroup, SgiTarget, SgiTargetGroup},
};
use mps3_an536::InterruptHandler;
use semihosting::println;

const SGI_INTID_LO: IntId = IntId::sgi(3);
const SGI_INTID_HI: IntId = IntId::sgi(4);

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

#[unsafe(link_section = ".irq_entries")]
#[used]
pub static HANDLE_SGI_LO: InterruptHandler = InterruptHandler::new(SGI_INTID_LO, handle_sgi_lo);

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

#[unsafe(link_section = ".irq_entries")]
#[used]
pub static HANDLE_SGI_HI: InterruptHandler = InterruptHandler::new(SGI_INTID_HI, handle_sgi_hi);

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
        // let's go re-entrant
        unsafe {
            aarch32_cpu::interrupt::enable();
        }
        // handle the interrupt
        println!("- handle_interrupt_with_id({:?})", next_int_id);
        unsafe extern "Rust" {
            static __irq_entries_start: InterruptHandler;
            static __irq_entries_end: InterruptHandler;
        }
        let irq_entries_start: *const InterruptHandler = core::ptr::addr_of!(__irq_entries_start);
        let irq_entries_end: *const InterruptHandler = core::ptr::addr_of!(__irq_entries_end);
        let mut p = irq_entries_start;
        while p != irq_entries_end {
            let irq_entry = unsafe { p.read() };
            if irq_entry.matches(next_int_id) {
                irq_entry.execute();
                break;
            }
            p = unsafe { p.offset(1) };
        }
        // turn interrupts off again
        aarch32_cpu::interrupt::disable();
        GicCpuInterface::end_interrupt(next_int_id, InterruptGroup::Group1);
    }
    println!("< IRQ");
}
