//! # Run-time support for Arm Cortex-A (AArch32)
//!
//! This library implements a simple Arm vector table, suitable for getting into
//! a Rust application running in System Mode. It also provides a reference
//! start up method. Most Cortex-A based systems will require chip specific
//! start-up code, so the start-up method can over overridden.
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
//! - `vfp-dp`: Enables support for the double-precision VFP floating point
//!   support. If your target CPU has this feature or support for NEON which
//!   also implies double-precision support, this feature should be activated.
//! - `eabi-fpu`: Enables the FPU, even if you selected a soft-float ABI target.
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
//! * `_stack_top` - the address of the top of some region of RAM that we can
//!   use as stack space, with eight-byte alignment. Our linker script PROVIDEs
//!   a default pointing at the top of RAM.
//! * `__sbss` - the start of zero-initialised data in RAM. Must be 4-byte
//!   aligned.
//! * `__ebss` - the end of zero-initialised data in RAM. Must be 4-byte
//!   aligned.
//! * `_fiq_stack_size` - the number of bytes to be reserved for stack space
//!   when in FIQ mode; must be a multiple of 8.
//! * `_irq_stack_size` - the number of bytes to be reserved for stack space
//!   when in FIQ mode; must be a multiple of 8.
//! * `_svc_stack_size` - the number of bytes to be reserved for stack space
//!   when in SVC mode; must be a multiple of 8.
//! * `__sdata` - the start of initialised data in RAM. Must be 4-byte aligned.
//! * `__edata` - the end of initialised data in RAM. Must be 4-byte aligned.
//! * `__sidata` - the start of the initialisation values for data, in read-only
//!   memory. Must be 4-byte aligned.
//!
//! Using our default start-up function `_default_start`, the memory between
//! `__sbss` and `__ebss` is zeroed, and the memory between `__sdata` and
//! `__edata` is initialised with the data found at `__sidata`.
//!
//! The stacks look like:
//!
//! ```text
//! +------------------+ <----_stack_top
//! |     UND Stack    | } _und_stack_size bytes
//! +------------------+
//! |     SVC Stack    | } _svc_stack_size bytes
//! +------------------+
//! |     ABT Stack    | } _abt_stack_size bytes
//! +------------------+
//! |     IRQ Stack    | } _irq_stack_size bytes
//! +------------------+
//! |     FIQ Stack    | } _fiq_stack_size bytes
//! +------------------+
//! |     SYS Stack    | } No specific size
//! +------------------+
//! ```
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
//! a normal Rust function.
//!
//! ```rust
//! use cortex_a_rt::entry;
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
//! use cortex_a_rt::exception;
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
//! use cortex_a_rt::exception;
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
//! in SVC mode when a [Supervisor Call Exception] occurs.
//!
//! [Supervisor Call Exception]:
//!     https://developer.arm.com/documentation/ddi0406/c/System-Level-Architecture/The-System-Level-Programmers--Model/Exception-descriptions/Supervisor-Call--SVC--exception?lang=en
//!
//! Returning from this function will cause execution to resume at the function
//! the triggered the exception, immediately after the SVC instruction. You
//! cannot control where execution resumes. The function is passed the literal
//! integer argument to the `svc` instruction, which is extracted from the
//! machine code for you by the default assembly trampoline.
//!
//! Our linker script PROVIDEs a default `_svc_handler` symbol which is an alias
//! for the `_default_handler` function. You can override it by defining your
//! own `_svc_handler` function, like:
//!
//! ```rust
//! #[unsafe(no_mangle)]
//! extern "C" fn _svc_handler(svc: u32) {
//!     // do stuff here
//! }
//! ```
//!
//! You can also create a `_svc_handler` function by using the
//! `#[exception(SupervisorCall)]` attribute on a normal Rust function.
//!
//! ```rust
//! use cortex_a_rt::exception;
//!
//! #[exception(SupervisorCall)]
//! fn my_svc_handler(arg: u32) {
//!     // do stuff here
//! }
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
//! use cortex_a_rt::exception;
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
//! use cortex_a_rt::exception;
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
//! called in ABT mode when a [Data Abort Exception] occurs.
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
//! use cortex_a_rt::exception;
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
//! use cortex_a_rt::exception;
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
//! use cortex_a_rt::irq;
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
//!   mode. Some SoCs require a chip specific startup for tasks like MMU
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
//!
//! The assembly language trampolines are required because Armv7-A processors do
//! not save a great deal of state on entry to an exception handler, unlike
//! Armv7-M (and other M-Profile) processors. We must therefore save this state
//! to the stack using assembly language, before transferring to an `extern "C"`
//! function. We do not change modes before entering that `extern "C"` function
//! \- that's for the handler to deal with as it wishes. Because FIQ is often
//! performance-sensitive, we don't supply an FIQ trampoline; if you want to use
//! FIQ, you have to write your own assembly routine, allowing you to preserve
//! only whatever state is important to you.
//!
//! ## Examples
//!
//! You can find example code using QEMU inside the [project
//! repository](https://github.com/rust-embedded/cortex-ar/tree/main/examples)

#![no_std]

#[cfg(target_arch = "arm")]
use cortex_ar::register::{cpsr::ProcessorMode, Cpsr};

pub use cortex_ar_rt_macros::{entry, exception, irq};

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
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    .section .vector_table,"ax",%progbits
    .global _vector_table
    .type _vector_table, %function
    _vector_table:
        ldr     pc, =_start
        ldr     pc, =_asm_undefined_handler
        ldr     pc, =_asm_svc_handler
        ldr     pc, =_asm_prefetch_abort_handler
        ldr     pc, =_asm_data_abort_handler
        nop
        ldr     pc, =_asm_irq_handler
        ldr     pc, =_asm_fiq_handler
    .size _vector_table, . - _vector_table
    "#
);

/// This macro expands to code for saving context on entry to an exception
/// handler.
///
/// It should match `restore_context!`.
///
/// On entry to this block, we assume that we are in exception context.
#[cfg(all(
    target_arch = "arm",
    not(any(target_abi = "eabihf", feature = "eabi-fpu"))
))]
macro_rules! save_context {
    () => {
        r#"
        // save preserved registers (and gives us some working area)
        push    {{r0-r3}}
        // align SP down to eight byte boundary
        mov     r0, sp
        and     r0, r0, 7
        sub     sp, r0
        // push alignment amount, and final preserved register
        push    {{r0, r12}}
        "#
    };
}

/// This macro expands to code for restoring context on exit from an exception
/// handler.
///
/// It should match `save_context!`.
#[cfg(all(
    target_arch = "arm",
    not(any(target_abi = "eabihf", feature = "eabi-fpu"))
))]
macro_rules! restore_context {
    () => {
        r#"
        // restore alignment amount, and preserved register
        pop     {{r0, r12}}
        // restore pre-alignment SP
        add     sp, r0
        // restore more preserved registers
        pop     {{r0-r3}}
        "#
    };
}

/// This macro expands to code for saving context on entry to an exception
/// handler.
///
/// It should match `restore_context!`.
#[cfg(all(
    target_arch = "arm",
    any(target_abi = "eabihf", feature = "eabi-fpu"),
    not(feature = "vfp-dp")
))]
macro_rules! save_context {
    () => {
        r#"
        // save preserved registers (and gives us some working area)
        push    {{r0-r3}}
        // save FPU context
        vpush   {{d0-d7}}
        vmrs    r0, FPSCR
        vmrs    r1, FPEXC
        push    {{r0-r1}}
        // align SP down to eight byte boundary
        mov     r0, sp
        and     r0, r0, 7
        sub     sp, r0
        // push alignment amount, and final preserved register
        push    {{r0, r12}}
        "#
    };
}

/// This macro expands to code for restoring context on exit from an exception
/// handler.
///
/// It should match `save_context!`.
#[cfg(all(
    target_arch = "arm",
    any(target_abi = "eabihf", feature = "eabi-fpu"),
    not(feature = "vfp-dp")
))]
macro_rules! restore_context {
    () => {
        r#"
        // restore alignment amount, and preserved register
        pop     {{r0, r12}}
        // restore pre-alignment SP
        add     sp, r0
        // pop FPU state
        pop     {{r0-r1}}
        vmsr    FPEXC, r1
        vmsr    FPSCR, r0
        vpop    {{d0-d7}}
        // restore more preserved registers
        pop     {{r0-r3}}
        "#
    };
}

/// This macro expands to code for saving context on entry to an exception
/// handler.
///
/// It should match `restore_context!`.
#[cfg(all(
    target_arch = "arm",
    any(target_abi = "eabihf", feature = "eabi-fpu"),
    feature = "vfp-dp"
))]
macro_rules! save_context {
    () => {
        r#"
        // save preserved registers (and gives us some working area)
        push    {{r0-r3}}
        // save FPU context
        vpush   {{d0-d7}}
        vpush   {{d16-d31}}
        vmrs    r0, FPSCR
        vmrs    r1, FPEXC
        push    {{r0-r1}}
        // align SP down to eight byte boundary
        mov     r0, sp
        and     r0, r0, 7
        sub     sp, r0
        // push alignment amount, and final preserved register
        push    {{r0, r12}}
        "#
    };
}

/// This macro expands to code for restoring context on exit from an exception
/// handler.
///
/// It should match `save_context!`.
#[cfg(all(
    target_arch = "arm",
    any(target_abi = "eabihf", feature = "eabi-fpu"),
    feature = "vfp-dp"
))]
macro_rules! restore_context {
    () => {
        r#"
        // restore alignment amount, and preserved register
        pop     {{r0, r12}}
        // restore pre-alignment SP
        add     sp, r0
        // pop FPU state
        pop     {{r0-r1}}
        vmsr    FPEXC, r1
        vmsr    FPSCR, r0
        vpop    {{d16-d31}}
        vpop    {{d0-d7}}
        // restore more preserved registers
        pop     {{r0-r3}}
        "#
    };
}

// Our assembly language exception handlers
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
       
    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _undefined_handler(addr: usize) -> usize;`
    // or
    // `extern "C" fn _undefined_handler(addr: usize) -> !;`
    .section .text._asm_default_undefined_handler
    .global _asm_default_undefined_handler
    .type _asm_default_undefined_handler, %function
    _asm_default_undefined_handler:
        // state save from compiled code
        srsfd   sp!, #{und_mode}
        // to work out what mode we're in, we need R0
        push    {{r0}}
        // First adjust LR for two purposes: Passing the faulting instruction to the C handler,
        // and to return to the failing instruction after the C handler returns.
        // Load processor status for the calling code
        mrs     r0, spsr
        // Was the code that triggered the exception in Thumb state?
        tst     r0, {t_bit}
        // Subtract 2 in Thumb Mode, 4 in Arm Mode - see p.1206 of the ARMv7-A architecture manual.
        ite     eq
        subeq   lr, lr, #4
        subne   lr, lr, #2
        // now do our standard exception save (which saves the 'wrong' R0)
    "#,
    save_context!(),
    r#"
        // Pass the faulting instruction address to the handler.
        mov     r0, lr
        // call C handler
        bl      _undefined_handler
        // if we get back here, assume they returned a new LR in r0
        mov     lr, r0
        // do our standard restore (with the 'wrong' R0)
    "#,
    restore_context!(),
    r#"
        // get the R0 we saved early
        pop     {{r0}}
        // overwrite the saved LR with the one from the C handler
        str     lr, [sp]
        // Return from the asm handler
        rfefd   sp!
    .size _asm_default_undefined_handler, . - _asm_default_undefined_handler


    .section .text._asm_default_svc_handler

    // Called from the vector table when we have an software interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _svc_handler(svc: u32);`
    .global _asm_default_svc_handler
    .type _asm_default_svc_handler, %function
    _asm_default_svc_handler:
        srsfd   sp!, #{svc_mode}
    "#,
    save_context!(),
    r#"
        mrs      r0, cpsr                 // Load processor status
        tst      r0, {t_bit}              // Occurred in Thumb state?
        ldrhne   r0, [lr,#-2]             // Yes: Load halfword and...
        bicne    r0, r0, #0xFF00          // ...extract comment field
        ldreq    r0, [lr,#-4]             // No: Load word and...
        biceq    r0, r0, #0xFF000000      // ...extract comment field
        // r0 now contains SVC number
        bl       _svc_handler
    "#,
    restore_context!(),
    r#"
        rfefd   sp!
    .size _asm_default_svc_handler, . - _asm_default_svc_handler


    .section .text._asm_default_data_abort_handler

    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _data_abort_handler(addr: usize);`
    .global _asm_default_data_abort_handler
    .type _asm_default_data_abort_handler, %function
    _asm_default_data_abort_handler:
        // Subtract 8 from the stored LR, see p.1214 of the ARMv7-A architecture manual.
        subs    lr, lr, #8
        // state save from compiled code
        srsfd   sp!, #{abt_mode}
    "#,
    save_context!(),
    r#"
        // Pass the faulting instruction address to the handler.
        mov     r0, lr
        // call C handler
        bl      _data_abort_handler
        // if we get back here, assume they returned a new LR in r0
        mov     lr, r0
    "#,
    restore_context!(),
    r#"
        // overwrite the saved LR with the one from the C handler
        str     lr, [sp]
        // Return from the asm handler
        rfefd   sp!
    .size _asm_default_data_abort_handler, . - _asm_default_data_abort_handler


    .section .text._asm_default_prefetch_abort_handler

    // Called from the vector table when we have a prefetch abort.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _prefetch_abort_handler(addr: usize);`
    .global _asm_default_prefetch_abort_handler
    .type _asm_default_prefetch_abort_handler, %function
    _asm_default_prefetch_abort_handler:
        // Subtract 4 from the stored LR, see p.1212 of the ARMv7-A architecture manual.
        subs    lr, lr, #4
        // state save from compiled code
        srsfd   sp!, #{abt_mode}
    "#,
    save_context!(),
    r#"
        // Pass the faulting instruction address to the handler.
        mov     r0, lr
        // call C handler
        bl      _prefetch_abort_handler
        // if we get back here, assume they returned a new LR in r0
        mov     lr, r0
    "#,
    restore_context!(),
    r#"
        // overwrite the saved LR with the one from the C handler
        str     lr, [sp]
        // Return from the asm handler
        rfefd   sp!
    .size _asm_default_prefetch_abort_handler, . - _asm_default_prefetch_abort_handler


    .section .text._asm_default_irq_handler

    // Called from the vector table when we have an interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _irq_handler();`
    .global _asm_default_irq_handler
    .type _asm_default_irq_handler, %function
    _asm_default_irq_handler:
        // make sure we jump back to the right place
        sub     lr, lr, 4
        // The hardware has copied CPSR to SPSR_irq and LR to LR_irq for us.
        // Now push SPSR_irq and LR_irq to the SYS stack.
        srsfd   sp!, #{sys_mode}
        // switch to system mode
        cps     #{sys_mode}
        // we also need to save LR, so we can be re-entrant
        push    {{lr}}
        // save state to the system stack (adjusting SP for alignment)
    "#,
        save_context!(),
    r#"
        // call C handler
        bl      _irq_handler
        // restore from the system stack
    "#,
        restore_context!(),
    r#"
        // restore LR
        pop     {{lr}}
        // pop CPSR and LR from the stack (which also restores the mode)
        rfefd   sp!
    .size _asm_default_irq_handler, . - _asm_default_irq_handler


    .section .text._asm_default_fiq_handler

    // Our default FIQ handler
    .global _asm_default_fiq_handler
    .type _asm_default_fiq_handler, %function
    _asm_default_fiq_handler:
        b       _asm_default_fiq_handler
    .size    _asm_default_fiq_handler, . - _asm_default_fiq_handler
    "#,
    svc_mode = const ProcessorMode::Svc as u8,
    und_mode = const ProcessorMode::Und as u8,
    abt_mode = const ProcessorMode::Abt as u8,
    sys_mode = const ProcessorMode::Sys as u8,
    t_bit = const {
        Cpsr::new_with_raw_value(0)
            .with_t(true)
            .raw_value()
    },
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

// Default start-up code for Armv7-A
//
// We set up our stacks and `kmain` in system mode.
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    .section .text.default_start
    .align 0

    .global _default_start
    .type _default_start, %function
    _default_start:
        // Set up stacks.
        ldr     r0, =_stack_top
        // Set stack pointer (right after) and mask interrupts for for UND mode (Mode 0x1B)
        msr     cpsr, {und_mode}
        mov     sp, r0
        ldr     r1, =_und_stack_size
        sub     r0, r0, r1
        // Set stack pointer (right after) and mask interrupts for for SVC mode (Mode 0x13)
        msr     cpsr, {svc_mode}
        mov     sp, r0
        ldr     r1, =_svc_stack_size
        sub     r0, r0, r1
        // Set stack pointer (right after) and mask interrupts for for ABT mode (Mode 0x17)
        msr     cpsr, {abt_mode}
        mov     sp, r0
        ldr     r1, =_abt_stack_size
        sub     r0, r0, r1
        // Set stack pointer (right after) and mask interrupts for for IRQ mode (Mode 0x12)
        msr     cpsr, {irq_mode}
        mov     sp, r0
        ldr     r1, =_irq_stack_size
        sub     r0, r0, r1
        // Set stack pointer (right after) and mask interrupts for for FIQ mode (Mode 0x11)
        msr     cpsr, {fiq_mode}
        mov     sp, r0
        ldr     r1, =_fiq_stack_size
        sub     r0, r0, r1
        // Set stack pointer (right after) and mask interrupts for for System mode (Mode 0x1F)
        msr     cpsr, {sys_mode}
        mov     sp, r0
        // Clear the Thumb Exception bit because we're in Arm mode
        mrc     p15, 0, r0, c1, c0, 0
        bic     r0, #{te_bit}
        mcr     p15, 0, r0, c1, c0, 0
    "#,
    fpu_enable!(),
    r#"
        // Initialise .bss
        ldr     r0, =__sbss
        ldr     r1, =__ebss
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
        cortex_ar::register::Sctlr::new_with_raw_value(0)
            .with_te(true)
            .raw_value()
    }
);
