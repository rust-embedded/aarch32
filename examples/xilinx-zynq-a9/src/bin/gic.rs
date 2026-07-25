//! # GIC example for the Arm Cortex-A9 on a Xilinx Zynq-7000
//!
//! Sets up the memory-mapped GIC (the GICv2 programming model), configures a
//! Software Generated Interrupt, sends it to this same core, and handles it in
//! the IRQ handler. This exercises the interrupt controller on a single core,
//! before we add a second core.

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

use arm_gic::{
    IntId, InterruptGroup,
    gicv2::{GicV2, SgiTarget, SgiTargetListFilter},
};
use critical_section::Mutex;
use semihosting::println;

use aarch32_rt::entry;
use xilinx_zynq_a9 as _;

/// Set once the IRQ handler has run.
static IRQ_FIRED: AtomicBool = AtomicBool::new(false);

/// Shared interrupt controller driver.
///
/// The GICv2 CPU interface is memory-mapped, so ack/EOI need `&mut` access to
/// the driver from inside the IRQ handler. We therefore share it behind a
/// critical-section `Mutex`.
static GLOBAL_GIC: Mutex<RefCell<Option<GicV2<'static>>>> = Mutex::new(RefCell::new(None));

/// The Software Generated Interrupt we send to ourselves.
const SGI_INTID: IntId = IntId::sgi(3);

/// The interrupt group we use.
///
/// [`GicV2::setup`] only enables Group 0 signalling on the CPU interface (it
/// writes `GICC_CTLR = 0b1`), even when the GIC implements the Security
/// Extensions and assigns every interrupt to Group 1 at the Distributor. So we
/// keep our SGI in Group 0, where it is actually delivered. With `FIQEn` clear,
/// Group 0 interrupts are signalled as IRQs.
const GROUP: InterruptGroup = InterruptGroup::Group0;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch32-rt`.
#[entry]
fn main() -> ! {
    xilinx_zynq_a9::init();

    println!("I am core {:08x?}", aarch32_cpu::register::Mpidr::read());

    // SAFETY: this is the only call to `make_gic()`.
    let mut gic = unsafe { xilinx_zynq_a9::make_gic() };

    // `setup()` only enabled Group 1 in the Distributor (this GIC reports the
    // Security Extensions), so enable Group 0 too - that is the group our SGI
    // and the CPU interface use.
    gic.enable_group0(true);

    println!("Configure SGI {:?} as {:?}...", SGI_INTID, GROUP);
    gic.set_interrupt_priority(SGI_INTID, 0x31);
    gic.set_group(SGI_INTID, GROUP);
    gic.enable_interrupt(SGI_INTID, true).unwrap();

    critical_section::with(|cs| {
        GLOBAL_GIC.borrow_ref_mut(cs).replace(gic);
    });

    unsafe {
        aarch32_cpu::interrupt::enable();
    }

    println!("Send SGI to self");
    critical_section::with(|cs| {
        let mut gic = GLOBAL_GIC.borrow_ref_mut(cs);
        let gic = gic.as_mut().unwrap();
        gic.send_sgi(
            SGI_INTID,
            SgiTarget::List {
                target_list_filter: SgiTargetListFilter::ForwardSelfOnly,
                target_list: 0,
            },
        );
    });

    // Wait for the IRQ handler to run.
    for _ in 0..1_000_000 {
        if IRQ_FIRED.load(Ordering::SeqCst) {
            break;
        }
        aarch32_cpu::asm::wfi();
    }

    if IRQ_FIRED.load(Ordering::SeqCst) {
        println!("SGI handled");
    } else {
        println!("SGI missing?!");
    }

    xilinx_zynq_a9::exit(0);
}

/// Called when the Arm CPU gets an IRQ.
///
/// Talks to the GIC to find out which interrupt is pending, handles it, and
/// then tells the GIC it has been handled.
#[aarch32_rt::irq]
fn irq_handler() {
    critical_section::with(|cs| {
        let mut gic = GLOBAL_GIC.borrow_ref_mut(cs);
        let gic = gic.as_mut().unwrap();
        while let Some(intid) = gic.get_and_acknowledge_interrupt(GROUP) {
            println!("- got interrupt {:?}", intid);
            IRQ_FIRED.store(true, Ordering::SeqCst);
            gic.end_interrupt(intid, GROUP);
        }
    });
}
