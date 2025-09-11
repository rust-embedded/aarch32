//! # GIC example for Arm Cortex-R52 on an MPS2-AN336
//!
//! Uses a linker section to store InterruptHandler objects.

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r_rt::{entry, irq};

// pull in our library
use mps3_an536::InterruptHandler;

use arm_gic::{
    gicv3::{GicV3, Group, InterruptGroup, SgiTarget, SgiTargetGroup},
    IntId,
};
use semihosting::println;

/// Offset from PERIPHBASE for GIC Distributor
const GICD_BASE_OFFSET: usize = 0x0000_0000usize;

/// Offset from PERIPHBASE for the first GIC Redistributor
const GICR_BASE_OFFSET: usize = 0x0010_0000usize;

const SGI_INTID_LO: IntId = IntId::sgi(3);
const SGI_INTID_HI: IntId = IntId::sgi(4);

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
    let mut gic: GicV3 = unsafe { GicV3::new(gicd_base.cast(), gicr_base.cast(), 1, false) };
    println!("Calling git.setup(0)");
    gic.setup(0);
    GicV3::set_priority_mask(0x80);

    // Configure a Software Generated Interrupt for Core 0
    println!("Configure low-prio SGI...");
    gic.set_interrupt_priority(SGI_INTID_LO, Some(0), 0x31);
    gic.set_group(SGI_INTID_LO, Some(0), Group::Group1NS);

    println!("Configure high-prio SGI...");
    gic.set_interrupt_priority(SGI_INTID_HI, Some(0), 0x10);
    gic.set_group(SGI_INTID_HI, Some(0), Group::Group1NS);

    println!("gic.enable_interrupt()");
    gic.enable_interrupt(SGI_INTID_LO, Some(0), true);
    gic.enable_interrupt(SGI_INTID_HI, Some(0), true);

    println!("Enabling interrupts...");
    dump_cpsr();
    unsafe {
        cortex_ar::interrupt::enable();
    }
    dump_cpsr();

    // Send it
    println!("Send lo-prio SGI");
    GicV3::send_sgi(
        SGI_INTID_LO,
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
        SgiTargetGroup::CurrentGroup1,
    );

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

#[unsafe(link_section = ".irq_entries")]
#[used]
pub static HANDLE_SGI_LO: InterruptHandler = InterruptHandler::new(SGI_INTID_LO, handle_sgi_lo);

/// Handles the low-prio SGI
fn handle_sgi_lo(int_id: IntId) {
    println!("- got {:?}, sending hi-prio {:?}", int_id, SGI_INTID_HI);
    GicV3::send_sgi(
        SGI_INTID_HI,
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
        SgiTargetGroup::CurrentGroup1,
    );
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
#[cfg(feature = "gic")]
#[irq]
fn irq_handler() {
    println!("> IRQ");
    while let Some(next_int_id) = GicV3::get_and_acknowledge_interrupt(InterruptGroup::Group1) {
        // let's go re-entrant
        unsafe {
            cortex_ar::interrupt::enable();
        }
        // handle the interrupt
        println!("- handle_interrupt_with_id({:?})", next_int_id);
        extern "Rust" {
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
        cortex_ar::interrupt::disable();
        GicV3::end_interrupt(next_int_id, InterruptGroup::Group1);
    }
    println!("< IRQ");
}
