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
//!
//! - `fpu-d32`: Make the interrupt context store routines save the upper
//!   double-precision registers.
//!
//!   If your program is using all 32 double-precision registers (e.g. if you
//!   have set the `+d32` target feature) then you need to enable this option
//!   otherwise important FPU state may be lost when an exception occurs.
//
//! - `el2-mode`: Leave the processor in EL2/PL2 mode on boot-up, and expect to
//!   handle interrupts in HYP mode using ELR_hyp. Useful if you want to write a
//!   hypervisor or other low-level firmware.
//!
//! - `svc-stack-interrupt`: Use the SVC stack when an interrupt occurs, instead
//!   of using the SYS stack. Useful if you are writing an RTOS and your SYS
//!   stack is actually the USR stack for the running task.
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
//!   each stack). Must be > 0. Defaults to 1.
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
//! in SYS mode or SVC mode (not IRQ mode!) when an [Interrupt] occurs. Use the
//! `svc-stack-interrupt` feature to select SVC mode instead of the default SYS
//! mode. You might want to use `svc-stack-interrupt` if you are running an RTOS
//! and you don't want to push a bunch of state into the running thread's stack
//! when an interrupt occurs.
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
//!   handler will call `_irq_handler` in SYS mode or SVC mode (but not IRQ
//!   mode), saving state as required.
//!
//! * `_asm_fiq_handler` - a naked function to call when a Fast Interrupt
//!   Request (FIQ) occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_fiq_handler` but you can override it. The provided default
//!   just spins forever.
//!
//! ## SMP Support
//!
//! This library supports SMP operation on ARMv7-A, ARMv7-R and ARMv8-R.
//!
//! To enable SMP support, add `PROVIDE(_num_cores = N)` to your linker script
//! (e.g. your `memory.x` file). This will cause space for 'N' copies of each
//! stack to be reserved so that each core gets its own stack (see the section
//! on 'Stacks', above).
//!
//! You must also write a function called `_asm_secondary_core_park` (which must
//! be written in assembly, and not Rust, because it is executed before stacks
//! and global memory are initialised). On start-up the bottom eight bits of
//! MPIDR register are taken as the Core ID. On Core ID 0, normal start-up will
//! occur. For non-zero Core IDs (so-called *secondary cores*), the cores call
//! the `_asm_secondary_core_park` function, passing the core ID in `r0`. This
//! function (which defaults to an infinite loop) should put the running core to
//! sleep and cause it to wait for some sort of signal from Core 0. This allows
//! Core 0 to complete the initialisation of global memory (`.data`, `.bss`,
//! etc) before the secondary cores run (and those cores must not re-initialise
//! global memory). After the cores have left the park routine and completed
//! their local initialisation (i.e. set their stack pointers to their unique
//! stacks), they execute the function `kmain_secondary` (recall that Core 0
//! executes a function called `kmain`).
//!
//! In our example for the MPS3-AN536, we have the secondary core wait on a
//! hardware register in one of the peripherals, because it has a known value at
//! reset.
//!
//! ```rust,ignore
//! #[unsafe(naked)]
//! #[unsafe(no_mangle)]
//! #[unsafe(link_section = ".text.startup")]
//! #[instruction_set(arm::a32)]
//! pub unsafe extern "C" fn _asm_secondary_core_park() {
//!     core::arch::naked_asm!(
//!         r#"
//!         // Some hardware register
//!         ldr     r1, =0xE020_2000
//!     1:
//!         // Wait until Core 0 does a 'sev'
//!         wfe
//!         // Spin until register is non-zero.
//!         ldr     r2, [r1]  
//!         cmp     r2, 0
//!         beq     1b
//!         // return to start-up
//!         bx      lr
//!     "#,
//!     );
//! }
//! ```
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
//! * `_asm_default_core_park_handler` - spins secondary cores forever
//! * `_default_handler` - a C compatible function that spins forever.
//! * `_asm_init_segments` - initialises `.bss` and `.data` and zeroes the
//!   stacks
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
//! function. Because FIQ is often performance-sensitive, we don't supply an FIQ
//! trampoline; if you want to use FIQ, you have to write your own assembly
//! routine, allowing you to preserve only whatever state is important to you.
//!
//! ## Examples
//!
//! You can find example code using QEMU inside the [project
//! repository](https://github.com/rust-embedded/aarch32/tree/main/examples)

#![no_std]

// *****************************************************************************
// Public Modules
// *****************************************************************************

pub mod sections;
pub mod stacks;

// *****************************************************************************
// Public Imports
// *****************************************************************************

pub use aarch32_rt_macros::{entry, exception, irq};

// *****************************************************************************
// Private Modules
// *****************************************************************************

#[cfg(all(arm_architecture = "v8-r", feature = "el2-mode"))]
mod arch_v8_hyp;

#[cfg(all(
    armv7_or_higher,
    not(all(arm_architecture = "v8-r", feature = "el2-mode"))
))]
mod arch_v7;

#[cfg(armv6_or_lower)]
mod arch_v4;

// *****************************************************************************
// Private Imports
// *****************************************************************************

#[cfg(target_arch = "arm")]
use aarch32_cpu::register::{cpsr::ProcessorMode, Cpsr};

// *****************************************************************************
// Types
// *****************************************************************************

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

// *****************************************************************************
// Macros
// *****************************************************************************

/// This macro expands to nothing.
///
/// It's a placeholder for the FPU saving/restoring routine, used on
/// targets without FPU support.
#[cfg(not(any(target_abi = "eabihf", feature = "eabi-fpu")))]
#[macro_export]
macro_rules! fpu_context {
    ("save") => {
        ""
    };
    ("restore") => {
        ""
    };
}

/// This macro expands to code for saving/restoring FPU context in an exception
/// handler. It pushes a multiple of eight bytes to preserve AAPCS alignment.
/// It may damage R0-R3.
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
macro_rules! fpu_context {
    // save all D16 FPU context, except D8-D15
    ("save") => {
        r#"
        vpush   {{ d0-d7 }}
        vmrs    r0, FPSCR
        vmrs    r1, FPEXC
        push    {{ r0-r1 }}
        "#
    };
    // restore all D16 FPU context, except D8-D15
    ("restore") => {
        r#"
        pop     {{ r0-r1 }}
        vmsr    FPEXC, r1
        vmsr    FPSCR, r0
        vpop    {{ d0-d7 }}
        "#
    };
}

/// This macro expands to code for saving/restoring FPU context in an exception
/// handler. It pushes a multiple of eight bytes to preserve AAPCS alignment. It
/// may damage R0-R3.
///
/// On entry to this block, we assume that we are in exception context.
///
/// This version saves FPU state assuming 32 DP registers (a 'D32' FPU
/// configuration).
///
/// EABI specifies D8-D15 as callee-save, and so we don't preserve them because
/// any C function we call to handle the exception will preserve/restore them
/// itself as required.
#[cfg(all(any(target_abi = "eabihf", feature = "eabi-fpu"), feature = "fpu-d32"))]
#[macro_export]
macro_rules! fpu_context {
    // save all D32 FPU context, except D8-D15
    ("save") => {
        r#"
        vpush   {{ d0-d7 }}
        vpush   {{ d16-d31 }}
        vmrs    r0, FPSCR
        vmrs    r1, FPEXC
        push    {{ r0-r1 }}
        "#
    };
    // restore all D32 FPU context, except D8-D15
    ("restore") => {
        r#"
        pop     {{ r0-r1 }}
        vmsr    FPEXC, r1
        vmsr    FPSCR, r0
        vpop    {{ d16-d31 }}
        vpop    {{ d0-d7 }}
        "#
    };
}

// *****************************************************************************
// Functions
// *****************************************************************************

// The Interrupt Vector Table, and some default assembly-language handler.
//
// Needs to be aligned to 5bits/2^5 to be stored correctly in VBAR
//
// Need to be assembled as Arm-mode because the Thumb Exception bit is cleared
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    .pushsection .vector_table,"ax",%progbits
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
    .popsection
    "#
);

// Shared library routines for all architectures
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    // The _asm_core_start function takes the core number in r0. It sets
    // up the stack pointers, the FPU (if required), and jumps to kmain.
    .pushsection .text._asm_core_start
    .arm
    .global _asm_core_start
    .type _asm_core_start, %function
    _asm_core_start:
        // Keep our core number for later
        mov     r12, r0
        // Set up stacks (core number in r0)
        bl      _stack_setup_preallocated
    "#,
    #[cfg(armv6_or_higher)]
    r#"
        // Clear Thumb Exception bit
        mrc     p15, 0, r0, c1, c0, 0
        bic     r0, #0x40000000
        mcr     p15, 0, r0, c1, c0, 0
    "#,
    #[cfg(any(target_abi = "eabihf", feature = "eabi-fpu"))]
    r#"
        // Allow VFP coprocessor access
        mrc     p15, 0, r0, c1, c0, 2
        orr     r0, r0, #0xF00000
        mcr     p15, 0, r0, c1, c0, 2
        // Enable VFP
        mov     r0, #0x40000000
        vmsr    fpexc, r0
    "#,
    r#"
        // Zero all registers before calling kmain (except r0)
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
        // Check if this is the primary core
        mov     r0, r12
        mov     r12, 0
        cmp     r0, 0
        bne     1f
        // Jump to application with primary core
        bl      kmain
        // In case the application returns, loop forever
        b       .
    1:
        // Jump to application with secondary core
        bl      kmain_secondary
        // In case the application returns, loop forever
        b       .
    .size _asm_core_start, . - _asm_core_start
    .popsection

    // Default main_secondary function just returns so we end up spinning
    .pushsection .text._default_kmain_secondary
    .global _default_kmain_secondary
    .arm
    .type _default_kmain_secondary, %function
    _default_kmain_secondary:
        bx      lr
    .size _default_kmain_secondary, . - _default_kmain_secondary
    .popsection

    // Configure a stack for every mode. Leaves you in sys mode.
    //
    // Pass the core number in r0
    .pushsection .text._stack_setup_preallocated
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
        // return to caller
        bx      r3
    .size _stack_setup_preallocated, . - _stack_setup_preallocated
    .popsection

    // Initialises stacks, .data and .bss
    .pushsection .text._asm_init_segments
    .arm
    .global _asm_init_segments
    .type _asm_init_segments, %function
    _asm_init_segments:
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
    .size _asm_init_segments, . - _asm_init_segments
    .popsection
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
);

// Default asm FIQ exception handler (it's just a spin-loop)
//
// We end up here if a FIQ fires and the weak 'PROVIDE' in the link.x
// file hasn't been over-ridden.
//
// Cannot be a Rust/C function because it can only touch registers R8 to R12, SP and LR
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    .pushsection .text._asm_default_fiq_handler

    // Our default FIQ handler
    .global _asm_default_fiq_handler
    .type _asm_default_fiq_handler, %function
    _asm_default_fiq_handler:
        b       _asm_default_fiq_handler
    .size    _asm_default_fiq_handler, . - _asm_default_fiq_handler
    .popsection
    "#,
);

/// Our default exception handler.
///
/// We end up here if an exception fires and the weak 'PROVIDE' in the link.x
/// file hasn't been over-ridden.
///
/// The assembly trampolines allow this to be a normal `extern "C"` function,
/// because they save and restore the necessary state.
#[unsafe(no_mangle)]
pub extern "C" fn _default_handler() {
    loop {
        core::hint::spin_loop();
    }
}

/// LLVM intrinsic for memory barriers
///
/// Only required on Armv4T and Armv5TE, because Armv6K onwards support atomics.
#[unsafe(no_mangle)]
#[cfg(armv5te_or_lower)]
pub extern "C" fn __sync_synchronize() {
    // we don't have a barrier instruction - the linux kernel just uses an empty inline asm block
    // so we do the same.
    unsafe {
        core::arch::asm!("");
    }
}

// *****************************************************************************
// End of file
// *****************************************************************************
