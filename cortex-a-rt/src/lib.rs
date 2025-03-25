//! # Run-time support for Arm Cortex-A
//!
//! This library implements a simple Arm vector table, suitable for getting into
//! a Rust application running in System Mode. It also provides a reference start up method.
//! Usually, most Cortex-A based systems will require chip specific start-up code, so the
//! start-up method can over overriden.
//!
//! ## Features
//!
//! - `vfp-dp`: Enables support for the double-precision VFP floating point support. If your target
//!    CPU has this feature or support for NEON which also implies double-precision support, this
//!    feature should be activated.
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
//! We assume the following global symbols exist:
//!
//! ### Constants
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
//! ### Functions
//!
//! * `boot_core` - the `extern "C"` entry point to your application. The CPU ID
//!   will be passed as the first argument to this function.
//!
//!   Expected prototype:
//!
//!   ```rust
//!   #[unsafe(no_mangle)]
//!   extern "C" fn boot_core(cpu_id: u32) -> !;
//!   ```
//!
//! * `_svc_handler` - an `extern "C"` function to call when an SVC Exception
//!   occurs. Our linker script PROVIDEs a default function at
//!   `_default_handler` but you can override it.
//!
//!   Expected prototype:
//!
//!   ```rust
//!   #[unsafe(no_mangle)]
//!   extern "C" fn _svc_handler(svc: u32);
//!   ```
//!
//! * `_irq_handler` - an `extern "C"` function to call when an Interrupt
//!   occurs. Our linker script PROVIDEs a default function at
//!   `_default_handler` but you can override it.
//!
//!   Expected prototype:
//!
//!   ```rust
//!   #[unsafe(no_mangle)]
//!   extern "C" fn _irq_handler();
//!   ```
//!
//! * `_undefined_handler` - an `extern "C"` function to call when an Undefined Exception
//!   occurs. Our linker script PROVIDEs a default function at
//!   `_default_handler` but you can override it. It will be called by the
//!   `_asm_default_undefined_handler` unless that function is overriden as well.
//!
//!   Expected prototype:
//!
//!   ```rust
//!   #[unsafe(no_mangle)]
//!   extern "C" fn _undefined_handler(faulting_instruction: u32);
//!   ```
//!
//! * `_abort_handler` - an `extern "C"` function to call when a Data Abort Exception
//!   occurs. Our linker script PROVIDEs a default function at
//!   `_default_handler` but you can override it. It will be called by the
//!   `_asm_default_abort_handler` unless that function is overriden as well.
//!
//!   Expected prototype:
//!
//!   ```rust
//!   #[unsafe(no_mangle)]
//!   extern "C" fn _abort_handler(faulting_instruction: u32);
//!   ```
//!
//! * `_prefetch_handler` - an `extern "C"` function to call when a Prefetch Abort Exception
//!   occurs. Our linker script PROVIDEs a default function at
//!   `_default_handler` but you can override it. It will be called by the
//!   `_asm_default_prefetch_handler` unless that function is overriden as well.
//!
//!   Expected prototype:
//!
//!   ```rust
//!   #[unsafe(no_mangle)]
//!   extern "C" fn _prefetch_handler(faulting_instruction: u32);
//!   ```
//!
//! ### ASM functions
//!
//! * `__start` - a Reset handler. Our linker script PROVIDEs a default function
//!   at `_default_start` but you can override it. Most Cortex-A SoCs require
//!   a chip specific startup for tasks like MMU initialization or chip specific
//!   initialization routines.
//! * `_asm_fiq_handler` - a naked function to call when a Fast Interrupt
//!   Request (FIQ) occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_fiq_handler` but you can override it.
//! * `_asm_undefined_handler` - a naked function to call when an Undefined
//!   Exception occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_undefined_handler` but you can override it.
//! * `_asm_prefetch_handler` - a naked function to call when an Prefetch
//!   Exception occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_prefetch_handler` but you can override it. The provided default
//!   handler will perform an exception return to the faulting address.
//! * `_asm_abort_handler` - a naked function to call when an Abort Exception
//!   occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_abort_handler` but you can override it. The provided default handler
//!   will perform an exception return to the faulting address.
//!
//! On start-up, the memory between `__sbss` and `__ebss` is zeroed, and the
//! memory between `__sdata` and `__edata` is initialised with the data found at
//! `__sidata`.
//!
//! This library produces global symbols called:
//!
//! * `_vector_table` - the start of the interrupt vector table
//! * `_default_start` - the default Reset handler, that sets up some stacks and
//!   calls an `extern "C"` function called `boot_core`.
//! * `_asm_default_fiq_handler` - an FIQ handler that just spins
//! * `_asm_default_handler` - an exception handler that just spins
//! * `_asm_svc_handler` - assembly language trampoline for SVC Exceptions that
//!   calls `_svc_handler`
//! * `_asm_irq_handler` - assembly language trampoline for Interrupts that
//!   calls `_irq_handler`
//!
//! The assembly language trampolines are required because Armv7-R (and Armv8-R)
//! processors do not save a great deal of state on entry to an exception
//! handler, unlike Armv7-M (and other M-Profile) processors. We must therefore
//! save this state to the stack using assembly language, before transferring to
//! an `extern "C"` function. We do not change modes before entering that
//! `extern "C"` function - that's for the handler to deal with as it wishes. We
//! supply a default handler that prints an error message to Semihosting so you
//! know if you hit an unexpected exception. Because FIQ is often
//! performance-sensitive, we don't supply an FIQ trampoline; if you want to use
//! FIQ, you have to write your own assembly routine, allowing you to preserve
//! only whatever state is important to you.
//!
//! If our start-up routine doesn't work for you (e.g. if you have to initialise
//! your memory controller before you touch RAM), supply your own `_start`
//! function (but feel free to call our `_default_start` as part of it).
//!
//! ## Examples
//!
//! You can find example code using QEMU inside the
//! [project repository](https://github.com/rust-embedded/cortex-ar/tree/main/examples)

#![no_std]

use cortex_ar::{
    asm::nop,
    register::{cpsr::ProcessorMode, Cpsr},
};

/// Our default exception handler.
///
/// We end up here if an exception fires and the weak 'PROVIDE' in the link.x
/// file hasn't been over-ridden.
#[no_mangle]
pub extern "C" fn _default_handler() {
    loop {
        nop();
    }
}

// The Interrupt Vector Table, and some default assembly-language handler.
core::arch::global_asm!(
    r#"
    .section .vector_table
    .align 0

    .global _vector_table
    .type _vector_table, %function
    _vector_table:
        ldr     pc, =_start
        ldr     pc, =_asm_undefined_handler
        ldr     pc, =_asm_svc_handler
        ldr     pc, =_asm_prefetch_handler
        ldr     pc, =_asm_abort_handler
        nop
        ldr     pc, =_asm_irq_handler
        ldr     pc, =_asm_fiq_handler
    .size _vector_table, . - _vector_table

    .section .text.handlers

    .global _asm_default_fiq_handler
    .type _asm_default_fiq_handler, %function
    _asm_default_fiq_handler:
        b       _asm_default_fiq_handler
    .size    _asm_default_fiq_handler, . - _asm_default_fiq_handler

    .global _asm_default_handler
    .type _asm_default_handler, %function
    _asm_default_handler:
        b       _asm_default_handler
    .size _asm_default_handler, . - _asm_default_handler
    "#
);

/// This macro expands to code for saving context on entry to an exception
/// handler.
///
/// It should match `restore_context!`.
///
/// On entry to this block, we assume that we are in exception context.
#[cfg(not(any(target_abi = "eabihf", feature = "eabi-fpu")))]
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
#[cfg(not(any(target_abi = "eabihf", feature = "eabi-fpu")))]
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
#[cfg(all(any(target_abi = "eabihf", feature = "eabi-fpu"), feature = "vfp-dp"))]
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
#[cfg(all(any(target_abi = "eabihf", feature = "eabi-fpu"), feature = "vfp-dp"))]
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
core::arch::global_asm!(
    r#"
    .section .text.handlers
    .align 0

    // Called from the vector table when we have an software interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn svc_handler(svc: u32, context: *const u32);`
    .global _asm_svc_handler
    .type _asm_svc_handler, %function
    _asm_svc_handler:
        srsfd   sp!, {svc_mode}
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
    .size _asm_svc_handler, . - _asm_svc_handler


    // Called from the vector table when we have an interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn irq_handler();`
    .global _asm_irq_handler
    .type _asm_irq_handler, %function
    _asm_irq_handler:
        sub     lr, lr, 4
        srsfd   sp!, {irq_mode}
    "#,
    save_context!(),
    r#"
        // call C handler
        bl      _irq_handler
    "#,
    restore_context!(),
    r#"
        rfefd   sp!
    .size _asm_irq_handler, . - _asm_irq_handler


    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _undefined_handler();`
    .global _asm_default_undefined_handler
    .type _asm_default_undefined_handler, %function
    _asm_default_undefined_handler:
        // First adjust LR for two purposes: Passing the faulting instruction to the C handler,
        // and to return to the failing instruction after the C handler returns.
        // Load processor status
        mrs      r4, cpsr
        // Occurred in Thumb state?
        tst      r4, {t_bit}
        // If not in Thumb mode, branch to not_thumb
        beq     not_thumb
        subs    lr, lr, #2
        b       done
not_thumb:
        // Subtract 4 from LR (ARM mode)
        subs    lr, lr, #4
done:
        // state save from compiled code
        srsfd   sp!, {und_mode}
    "#,
    save_context!(),
    r#"
        // Pass the faulting instruction address to the handler.
        mov r0, lr
        // call C handler
        bl      _undefined_handler
    "#,
    restore_context!(),
    r#"
        // Return to the failing instruction which is the recommended approach by ARM.
        rfefd   sp!
    .size _asm_default_undefined_handler, . - _asm_default_undefined_handler


    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _prefetch_handler();`
    .global _asm_default_prefetch_handler
    .type _asm_default_prefetch_handler, %function
    _asm_default_prefetch_handler:
        // Subtract 4 from the stored LR, see p.1212 of the ARMv7-A architecture manual.
        subs    lr, lr, #4
        // state save from compiled code
        srsfd   sp!, {abt_mode}
    "#,
    save_context!(),
    r#"
        // Pass the faulting instruction address to the handler.
        mov     r0, lr
        // call C handler
        bl      _prefetch_handler
    "#,
    restore_context!(),
    r#"
        // Return to the failing instruction which is the recommended approach by ARM.
        rfefd   sp!
    .size _asm_default_prefetch_handler, . - _asm_default_prefetch_handler


    // Called from the vector table when we have an undefined exception.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn _abort_handler();`
    .global _asm_default_abort_handler
    .type _asm_default_abort_handler, %function
    _asm_default_abort_handler:
        // Subtract 8 from the stored LR, see p.1214 of the ARMv7-A architecture manual.
        subs    lr, lr, #8
        // state save from compiled code
        srsfd   sp!, {abt_mode}
    "#,
    save_context!(),
    r#"
        // Pass the faulting instruction address to the handler.
        mov r0, lr
        // call C handler
        bl      _abort_handler
    "#,
    restore_context!(),
    r#"
        // Return to the failing instruction which is the recommended approach by ARM.
        rfefd   sp!
    .size _asm_default_abort_handler, . - _asm_default_abort_handler
    "#,
    svc_mode = const ProcessorMode::Svc as u8,
    irq_mode = const ProcessorMode::Irq as u8,
    und_mode = const ProcessorMode::Und as u8,
    abt_mode = const ProcessorMode::Abt as u8,
    t_bit = const {
        Cpsr::new_with_raw_value(0)
            .with_t(true)
            .raw_value()
    },
);

/// This macro expands to code to turn on the FPU
#[cfg(any(target_abi = "eabihf", feature = "eabi-fpu"))]
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
#[cfg(not(any(target_abi = "eabihf", feature = "eabi-fpu")))]
macro_rules! fpu_enable {
    () => {
        r#"
        // no FPU - do nothing
        "#
    };
}

// Default start-up code for Armv7-A
//
// We set up our stacks and `boot_core` in system mode.
core::arch::global_asm!(
    r#"
    .section .text.startup
    .align 0

    .global _default_start
    .type _default_start, %function
    _default_start:
        // only allow cpu0 through for initialization
        // Read MPIDR
        mrc     p15, 0, r1, c0, c0, 5
        // Extract CPU ID bits. For single-core systems, this should always be 0
        mov     r2, #0xFFFF
        and     r1, r1, r2
        cmp     r1, #0
        beq     1f
    0:
        wfe
        // When Core 0 emits a SEV, the other cores will wake up.
        // Load CPU ID.
        mrc     p15, 0, r0, c0, c0, 5
        // Extract CPU ID bits.
        mov     r2, #0xFFFF
        and     r0, r0, r2
        bl      boot_core
        // Should never returns, loop permanently here.
        b       .
    1:
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
        // Jump to application
        // Load CPU ID, we are CPU0
        ldr    r0, =0x0
        bl      boot_core
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
