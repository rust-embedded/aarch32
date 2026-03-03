//! # Run-time support for AArch32 Processors
//!
//! This library implements a simple Arm vector table, suitable for getting into
//! a Rust application running in System Mode. It also provides a reference
//! start up method. Most AArch32 based systems will require chip specific
//! start-up code, so the start-up method can be overridden.
//!
//! The default startup routine provided by this crate does not include any
//! special handling for multi-core support because this is oftentimes
//! implementation defined and the exact handling depends on the specific chip
//! in use. Many implementations only run the startup routine with one core and
//! will keep other cores in reset until they are woken up by an implementation
//! specific mechanism. For other implementations where multi-core specific
//! startup adaptions are necessary, the startup routine can be overwritten by
//! the user.
//!
//! ## Features
//!
//! - `eabi-fpu`: Enables the FPU, even if you selected a soft-float ABI target.
//! - `fpu-d32`: Make the interrupt context store routines save the upper
//!   double-precision registers.
//!
//!   If your program is using all 32 double-precision registers (e.g. if you
//!   have set the `+d32` target feature) then you need to enable this option
//!   otherwise important FPU state may be lost when an exception occurs.
//!
//! ## Information about the Run-Time
//!
//! Transferring from System Mode to User Mode (i.e. implementing an RTOS) is
//! not handled here.
//!
//! If your processor starts in Hyp mode, this runtime will be transfer it to
//! System mode. If you wish to write a hypervisor, you will need to replace
//! this library with something more advanced.
//!
//! We assume that a set of symbols exist, either for constants or for C
//! compatible functions or for naked raw-assembly functions. They are described
//! in the next three sections.
//!
//! ## Constants
//!
//! * `_num_cores` - the number of CPU core (and hence the number of copies of
//!   each stack). Must be > 0.
//! * `__sbss` - the start of zero-initialised data in RAM. Must be 4-byte
//!   aligned.
//! * `__ebss` - the end of zero-initialised data in RAM. Must be 4-byte
//!   aligned.
//! * `_fiq_stack_size` - the number of bytes to be reserved for stack space
//!   when in FIQ mode; will be padded to a multiple of 8.
//! * `_irq_stack_size` - the number of bytes to be reserved for stack space
//!   when in FIQ mode; will be padded to a multiple of 8.
//! * `_svc_stack_size` - the number of bytes to be reserved for stack space
//!   when in SVC mode; will be padded to a multiple of 8.
//! * `_und_stack_size` - the number of bytes to be reserved for stack space
//!   when in Undefined mode; will be padded to a multiple of 8.
//! * `_abt_stack_size` - the number of bytes to be reserved for stack space
//!   when in Abort mode; will be padded to a multiple of 8.
//! * `_hyp_stack_size` - the number of bytes to be reserved for stack space
//!   when in Hyp mode; will be padded to a multiple of 8.
//! * `_sys_stack_size` - the number of bytes to be reserved for stack space
//!   when in System mode; will be padded to a multiple of 8.
//! * `__sdata` - the start of initialised data in RAM. Must be 4-byte aligned.
//! * `__edata` - the end of initialised data in RAM. Must be 4-byte aligned.
//! * `__sidata` - the start of the initialisation values for data, in read-only
//!   memory. Must be 4-byte aligned.
//!
//! Using our default start-up function `_default_start`, the memory between
//! `__sbss` and `__ebss` is zeroed, and the memory between `__sdata` and
//! `__edata` is initialised with the data found at `__sidata`.
//!
//! ## Stacks
//!
//! Stacks are located in `.stacks` section which is mapped to the `STACKS`
//! memory region. Per default, the stacks are pushed to the end of the `STACKS`
//! by a filler section. We allocate stacks for each core, based on the
//! `_num_cores` linker symbol.
//!
//! The stacks look like:
//!
//! ```text
//! +------------------+ <---- ORIGIN(STACKS) + LENGTH(STACKS)
//! |     SYS Stack    | } _sys_stack_size * _num_cores bytes
//! +------------------+
//! |     FIQ Stack    | } _fiq_stack_size * _num_cores bytes
//! +------------------+
//! |     IRQ Stack    | } _irq_stack_size * _num_cores bytes
//! +------------------+
//! |     HYP Stack    | } _hyp_stack_size * _num_cores bytes (only used on Armv8-R)
//! +------------------+
//! |     ABT Stack    | } _abt_stack_size * _num_cores bytes
//! +------------------+
//! |     SVC Stack    | } _svc_stack_size * _num_cores bytes
//! +------------------+
//! |     UND Stack    | } _und_stack_size * _num_cores bytes
//! +------------------+
//! |  filler section  |
//! +------------------+ <---- ORIGIN(STACKS)
//! ```
//!
//! Our linker script PROVIDEs a symbol `_pack_stacks`. By setting this symbol
//! to 0 in memory.x, the stacks can be moved to the beginning of the `STACKS`
//! region or the end of the previous section located in STACKS or its alias.
//!
//! ## C-Compatible Functions
//!
//! ### Main Function
//!
//! The symbol `kmain` should be an `extern "C"` function. It is called in SYS
//! mode after all the global variables have been initialised. There is no
//! default - this function is mandatory.
//!
//! ```rust
//! #[unsafe(no_mangle)]
//! extern "C" fn kmain() -> ! {
//!     loop { }
//! }
//! ```
//!
//! You can also create a 'kmain' function by using the `#[entry]` attribute on
//! a normal Rust function. The function will be renamed in such a way that the
//! start-up assembly code can find it, but normal Rust code cannot. Therefore
//! you can be assured that the function will only be called once (unless
//! someone resorts to `unsafe` Rust to import the `kmain` symbol as an `extern
//! "C" fn`).
//!
//! ```rust
//! use aarch32_rt::entry;
//!
//! #[entry]
//! fn my_main() -> ! {
//!     loop { }
//! }
//! ```
//!
//! ### Undefined Handler
//!
//! The symbol `_undefined_handler` should be an `extern "C"` function. It is
//! called in UND mode when an [Undefined Instruction Exception] occurs.
//!
//! [Undefined Instruction Exception]:
//!     https://developer.arm.com/documentation/ddi0406/c/System-Level-Architecture/The-System-Level-Programmers--Model/Exception-descriptions/Undefined-Instruction-exception?lang=en
//!
//! Our linker script PROVIDEs a default `_undefined_handler` symbol which is an
//! alias for the `_default_handler` function. You can override it by defining
//! your own `_undefined_handler` function, like:
//!
//! ```rust
//! /// Does not return
//! #[unsafe(no_mangle)]
//! extern "C" fn _undefined_handler(addr: usize) -> ! {
//!     loop { }
//! }
//! ```
//!
//! or:
//!
//! ```rust
//! /// Execution will continue from the returned address.
//! ///
//! /// Return `addr` to go back and execute the faulting instruction again.
//! #[unsafe(no_mangle)]
//! unsafe extern "C" fn _undefined_handler(addr: usize) -> usize {
//!     // do stuff here, then return to the address *after* the one
//!     // that failed
//!     addr + 4
//! }
//! ```
//!
//! You can create a `_undefined_handler` function by using the
//! `#[exception(Undefined)]` attribute on a Rust function with the appropriate
//! arguments and return type.
//!
//! ```rust
//! use aarch32_rt::exception;
//!
//! #[exception(Undefined)]
//! fn my_handler(addr: usize) -> ! {
//!     loop { }
//! }
//! ```
//!
//! or:
//!
//! ```rust
//! use aarch32_rt::exception;
//!
//! #[exception(Undefined)]
//! unsafe fn my_handler(addr: usize) -> usize {
//!     // do stuff here, then return the address to return to
//!     addr + 4
//! }
//! ```
//!
//! ### Supervisor Call Handler
//!
//! The symbol `_svc_handler` should be an `extern "C"` function. It is called
//! in SVC mode when an [Supervisor Call Exception] occurs.
//!
//! [Supervisor Call Exception]:
//!     https://developer.arm.com/documentation/ddi0406/c/System-Level-Architecture/The-System-Level-Programmers--Model/Exception-descriptions/Supervisor-Call--SVC--exception?lang=en
//!
//! Returning from this function will cause execution to resume at the function
//! the triggered the exception, immediately after the SVC instruction. You
//! cannot control where execution resumes. The function is passed the literal
//! integer argument to the `svc` instruction, which is extracted from the
//! machine code for you by the default assembly trampoline, along with
//! registers r0 through r5, in the form of a reference to a `Frame` structure.
//!
//! Our linker script PROVIDEs a default `_svc_handler` symbol which is an alias
//! for the `_default_handler` function. You can override it by defining your
//! own `_svc_handler` function, like:
//!
//! ```rust
//! #[unsafe(no_mangle)]
//! extern "C" fn _svc_handler(arg: u32, frame: &aarch32_rt::Frame) -> u32 {
//!     // do stuff here
//!     todo!()
//! }
//! ```
//!
//! You can also create a `_svc_handler` function by using the
//! `#[exception(SupervisorCall)]` attribute on a normal Rust function.
//!
//! ```rust
//! use aarch32_rt::exception;
//!
//! #[exception(SupervisorCall)]
//! fn svc_handler(arg: u32, frame: &aarch32_rt::Frame) -> u32 {
//!     // do stuff here
//!     todo!()
//! }
//! ```
//!
//! ### Hypervisor Call Handler
//!
//! The symbol `_hvc_handler` should be an `extern "C"` function. It is called
//! in HYP mode when an [Hypervisor Call Exception] occurs.
//!
//! [Hypervisor Call Exception]:
//!     https://developer.arm.com/documentation/ddi0406/c/System-Level-Architecture/The-System-Level-Programmers--Model/Exception-descriptions/Hypervisor-Call--HVC--exception?lang=en
//!
//! Returning from this function will cause execution to resume at the function
//! the triggered the exception, immediately after the HVC instruction. You
//! cannot control where execution resumes. The function is passed contents of
//! the Hypervisor Syndrome Register (HSR) register, which is fetched by the
//! default assembly trampoline, along with registers r0 through r5, in the form
//! of a reference to a `Frame` structure.
//!
//! Our linker script PROVIDEs a default `_hvc_handler` symbol which is an alias
//! for the `_default_handler` function. You can override it by defining your
//! own `_hvc_handler` function, like:
//!
//! ```rust
//! #[unsafe(no_mangle)]
//! extern "C" fn _hvc_handler(hsr: u32, frame: &aarch32_rt::Frame) -> u32 {
//!     // do stuff here
//!     todo!()
//! }
//! ```
//!
//! You can also create a `_hvc_handler` function by using the
//! `#[exception(HypervisorCall)]` attribute on a normal Rust function.
//!
//! ```rust
//! use aarch32_rt::exception;
//!
//! #[exception(HypervisorCall)]
//! fn my_hvc_handler(hsr: u32, frame: &aarch32_rt::Frame) -> u32 {
//!     // do stuff here
//!     todo!()
//! }
//! ```
//!
//! If you wish to inspect the HSR value, you can use the `aarch32-cpu` crate:
//!
//! ```rust,ignore
//! let hsr = aarch32_cpu::register::Hsr::new_with_raw_value(hsr);
//! ```
//!
//! ### Prefetch Abort Handler
//!
//! The symbol `_prefetch_abort_handler` should be an `extern "C"` function. It
//! is called in ABT mode when a [Prefetch Abort Exception] occurs.
//!
//! [Prefetch Abort Exception]:
//!     https://developer.arm.com/documentation/ddi0406/c/System-Level-Architecture/The-System-Level-Programmers--Model/Exception-descriptions/Prefetch-Abort-exception?lang=en
//!
//! Our linker script PROVIDEs a default `_prefetch_abort_handler` symbol which
//! is an alias for the `_default_handler` function. You can override it by
//! defining your own `_undefined_handler` function.
//!
//! This function takes the address of faulting instruction, and can either not
//! return:
//!
//! ```rust
//! #[unsafe(no_mangle)]
//! extern "C" fn _prefetch_abort_handler(addr: usize) -> ! {
//!     loop { }
//! }
//! ```
//!
//! Or it can return an address where execution should resume after the
//! Exception handler is complete (which is unsafe):
//!
//! ```rust
//! #[unsafe(no_mangle)]
//! unsafe extern "C" fn _prefetch_abort_handler(addr: usize) -> usize {
//!     // do stuff, then go back to the instruction after the one that failed
//!     addr + 4
//! }
//! ```
//!
//! You can create a `_prefetch_abort_handler` function by using the
//! `#[exception(PrefetchAbort)]` macro on a Rust function with the appropriate
//! arguments and return type.
//!
//! ```rust
//! use aarch32_rt::exception;
//!
//! #[exception(PrefetchAbort)]
//! fn my_handler(addr: usize) -> ! {
//!     loop { }
//! }
//! ```
//!
//! or:
//!
//! ```rust
//! use aarch32_rt::exception;
//!
//! #[exception(PrefetchAbort)]
//! unsafe fn my_handler(addr: usize) -> usize {
//!     // do stuff, then go back to the instruction after the one that failed
//!     addr + 4
//! }
//! ```
//!
//! ### Data Abort Handler
//!
//! The symbol `_data_abort_handler` should be an `extern "C"` function. It is
//! called in ABT mode when a Data Abort Exception occurs.
//!
//! [Data Abort Exception]:
//!     https://developer.arm.com/documentation/ddi0406/c/System-Level-Architecture/The-System-Level-Programmers--Model/Exception-descriptions/Data-Abort-exception?lang=en
//!
//! Our linker script PROVIDEs a default `_data_abort_handler` symbol which is
//! an alias for the `_default_handler` function. You can override it by
//! defining your own `_undefined_handler` function.
//!
//! This function takes the address of faulting instruction, and can either not
//! return:
//!
//! ```rust
//! #[unsafe(no_mangle)]
//! extern "C" fn _data_abort_handler(addr: usize) -> ! {
//!     loop { }
//! }
//! ```
//!
//! Or it can return an address where execution should resume after the
//! Exception handler is complete (which is unsafe):
//!
//! ```rust
//! #[unsafe(no_mangle)]
//! unsafe extern "C" fn _data_abort_handler(addr: usize) -> usize {
//!     // do stuff, then go back to the instruction after the one that failed
//!     addr + 4
//! }
//! ```
//!
//! You can create a `_data_abort_handler` function by using the
//! `#[exception(DataAbort)]` macro on a Rust function with the appropriate
//! arguments and return type.
//!
//! ```rust
//! use aarch32_rt::exception;
//!
//! #[exception(DataAbort)]
//! fn my_handler(addr: usize) -> ! {
//!     loop { }
//! }
//! ```
//!
//! or:
//!
//! ```rust
//! use aarch32_rt::exception;
//!
//! #[exception(DataAbort)]
//! unsafe fn my_handler(addr: usize) -> usize {
//!     // do stuff, then go back to the instruction after the one that failed
//!     addr + 4
//! }
//! ```
//!
//! ### IRQ Handler
//!
//! The symbol `_irq_handler` should be an `extern "C"` function. It is called
//! in SYS mode (not IRQ mode!) when an [Interrupt] occurs.
//!
//! [Interrupt]:
//!     https://developer.arm.com/documentation/ddi0406/c/System-Level-Architecture/The-System-Level-Programmers--Model/Exception-descriptions/IRQ-exception?lang=en
//!
//! Returning from this function will cause execution to resume at wherever it
//! was interrupted. You cannot control where execution resumes.
//!
//! This function is entered with interrupts masked, but you may unmask (i.e.
//! enable) interrupts inside this function if desired. You will probably want
//! to talk to your interrupt controller first, otherwise you'll just keep
//! re-entering this interrupt handler recursively until you stack overflow.
//!
//! Our linker script PROVIDEs a default `_irq_handler` symbol which is an alias
//! for `_default_handler`. You can override it by defining your own
//! `_irq_handler` function.
//!
//! Expected prototype:
//!
//! ```rust
//! #[unsafe(no_mangle)]
//! extern "C" fn _irq_handler() {
//!     // 1. Talk to interrupt controller
//!     // 2. Handle interrupt
//!     // 3. Clear interrupt
//! }
//! ```
//!
//! You can also create a `_irq_handler` function by using the `#[irq]`
//! attribute on a normal Rust function.
//!
//! ```rust
//! use aarch32_rt::irq;
//!
//! #[irq]
//! fn my_irq_handler() {
//!     // 1. Talk to interrupt controller
//!     // 2. Handle interrupt
//!     // 3. Clear interrupt
//! }
//! ```
//!
//! ## ASM functions
//!
//! These are the naked 'raw' assembly functions the run-time requires:
//!
//! * `_start` - a Reset handler. Our linker script PROVIDEs a default function
//!   at `_default_start` but you can override it. The provided default start
//!   function will initialise all global variables and then call `kmain` in SYS
//!   mode. Some SoCs require a chip specific startup for tasks like MPU
//!   initialization or chip specific initialization routines, so if our
//!   start-up routine doesn't work for you, supply your own `_start` function
//!   (but feel free to call our `_default_start` as part of it).
//!
//! * `_asm_undefined_handler` - a naked function to call when an Undefined
//!   Exception occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_undefined_handler` but you can override it. The provided
//!   default handler will call `_undefined_handler` in UND mode, saving state
//!   as required.
//!
//! * `_asm_svc_handler` - a naked function to call when an Supervisor Call
//!   (SVC) Exception occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_svc_handler` but you can override it. The provided default
//!   handler will call `_svc_handler` in SVC mode, saving state as required.
//!
//! * `_asm_prefetch_abort_handler` - a naked function to call when a Prefetch
//!   Abort Exception occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_prefetch_abort_handler` but you can override it. The
//!   provided default handler will call `_prefetch_abort_handler`, saving state
//!   as required. Note that Prefetch Abort Exceptions are handled in Abort Mode
//!   (ABT), Monitor Mode (MON) or Hyp Mode (HYP), depending on CPU
//!   configuration.
//!
//! * `_asm_data_abort_handler` - a naked function to call when a Data Abort
//!   Exception occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_data_abort_handler` but you can override it. The provided
//!   default handler will call `_data_abort_handler` in ABT mode, saving state
//!   as required.
//!
//! * `_asm_irq_handler` - a naked function to call when an Undefined Exception
//!   occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_irq_handler` but you can override it. The provided default
//!   handler will call `_irq_handler` in SYS mode (not IRQ mode), saving state
//!   as required.
//!
//! * `_asm_fiq_handler` - a naked function to call when a Fast Interrupt
//!   Request (FIQ) occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_fiq_handler` but you can override it. The provided default
//!   just spins forever.
//!
//! ## Outputs
//!
//! This library produces global symbols called:
//!
//! * `_vector_table` - the start of the interrupt vector table
//! * `_default_start` - the default Reset handler, that sets up some stacks and
//!   calls an `extern "C"` function called `kmain`.
//! * `_asm_default_undefined_handler` - assembly language trampoline that calls
//!   `_undefined_handler`
//! * `_asm_default_svc_handler` - assembly language trampoline that calls
//!   `_svc_handler`
//! * `_asm_default_prefetch_abort_handler` - assembly language trampoline that
//!   calls `_prefetch_abort_handler`
//! * `_asm_default_data_abort_handler` - assembly language trampoline that
//!   calls `_data_abort_handler`
//! * `_asm_default_irq_handler` - assembly language trampoline that calls
//!   `_irq_handler`
//! * `_asm_default_fiq_handler` - an FIQ handler that just spins
//! * `_default_handler` - a C compatible function that spins forever.
//! * `_init_segments` - initialises `.bss` and `.data` and zeroes the stacks
//! * `_stack_setup_preallocated` - initialises UND, SVC, ABT, IRQ, FIQ and SYS
//!   stacks from the `.stacks` section defined in link.x, based on
//!   _xxx_stack_size values, and the core number given in `r0`
//! * `_xxx_stack_high_end` and `_xxx_stack_low_end` where the former is the top
//!   and the latter the bottom of the stack for each mode (`und`, `svc`, `abt`,
//!   `irq`, `fiq`, `sys`)
//!
//! The assembly language trampolines are required because AArch32 processors do
//! not save a great deal of state on entry to an exception handler, unlike
//! Armv7-M (and other M-Profile) processors. We must therefore save this state
//! to the stack using assembly language, before transferring to an `extern "C"`
//! function. We do not change modes before entering that `extern "C"` function
//! — that's for the handler to deal with as it wishes. Because FIQ is often
//! performance-sensitive, we don't supply an FIQ trampoline; if you want to use
//! FIQ, you have to write your own assembly routine, allowing you to preserve
//! only whatever state is important to you.
//!
//! ## Examples
//!
//! You can find example code using QEMU inside the [project
//! repository](https://github.com/rust-embedded/aarch32/tree/main/examples)

#![no_std]

#[cfg(target_arch = "arm")]
use aarch32_cpu::register::{cpsr::ProcessorMode, Cpsr};

#[cfg(all(arm_architecture = "v8-r", not(feature = "el2-mode")))]
use aarch32_cpu::register::Hactlr;

pub use aarch32_rt_macros::{entry, exception, irq};

#[cfg(all(target_arch = "arm", arm_architecture = "v8-r", feature = "el2-mode"))]
mod arch_v8_hyp;

#[cfg(all(
    target_arch = "arm",
    any(
        arm_architecture = "v7-a",
        arm_architecture = "v7-r",
        all(arm_architecture = "v8-r", not(feature = "el2-mode"))
    ),
))]
mod arch_v7;

#[cfg(all(
    target_arch = "arm",
    not(any(
        arm_architecture = "v7-a",
        arm_architecture = "v7-r",
        arm_architecture = "v8-r"
    ))
))]
mod arch_v4;

pub mod stacks;

/// Our default exception handler.
///
/// We end up here if an exception fires and the weak 'PROVIDE' in the link.x
/// file hasn't been over-ridden.
#[no_mangle]
pub extern "C" fn _default_handler() {
    loop {
        core::hint::spin_loop();
    }
}

// The Interrupt Vector Table, and some default assembly-language handler.
//
// Needs to be aligned to 5bits/2^5 to be stored correctly in VBAR
//
// Need to be assembled as Arm-mode because the Thumb Exception bit is cleared
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    .section .vector_table,"ax",%progbits
    .arm
    .global _vector_table
    .type _vector_table, %function
    .align 5
    _vector_table:
        ldr     pc, =_start
        ldr     pc, =_asm_undefined_handler
        ldr     pc, =_asm_svc_handler
        ldr     pc, =_asm_prefetch_abort_handler
        ldr     pc, =_asm_data_abort_handler
        ldr     pc, =_asm_hvc_handler
        ldr     pc, =_asm_irq_handler
        ldr     pc, =_asm_fiq_handler
    .size _vector_table, . - _vector_table
    "#
);

/// Arguments stacked on interrupt
///
/// This struct is very carefully designed to match the layout of the
/// registers pushed to the stack in our SVC handler.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Frame {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
}

/// This macro expands to code for saving FPU context on entry to an exception
/// handler. It pushes a multiple of eight bytes to preserve AAPCS alignment.
/// It may damage R0-R3.
///
/// It should match `restore_fpu_context!`
///
/// On entry to this block, we assume that we are in exception context.
#[cfg(not(any(target_abi = "eabihf", feature = "eabi-fpu")))]
#[macro_export]
macro_rules! save_fpu_context {
    () => {
        ""
    };
}

/// This macro expands to code for restoring context on exit from an exception
/// handler.
///
/// It should match `save_fpu_context!`.
#[cfg(not(any(target_abi = "eabihf", feature = "eabi-fpu")))]
#[macro_export]
macro_rules! restore_fpu_context {
    () => {
        ""
    };
}

/// This macro expands to code for saving FPU context on entry to an exception
/// handler. It pushes a multiple of eight bytes to preserve AAPCS alignment.
/// It may damage R0-R3.
///
/// It should match `restore_fpu_context!`
///
/// On entry to this block, we assume that we are in exception context.
///
/// This version saves FPU state, assuming 16 DP registers (a 'D16' or 'D16SP'
/// FPU configuration). Note that SP-only FPUs still have DP registers
/// - each DP register holds two SP values.
///
/// EABI specifies D8-D15 as callee-save, and so we don't
/// preserve them because any C function we call to handle the exception will
/// preserve/restore them itself as required.
#[cfg(all(
    any(target_abi = "eabihf", feature = "eabi-fpu"),
    not(feature = "fpu-d32")
))]
#[macro_export]
macro_rules! save_fpu_context {
    () => {
        r#"
        // save all D16 FPU context, except D8-D15
        vpush   {{ d0-d7 }}
        vmrs    r0, FPSCR
        vmrs    r1, FPEXC
        push    {{ r0-r1 }}
        "#
    };
}

/// This macro expands to code for restoring context on exit from an exception
/// handler. It restores FPU state, assuming 16 DP registers (a 'D16' or
/// 'D16SP' FPU configuration).
///
/// It should match `save_fpu_context!`.
#[cfg(all(
    any(target_abi = "eabihf", feature = "eabi-fpu"),
    not(feature = "fpu-d32")
))]
#[macro_export]
macro_rules! restore_fpu_context {
    () => {
        r#"
        // restore all D16 FPU context, except D8-D15
        pop     {{ r0-r1 }}
        vmsr    FPEXC, r1
        vmsr    FPSCR, r0
        vpop    {{ d0-d7 }}
        "#
    };
}

/// This macro expands to code for saving FPU context on entry to an exception
/// handler. It pushes a multiple of eight bytes to preserve AAPCS alignment.
/// It may damage R0-R3.
///
/// It should match `restore_fpu_context!`
///
/// On entry to this block, we assume that we are in exception context.
///
/// This version saves FPU state assuming 32 DP registers (a 'D32' FPU
/// configuration).
///
/// EABI specifies D8-D15 as callee-save, and so we don't
/// preserve them because any C function we call to handle the exception will
/// preserve/restore them itself as required.
#[cfg(all(any(target_abi = "eabihf", feature = "eabi-fpu"), feature = "fpu-d32"))]
#[macro_export]
macro_rules! save_fpu_context {
    () => {
        r#"
        // save all D32 FPU context, except D8-D15
        vpush   {{ d0-d7 }}
        vpush   {{ d16-d31 }}
        vmrs    r0, FPSCR
        vmrs    r1, FPEXC
        push    {{ r0-r1 }}
        "#
    };
}

/// This macro expands to code for restoring context on exit from an exception
/// handler. It restores FPU state, assuming 32 DP registers (a 'D32' FPU
/// configuration).
///
/// It should match `save_fpu_context!`.
#[cfg(all(any(target_abi = "eabihf", feature = "eabi-fpu"), feature = "fpu-d32"))]
#[macro_export]
macro_rules! restore_fpu_context {
    () => {
        r#"
        // restore all D32 FPU context, except D8-D15
        pop     {{ r0-r1 }}
        vmsr    FPEXC, r1
        vmsr    FPSCR, r0
        vpop    {{ d16-d31 }}
        vpop    {{ d0-d7 }}
        "#
    };
}

// Generic FIQ placeholder that's just a spin-loop
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    .section .text._asm_default_fiq_handler

    // Our default FIQ handler
    .global _asm_default_fiq_handler
    .type _asm_default_fiq_handler, %function
    _asm_default_fiq_handler:
        b       _asm_default_fiq_handler
    .size    _asm_default_fiq_handler, . - _asm_default_fiq_handler
    "#,
);

/// This macro expands to code to turn on the FPU
#[cfg(all(target_arch = "arm", any(target_abi = "eabihf", feature = "eabi-fpu")))]
macro_rules! fpu_enable {
    () => {
        r#"
        // Allow VFP coprocessor access
        mrc     p15, 0, r0, c1, c0, 2
        orr     r0, r0, #0xF00000
        mcr     p15, 0, r0, c1, c0, 2
        // Enable VFP
        mov     r0, #0x40000000
        vmsr    fpexc, r0
        "#
    };
}

/// This macro expands to code that does nothing because there is no FPU
#[cfg(all(
    target_arch = "arm",
    not(any(target_abi = "eabihf", feature = "eabi-fpu"))
))]
macro_rules! fpu_enable {
    () => {
        r#"
        // no FPU - do nothing
        "#
    };
}

// Start-up code for Armv7-R (and Armv8-R once we've left EL2)
// Stack location and sizes are taken from sections defined in linker script
// We set up our stacks and `kmain` in system mode.
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    // Configure a stack for every mode. Leaves you in sys mode.
    //
    // Pass the core number in r0
    .section .text._stack_setup_preallocated
    .global _stack_setup_preallocated
    .arm
    .type _stack_setup_preallocated, %function
    _stack_setup_preallocated:
        // Save LR from whatever mode we're currently in
        mov     r3, lr
        // (we might not be in the same mode when we return).
        // Set stack pointer and mask interrupts for UND mode (Mode 0x1B)
        msr     cpsr_c, {und_mode}
        ldr	    r2, =_und_stack_high_end
        ldr	    r1, =_und_stack_size
        muls    r1, r1, r0
        subs    sp, r2, r1
        // Set stack pointer (right after) and mask interrupts for SVC mode (Mode 0x13)
        msr     cpsr_c, {svc_mode}
        ldr	    r2, =_svc_stack_high_end
        ldr	    r1, =_svc_stack_size
        muls    r1, r1, r0
        subs    sp, r2, r1
        // Set stack pointer (right after) and mask interrupts for ABT mode (Mode 0x17)
        msr     cpsr_c, {abt_mode}
        ldr	    r2, =_abt_stack_high_end
        ldr	    r1, =_abt_stack_size
        muls    r1, r1, r0
        subs    sp, r2, r1
        // Set stack pointer (right after) and mask interrupts for IRQ mode (Mode 0x12)
        msr     cpsr_c, {irq_mode}
        ldr	    r2, =_irq_stack_high_end
        ldr	    r1, =_irq_stack_size
        muls    r1, r1, r0
        subs    sp, r2, r1
        // Set stack pointer (right after) and mask interrupts for FIQ mode (Mode 0x11)
        msr     cpsr_c, {fiq_mode}
        ldr	    r2, =_fiq_stack_high_end
        ldr	    r1, =_fiq_stack_size
        muls    r1, r1, r0
        subs    sp, r2, r1
        // Set stack pointer (right after) and mask interrupts for System mode (Mode 0x1F)
        msr     cpsr_c, {sys_mode}
        ldr	    r2, =_sys_stack_high_end
        ldr	    r1, =_sys_stack_size
        muls    r1, r1, r0
        subs    sp, r2, r1
        // Clear the Thumb Exception bit because all vector table is written in Arm assembly
        // even on Thumb targets.
        mrc     p15, 0, r1, c1, c0, 0
        bic     r1, #{te_bit}
        mcr     p15, 0, r1, c1, c0, 0
        // return to caller
        bx      r3
    .size _stack_setup_preallocated, . - _stack_setup_preallocated

    // Initialises stacks, .data and .bss
    .section .text._init_segments
    .arm
    .global _init_segments
    .type _init_segments, %function
    _init_segments:
        // Zero .bss
        ldr     r0, =__sbss
        ldr     r1, =__ebss
        mov     r2, 0
    0:
        cmp     r1, r0
        beq     1f
        stm     r0!, {{r2}}
        b       0b
    1:
        // Zero the stacks
        ldr     r0, =_stacks_low_end
        ldr     r1, =_stacks_high_end
        mov     r2, 0
    0:
        cmp     r1, r0
        beq     1f
        stm     r0!, {{r2}}
        b       0b
    1:
        // Initialise .data
        ldr     r0, =__sdata
        ldr     r1, =__edata
        ldr     r2, =__sidata
    0:
        cmp     r1, r0
        beq     1f
        ldm     r2!, {{r3}}
        stm     r0!, {{r3}}
        b       0b
    1:
    	// return to caller
        bx      lr
    .size _init_segments, . - _init_segments
    "#,
    und_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Und)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    svc_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Svc)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    abt_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Abt)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    fiq_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Fiq)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    irq_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Irq)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    sys_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Sys)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    te_bit = const {
        aarch32_cpu::register::Sctlr::new_with_raw_value(0)
            .with_te(true)
            .raw_value()
    }
);

// Start-up code for CPUs that boot into EL1
//
// Go straight to our default routine
#[cfg(all(target_arch = "arm", not(arm_architecture = "v8-r")))]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    .section .text.default_start
    .arm
    .global _default_start
    .type _default_start, %function
    _default_start:
        // Init .data and .bss
        bl      _init_segments
        // Set up stacks.
        mov     r0, #0
        bl      _stack_setup_preallocated
    "#,
    fpu_enable!(),
    r#"
        // Zero all registers before calling kmain
        mov     r0, 0
        mov     r1, 0
        mov     r2, 0
        mov     r3, 0
        mov     r4, 0
        mov     r5, 0
        mov     r6, 0
        mov     r7, 0
        mov     r8, 0
        mov     r9, 0
        mov     r10, 0
        mov     r11, 0
        mov     r12, 0
        // Jump to application
        bl      kmain
        // In case the application returns, loop forever
        b       .
    .size _default_start, . - _default_start
    "#
);

// Start-up code for Armv8-R to switch to EL1.
//
// There's only one Armv8-R CPU (the Cortex-R52) and the FPU is mandatory, so we
// always enable it.
//
// We boot into EL2, set up a stack pointer, and run `kmain` in EL1.
#[cfg(all(arm_architecture = "v8-r", not(feature = "el2-mode")))]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    .section .text.default_start
    .arm
    .global _default_start
    .type _default_start, %function
    _default_start:
        // Are we in EL2? If not, skip the EL2 setup portion
        mrs     r0, cpsr
        and     r0, r0, 0x1F
        cmp     r0, {cpsr_mode_hyp}
        bne     1f
        // Set stack pointer
        ldr     sp, =_hyp_stack_high_end
        // Set the HVBAR (for EL2) to _vector_table
        ldr     r1, =_vector_table
        mcr     p15, 4, r1, c12, c0, 0
        // Configure HACTLR to let us enter EL1
        mrc     p15, 4, r1, c1, c0, 1
        mov     r2, {hactlr_bits}
        orr     r1, r1, r2
        mcr     p15, 4, r1, c1, c0, 1
        // Program the SPSR - enter system mode (0x1F) in Arm mode with IRQ, FIQ masked
        mov		r1, {sys_mode}
        msr		spsr_hyp, r1
        adr		r1, 1f
        msr		elr_hyp, r1
        dsb
        isb
        eret
    1:
        // Set the VBAR (for EL1) to _vector_table. NB: This isn't required on
        // Armv7-R because that only supports 'low' (default) or 'high'.
        ldr     r0, =_vector_table
        mcr     p15, 0, r0, c12, c0, 0
        // Init .data and .bss
        bl      _init_segments
        // Set up stacks.
        mov     r0, #0
        bl      _stack_setup_preallocated
        "#,
        fpu_enable!(),
        r#"
        // Zero all registers before calling kmain
        mov     r0, 0
        mov     r1, 0
        mov     r2, 0
        mov     r3, 0
        mov     r4, 0
        mov     r5, 0
        mov     r6, 0
        mov     r7, 0
        mov     r8, 0
        mov     r9, 0
        mov     r10, 0
        mov     r11, 0
        mov     r12, 0
        // Jump to application
        bl      kmain
        // In case the application returns, loop forever
        b       .
    .size _default_start, . - _default_start
    "#,
    cpsr_mode_hyp = const ProcessorMode::Hyp as u8,
    hactlr_bits = const {
        Hactlr::new_with_raw_value(0)
            .with_cpuactlr(true)
            .with_cdbgdci(true)
            .with_flashifregionr(true)
            .with_periphpregionr(true)
            .with_qosr(true)
            .with_bustimeoutr(true)
            .with_intmonr(true)
            .with_err(true)
            .with_testr1(true)
            .raw_value()
    },
    sys_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Sys)
            .with_i(true)
            .with_f(true)
            .raw_value()
    }
);

// Start-up code for Armv8-R to stay in EL2.
//
// There's only one Armv8-R CPU (the Cortex-R52) and the FPU is mandatory, so we
// always enable it.
//
// We boot into EL2, set up a HYP stack pointer, and run `kmain` in EL2.
#[cfg(all(arm_architecture = "v8-r", feature = "el2-mode"))]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3

    .section .text.default_start
    .global _default_start
    .type _default_start, %function
    _default_start:
        // Init .data and .bss
        bl      _init_segments
        // Set stack pointer
        ldr     sp, =_hyp_stack_high_end
        // Set the HVBAR (for EL2) to _vector_table
        ldr     r1, =_vector_table
        mcr     p15, 4, r1, c12, c0, 0
        // Mask IRQ and FIQ
        mrs     r0, CPSR
        orr     r0, {irq_fiq}
        msr     CPSR, r0
    "#,
    fpu_enable!(),
    r#"
        // Zero all registers before calling kmain
        mov     r0, 0
        mov     r1, 0
        mov     r2, 0
        mov     r3, 0
        mov     r4, 0
        mov     r5, 0
        mov     r6, 0
        mov     r7, 0
        mov     r8, 0
        mov     r9, 0
        mov     r10, 0
        mov     r11, 0
        mov     r12, 0
        // Jump to application
        bl      kmain
        // In case the application returns, loop forever
        b       .
    .size _default_start, . - _default_start
    "#,
    irq_fiq = const aarch32_cpu::register::Cpsr::new_with_raw_value(0).with_i(true).with_f(true).raw_value()
);
