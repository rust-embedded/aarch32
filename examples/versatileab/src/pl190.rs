//! The Versatile board's ARM PL190 PrimeCell Vectored Interrupt Controller

//! Represents our interrupt controller
#[derive(derive_mmio::Mmio)]
#[repr(C)]
pub struct Pl190 {
    /// IRQ Status Register
    #[mmio(PureRead)]
    vic_irqstatus: u32,
    /// FIQ Status Register
    #[mmio(PureRead)]
    vic_fiqstatus: u32,
    /// Raw Interrupt Status Register
    #[mmio(PureRead)]
    vic_rawintr: u32,
    /// Interrupt Select Register
    vic_intselect: u32,
    /// Interrupt Enable Register
    vic_intenable: u32,
    /// Interrupt Enable Clear Register
    #[mmio(Write)]
    vic_intenclear: u32,
    /// Software Interrupt Register
    vic_softint: u32,
    /// Software Interrupt Clear Register
    #[mmio(Write)]
    vic_softintclear: u32,
    /// Protection Enable Register
    vic_protection: u32,
    /// Vector Address Register
    vic_vectaddr: u32,
    /// Default Vector Address Register
    vic_defvectaddr: u32,
    _reserved1: [u32; 50],
    /// Vector Address Registers
    vic_vectaddrs: [u32; 16],
    _reserved2: [u32; 51],
    /// Vector Control Registers
    vic_vectcntl: [u32; 16],
}

impl Pl190 {
    /// Base address for the PL190 on the Versatile Application Board
    pub const VERSATILE_PL190_ADDR: usize = 0x10140000;

    /// Create a new PL190 Driver
    pub const fn create() -> MmioPl190<'static> {
        // Safety: This is where the PL190 lives
        unsafe { Pl190::new_mmio_at(Self::VERSATILE_PL190_ADDR) }
    }
}
