//! Code for managing CPSR (*Current Program Status Register*)

/// The current Processor Mode
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[bitbybit::bitenum(u5, exhaustive = false)]
pub enum ProcessorMode {
    /// User Mode
    Usr = 0b10000,
    /// FIQ Mode
    Fiq = 0b10001,
    /// IRQ Mode
    Irq = 0b10010,
    /// Supervisor Mode
    Svc = 0b10011,
    /// Monitor Mode
    Mon = 0b10110,
    /// Abort Mode
    Abt = 0b10111,
    /// Hyp Mode
    Hyp = 0b11010,
    /// Undefined Mode
    Und = 0b11011,
    /// System Mode
    Sys = 0b11111,
}

/// CPSR (*Current Program Status Register*)
#[bitbybit::bitfield(u32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cpsr {
    /// Negative Result from ALU
    #[bits(31..=31, r)]
    n: bool,
    /// Zero Result from ALU
    #[bits(30..=30, r)]
    z: bool,
    /// ALU operation Carry Out
    #[bits(29..=29, r)]
    c: bool,
    /// ALU operation Overflow
    #[bits(28..=28, r)]
    v: bool,
    /// Cumulative Saturation
    #[bits(27..=27, r)]
    q: bool,
    /// Jazelle State
    #[bits(24..=24, r)]
    j: bool,
    /// Endianness
    #[bits(9..=9, rw)]
    e: bool,
    /// Asynchronous Aborts
    #[bits(8..=8, rw)]
    a: bool,
    /// Interrupts Enabled
    #[bits(7..=7, rw)]
    i: bool,
    /// Fast Interrupts Enabled
    #[bits(6..=6, rw)]
    f: bool,
    /// Thumb state
    #[bits(5..=5, rw)]
    t: bool,
    /// Processor Mode
    #[bits(0..=4, rw)]
    mode: Option<ProcessorMode>,
}

impl Cpsr {
    /// Read CPSR (*Current Program Status Register*)
    ///
    /// On Armv4T and Armv5TE this will be an Arm function, even on the
    /// `thumb*` targets, as Thumb-1 cannot do an MRS.
    #[cfg_attr(not(feature = "check-asm"), inline)]
    #[cfg_attr(
        any(
            arm_architecture = "v4t",
            arm_architecture = "v5te",
            arm_architecture = "v6"
        ),
        instruction_set(arm::a32)
    )]
    pub fn read() -> Self {
        let r: u32;
        // Safety: Reading this register has no side-effects and is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrs {}, CPSR", out(reg) r, options(nomem, nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        Self::new_with_raw_value(r)
    }

    /// Write CPSR (*Current Program Status Register*)
    ///
    /// # Safety
    ///
    /// Changing the Program Status Register can affect whether interrupts are
    /// enabled, whether we are executing Arm or Thumb instructions, or which
    /// processor mode are in. You must be absolutely certain that the new CPSR
    /// value is valid and appropriate for continued Rust code execution.
    ///
    /// You almost certainly want to follow this with an [ISB](crate::asm::isb)
    /// instruction.
    ///
    /// On Armv4T and Armv5TE this will be an Arm function, even on the
    /// `thumb*` targets, as Thumb-1 cannot do an MSR.
    #[cfg_attr(not(feature = "check-asm"), inline)]
    #[cfg_attr(
        any(
            arm_architecture = "v4t",
            arm_architecture = "v5te",
            arm_architecture = "v6"
        ),
        instruction_set(arm::a32)
    )]
    pub unsafe fn write(_value: Self) {
        // Safety: This is risky, but we're in an unsafe function
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("msr CPSR, {}", in(reg) _value.raw_value());
        }
    }

    /// Modify SCTLR (*System Control Register*)
    ///
    /// # Safety
    ///
    /// See docs for [Self::write].
    #[inline]
    pub unsafe fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        unsafe {
            Self::write(value);
        }
    }
}

impl core::fmt::Debug for Cpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "CPSR {{ N={} Z={} C={} V={} Q={} J={} E={} A={} I={} F={} T={} MODE={:?} }}",
            self.n() as u8,
            self.z() as u8,
            self.c() as u8,
            self.v() as u8,
            self.q() as u8,
            self.j() as u8,
            self.e() as u8,
            self.a() as u8,
            self.i() as u8,
            self.f() as u8,
            self.t() as u8,
            self.mode(),
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Cpsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CPSR {{ N={0=31..32} Z={0=30..31} C={0=29..30} V={0=28..29} Q={0=27..28} J={0=24..25} E={0=9..10} A={0=8..9} I={0=7..8} F={0=6..7} T={0=5..6} MODE={0=0..5} }}", self.raw_value())
    }
}
