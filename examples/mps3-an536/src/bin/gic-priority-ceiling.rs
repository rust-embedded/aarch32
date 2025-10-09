//! GIC example to implement Priority Ceilings for Arm Cortex-R52 on an MPS2-AN336

#![no_std]
#![no_main]

use core::ptr::NonNull;

// pull in our start-up code
use cortex_r_rt::{entry, irq};

// pull in our library
use mps3_an536 as _;

use arm_gic::{
    gicv3::{GicCpuInterface, GicV3, Group, InterruptGroup, SgiTarget, SgiTargetGroup},
    IntId, UniqueMmioPointer,
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
    let gicd = unsafe { UniqueMmioPointer::new(NonNull::new(gicd_base.cast()).unwrap()) };
    let gicr = NonNull::new(gicr_base.cast()).unwrap();
    let mut gic = unsafe { GicV3::new(gicd, gicr, 1, false) };

    println!("Calling git.setup(0)");
    gic.setup(0);
    GicCpuInterface::set_priority_mask(0x80);

    // Configure a Software Generated Interrupt for Core 0
    println!("Configure low-prio SGI...");
    gic.set_interrupt_priority(SGI_INTID_LO, Some(0), 0x31)
        .unwrap();
    gic.set_group(SGI_INTID_LO, Some(0), Group::Group1NS)
        .unwrap();

    println!("Configure high-prio SGI...");
    gic.set_interrupt_priority(SGI_INTID_HI, Some(0), 0x10)
        .unwrap();
    gic.set_group(SGI_INTID_HI, Some(0), Group::Group1NS)
        .unwrap();

    println!("gic.enable_interrupt()");
    gic.enable_interrupt(SGI_INTID_LO, Some(0), true).unwrap();
    gic.enable_interrupt(SGI_INTID_HI, Some(0), true).unwrap();

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
    });
    println!("    - Post lock exit");
}

fn priority_ceiling_lock<F: FnMut()>(mut f: F) {
    let prio = GicCpuInterface::get_priority_mask();
    GicCpuInterface::set_priority_mask(4);

    f();

    GicCpuInterface::set_priority_mask(prio);
}
