//! # Cross-core GIC example for Arm Cortex-R52 on an MPS2-AN536

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

use arm_gic::{
    IntId,
    gicv3::{GicCpuInterface, Group, InterruptGroup, SgiTarget, SgiTargetGroup},
};
use critical_section::Mutex;
use semihosting::println;

use aarch32_rt::entry;

static CORE1_BOOTED: AtomicBool = AtomicBool::new(false);

static PING_PONG_COMPLETE: AtomicBool = AtomicBool::new(false);

/// How long core 0 waits for core 1
const CORE0_WILL_WAIT: usize = 100_000_000;

/// Shared interrupt controller driver
static GLOBAL_GIC: Mutex<RefCell<Option<arm_gic::gicv3::GicV3<'static>>>> =
    Mutex::new(RefCell::new(None));

const SGI_INTID: IntId = IntId::sgi(3);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch32-rt`.
#[entry]
fn main() -> ! {
    println!(
        "I am core 0 - {:08x?}",
        aarch32_cpu::register::Mpidr::read()
    );

    let mut gic = unsafe { mps3_an536_smp::make_gic() };

    // Configure two Software Generated Interrupts for Core 0
    println!("Configure SGI on both cores...");
    gic.set_interrupt_priority(SGI_INTID, Some(0), 0x31)
        .unwrap();
    gic.set_group(SGI_INTID, Some(0), Group::Group1NS).unwrap();
    gic.enable_interrupt(SGI_INTID, Some(0), true).unwrap();

    gic.set_interrupt_priority(SGI_INTID, Some(1), 0x31)
        .unwrap();
    gic.set_group(SGI_INTID, Some(1), Group::Group1NS).unwrap();
    gic.enable_interrupt(SGI_INTID, Some(1), true).unwrap();

    unsafe {
        aarch32_cpu::interrupt::enable();
    }

    critical_section::with(|cs| {
        let mut global_gic = GLOBAL_GIC.borrow_ref_mut(cs);
        global_gic.replace(gic);
    });

    mps3_an536_smp::start_core1();

    // wait some time for core 1 to start
    for counter in 0..=CORE0_WILL_WAIT {
        if CORE1_BOOTED.load(Ordering::SeqCst) {
            break;
        }
        if counter == CORE0_WILL_WAIT {
            println!("CPU 1 is missing?!");

            mps3_an536_smp::exit(0);
        }
    }

    // Send it
    println!("Send SGI to other core");
    GicCpuInterface::send_sgi(
        SGI_INTID,
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b10,
        },
        SgiTargetGroup::CurrentGroup1,
    )
    .unwrap();

    // let the other core finish
    for _ in 0..CORE0_WILL_WAIT {
        aarch32_cpu::asm::wfi();
        if PING_PONG_COMPLETE.load(Ordering::Relaxed) {
            println!("Got pong");
            break;
        }
    }

    mps3_an536_smp::exit(0);
}

/// The entry-point to the Rust application.
///
/// It is called by the start-up code below, on Core 1.
#[unsafe(no_mangle)]
pub extern "C" fn kmain2() {
    println!(
        "I am core 1 - {:08x?}",
        aarch32_cpu::register::Mpidr::read()
    );

    unsafe {
        aarch32_cpu::interrupt::enable();
    }

    critical_section::with(|cs| {
        let mut global_gic = GLOBAL_GIC.borrow_ref_mut(cs);
        let global_gic = global_gic.as_mut().unwrap();
        semihosting::println!("Calling git.init_cpu(1)");
        global_gic.init_cpu(1);
    });
    GicCpuInterface::enable_group1(true);
    GicCpuInterface::set_priority_mask(0xFF);

    CORE1_BOOTED.store(true, Ordering::SeqCst);

    loop {
        aarch32_cpu::asm::wfi();
    }
}

/// Called when either Arm CPU gets an IRQ
///
/// Talks to the GICv3 to find out which interrupts are pending,
/// handles the interrupt, and then tells the GICv3 it has been handled.
#[aarch32_rt::irq]
fn irq_handler() {
    let id = aarch32_cpu::register::Mpidr::read();
    println!("> IRQ on {:08x?}", id);
    while let Some(next_int_id) =
        GicCpuInterface::get_and_acknowledge_interrupt(InterruptGroup::Group1)
    {
        // handle the interrupt
        println!("- handle_interrupt_with_id({:?})", next_int_id);

        if id.0 == 0x8000_0001 {
            println!("- send SGI back to first core");
            GicCpuInterface::send_sgi(
                SGI_INTID,
                SgiTarget::List {
                    affinity3: 0,
                    affinity2: 0,
                    affinity1: 0,
                    target_list: 0b01,
                },
                SgiTargetGroup::CurrentGroup1,
            )
            .unwrap();
        } else {
            PING_PONG_COMPLETE.store(true, Ordering::Relaxed);
        }

        GicCpuInterface::end_interrupt(next_int_id, InterruptGroup::Group1);
    }
    println!("< IRQ on {:08x?}", id);
}
