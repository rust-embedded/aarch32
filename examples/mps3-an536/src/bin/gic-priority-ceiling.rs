//! GIC example to implement Priority Ceilings for Arm Cortex-R52 on an MPS2-AN336

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r_rt::{entry, irq};

// pull in our library
use mps3_an536 as _;

use arm_gic::{
    gicv3::{GicCpuInterface, Group, InterruptGroup, SgiTarget, SgiTargetGroup},
    IntId,
};
use semihosting::println;

const SGI_INTID_LO: IntId = IntId::sgi(3);
const SGI_INTID_HI: IntId = IntId::sgi(4);

// Priority for `SGI_INTID_LO`
const LOW_PRIORITY: u8 = 0x31;
// Priority for `SGI_INTID_HI`
const HIGH_PRIORITY: u8 = 0x10;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-r-rt`.
#[entry]
fn main() -> ! {
    let mut board = mps3_an536::Board::new().unwrap();

    // Only interrupts with a higher priority (numerically lower) will be signalled.
    GicCpuInterface::set_priority_mask(0x80);

    // Configure a Software Generated Interrupt for Core 0
    println!("Configure low-prio SGI...");
    board
        .gic
        .set_interrupt_priority(SGI_INTID_LO, Some(0), LOW_PRIORITY)
        .unwrap();
    board
        .gic
        .set_group(SGI_INTID_LO, Some(0), Group::Group1NS)
        .unwrap();

    println!("Configure high-prio SGI...");
    board
        .gic
        .set_interrupt_priority(SGI_INTID_HI, Some(0), HIGH_PRIORITY)
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
    dump_cpsr();
    unsafe {
        cortex_ar::interrupt::enable();
    }
    dump_cpsr();

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
        cortex_ar::asm::nop();
    }

    println!("IRQ test completed OK");

    semihosting::process::exit(0);
}

fn dump_cpsr() {
    let cpsr = cortex_ar::register::Cpsr::read();
    println!("CPSR: {:?}", cpsr);
}

#[irq]
fn irq_handler() {
    println!("> IRQ");
    while let Some(int_id) = GicCpuInterface::get_and_acknowledge_interrupt(InterruptGroup::Group1)
    {
        // let's go re-entrant
        unsafe {
            cortex_ar::interrupt::enable();
        }
        println!("- IRQ Handling {:?}", int_id);
        match int_id {
            SGI_INTID_HI => high_prio(),
            SGI_INTID_LO => low_prio(),
            _ => unreachable!("We handle all enabled IRQs"),
        }
        // turn interrupts off again
        cortex_ar::interrupt::disable();
        GicCpuInterface::end_interrupt(int_id, InterruptGroup::Group1);
    }
    println!("< IRQ");
}

/// High prio IRQ
fn high_prio() {
    println!("    - High prio!");
}

/// Low prio IRQ
fn low_prio() {
    println!("    - Low prio!");

    priority_ceiling_lock(|| {
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
        println!("    - Pre lock exit");
        println!("    - HIGH PRIO SHOULD RUN AFTER THIS");
    });
    println!("    - HIGH PRIO SHOULD RUN BEFORE THIS");
    println!("    - Post lock exit");
}

fn priority_ceiling_lock<F: FnMut()>(mut f: F) {
    let prio = GicCpuInterface::get_priority_mask();
    // Block everything up to, and including, `HIGH_PRIORITY`
    GicCpuInterface::set_priority_mask(HIGH_PRIORITY);

    f();

    GicCpuInterface::set_priority_mask(prio);
}
