//! GIC example for Arm Cortex-R52 on an MPS2-AN536
//!
//! As a single, unified, `#[irq]` handler.

#![no_std]
#![no_main]

use aarch32_rt::{entry, irq};
use arm_gic::{
    gicv3::{GicCpuInterface, Group, InterruptGroup, SgiTarget, SgiTargetGroup},
    IntId,
};
use mps3_an536 as _;
use semihosting::println;

const SGI_INTID_LO: IntId = IntId::sgi(3);
const SGI_INTID_HI: IntId = IntId::sgi(4);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch32-rt`.
#[entry]
fn main() -> ! {
    let mut board = mps3_an536::Board::new().unwrap();

    // Only interrupts with a higher priority (numerically lower) will be signalled.
    GicCpuInterface::set_priority_mask(0x80);

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

    semihosting::process::exit(0);
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
        // let's go re-entrant
        unsafe {
            aarch32_cpu::interrupt::enable();
        }
        println!("- IRQ Handling {:?}", int_id);
        if int_id == SGI_INTID_LO {
            println!(
                "- IRQ got {:?}, sending hi-prio {:?}",
                SGI_INTID_LO, SGI_INTID_HI
            );
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
            println!("- IRQ finished sending hi-prio!");
        }
        // turn interrupts off again
        aarch32_cpu::interrupt::disable();
        GicCpuInterface::end_interrupt(int_id, InterruptGroup::Group1);
    }
    println!("< IRQ");
}
