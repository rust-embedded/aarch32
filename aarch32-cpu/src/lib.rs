//! CPU/peripheral support for Arm AArch32
#![no_std]

mod critical_section;

#[cfg(any(
    doc,
    arm_architecture = "v7-a",
    arm_architecture = "v7-r",
    arm_architecture = "v8-r"
))]
#[path = "asmv7.rs"]
pub mod asm;

#[cfg(not(any(
    doc,
    arm_architecture = "v7-a",
    arm_architecture = "v7-r",
    arm_architecture = "v8-r"
)))]
#[path = "asmv4.rs"]
pub mod asm;

pub mod cache;
pub mod interrupt;
pub mod mmu;
pub mod register;
#[cfg(target_arch = "arm")]
pub mod stacks;

#[cfg(any(test, doc, arm_architecture = "v7-r"))]
pub mod pmsav7;

#[cfg(any(test, doc, arm_architecture = "v8-r"))]
pub mod generic_timer;

#[cfg(any(test, doc, arm_architecture = "v8-r"))]
pub mod pmsav8;

/// Generate an SVC call with no parameters.
///
/// Puts the first argument in the instruction. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// let value = svc!(0xFF);
/// ```
#[macro_export]
macro_rules! svc {
    ($num:expr) => { {
        let retval: u32;
        unsafe {
            core::arch::asm!("svc {arg}", arg = const $num, lateout("r0") retval);
        }
        retval
    } }
}

/// Generate an SVC call with 1 parameters
///
/// Puts the first argument in the instruction, and the parameter in r0. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const SYSCALL_FOO: u32 = 0x100;
/// let result = svc1!(0x00, SYSCALL_FOO);
/// ```
#[macro_export]
macro_rules! svc1 {
    ($num:expr, $arg0:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        unsafe {
            core::arch::asm!(
                // Do the SVCall
                "svc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval);
        }
        retval
    } }
}

/// Generate an SVC call with 2 parameters
///
/// Puts the first argument in the instruction, and the parameters in r0-r1. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const SYSCALL_FOO: u32 = 0x100;
/// let result = svc2!(0x00, SYSCALL_FOO, 1);
/// ```
#[macro_export]
macro_rules! svc2 {
    ($num:expr, $arg0:expr, $arg1:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        unsafe {
            core::arch::asm!(
                // Do the SVCall
                "svc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval,
                in("r1") arg1);
        }
        retval
    } }
}

/// Generate an SVC call with 3 parameters
///
/// Puts the first argument in the instruction, and the parameters in r0-r2. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const SYSCALL_FOO: u32 = 0x100;
/// let result = svc3!(0x00, SYSCALL_FOO, 1, 2);
/// ```
#[macro_export]
macro_rules! svc3 {
    ($num:expr, $arg0:expr, $arg1:expr, $arg2:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        let arg2: u32 = $arg2;
        unsafe {
            core::arch::asm!(
                // Do the SVCall
                "svc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval,
                in("r1") arg1,
                in("r2") arg2);
        }
        retval
    } }
}

/// Generate an SVC call with 4 parameters
///
/// Puts the first argument in the instruction, and the parameters in r0-r3. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const SYSCALL_FOO: u32 = 0x100;
/// let result = svc4!(0x00, SYSCALL_FOO, 1, 2, 3);
/// ```
#[macro_export]
macro_rules! svc4 {
    ($num:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        let arg2: u32 = $arg2;
        let arg3: u32 = $arg3;
        unsafe {
            core::arch::asm!(
                // Do the SVCall
                "svc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval,
                in("r1") arg1,
                in("r2") arg2,
                in("r3") arg3);
        }
        retval
    } }
}

/// Generate an SVC call with 5 parameters
///
/// Puts the first argument in the instruction, and the parameters in r0-r4. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const SYSCALL_FOO: u32 = 0x100;
/// let result = svc5!(0x00, SYSCALL_FOO, 1, 2, 3, 4);
/// ```
#[macro_export]
macro_rules! svc5 {
    ($num:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        let arg2: u32 = $arg2;
        let arg3: u32 = $arg3;
        let arg4: u32 = $arg4;
        unsafe {
            core::arch::asm!(
                // Do the SVCall
                "svc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval,
                in("r1") arg1,
                in("r2") arg2,
                in("r3") arg3,
                in("r4") arg4);
        }
        retval
    } }
}

/// Generate an SVC call with 6 parameters
///
/// Puts the first argument in the instruction, and the parameters in r0-r5. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const SYSCALL_FOO: u32 = 0x100;
/// let result = svc6!(0x00, SYSCALL_FOO, 1, 2, 3, 4, 5);
/// ```
#[macro_export]
macro_rules! svc6 {
    ($num:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        let arg2: u32 = $arg2;
        let arg3: u32 = $arg3;
        let arg4: u32 = $arg4;
        let arg5: u32 = $arg5;
        unsafe {
            core::arch::asm!(
                // Do the SVCall
                "svc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval,
                in("r1") arg1,
                in("r2") arg2,
                in("r3") arg3,
                in("r4") arg4,
                in("r5") arg5);
        }
        retval
    } }
}

/// Generate an HVC call with the given argument.
#[macro_export]
macro_rules! hvc {
    ($r0:expr) => {
        unsafe {
            core::arch::asm!("hvc {arg}", arg = const $r0);
        }
    }
}

/// Generate an HVC call with 1 parameters
///
/// Puts the first argument in the instruction, and the parameter in r0. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const HYPERCALL_FOO: u32 = 0x100;
/// let result = hvc1!(0x00, HYPERCALL_FOO);
/// ```
#[macro_export]
macro_rules! hvc1 {
    ($num:expr, $arg0:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        unsafe {
            core::arch::asm!(
                // Do the Hyper-call
                "hvc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval);
        }
        retval
    } }
}

/// Generate an HVC call with 2 parameters
///
/// Puts the first argument in the instruction, and the parameters in r0-r1. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const HYPERCALL_FOO: u32 = 0x100;
/// let result = hvc2!(0x00, HYPERCALL_FOO, 1);
/// ```
#[macro_export]
macro_rules! hvc2 {
    ($num:expr, $arg0:expr, $arg1:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        unsafe {
            core::arch::asm!(
                // Do the Hyper-call
                "hvc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval,
                in("r1") arg1);
        }
        retval
    } }
}

/// Generate an HVC call with 3 parameters
///
/// Puts the first argument in the instruction, and the parameters in r0-r2. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const HYPERCALL_FOO: u32 = 0x100;
/// let result = hvc3!(0x00, HYPERCALL_FOO, 1, 2);
/// ```
#[macro_export]
macro_rules! hvc3 {
    ($num:expr, $arg0:expr, $arg1:expr, $arg2:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        let arg2: u32 = $arg2;
        unsafe {
            core::arch::asm!(
                // Do the Hyper-call
                "hvc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval,
                in("r1") arg1,
                in("r2") arg2);
        }
        retval
    } }
}

/// Generate an HVC call with 4 parameters
///
/// Puts the first argument in the instruction, and the parameters in r0-r3. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const HYPERCALL_FOO: u32 = 0x100;
/// let result = hvc4!(0x00, HYPERCALL_FOO, 1, 2, 3);
/// ```
#[macro_export]
macro_rules! hvc4 {
    ($num:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        let arg2: u32 = $arg2;
        let arg3: u32 = $arg3;
        unsafe {
            core::arch::asm!(
                // Do the Hyper-call
                "hvc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval,
                in("r1") arg1,
                in("r2") arg2,
                in("r3") arg3);
        }
        retval
    } }
}

/// Generate an HVC call with 5 parameters
///
/// Puts the first argument in the instruction, and the parameters in r0-r4. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const HYPERCALL_FOO: u32 = 0x100;
/// let result = hvc5!(0x00, HYPERCALL_FOO, 1, 2, 3, 4);
/// ```
#[macro_export]
macro_rules! hvc5 {
    ($num:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        let arg2: u32 = $arg2;
        let arg3: u32 = $arg3;
        let arg4: u32 = $arg4;
        unsafe {
            core::arch::asm!(
                // Do the Hyper-call
                "hvc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval,
                in("r1") arg1,
                in("r2") arg2,
                in("r3") arg3,
                in("r4") arg4);
        }
        retval
    } }
}

/// Generate an HVC call with 6 parameters
///
/// Puts the first argument in the instruction, and the parameters in r0-r5. Gives you back
/// the value left in `r0` by the handler.
///
/// ```rust,ignore
/// const HYPERCALL_FOO: u32 = 0x100;
/// let result = hvc6!(0x00, HYPERCALL_FOO, 1, 2, 3, 4, 5);
/// ```
#[macro_export]
macro_rules! hvc6 {
    ($num:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        let arg2: u32 = $arg2;
        let arg3: u32 = $arg3;
        let arg4: u32 = $arg4;
        let arg5: u32 = $arg5;
        unsafe {
            core::arch::asm!(
                // Do the Hyper-call
                "hvc     {arg}",
                arg = const $num,
                inout("r0") arg0 => retval,
                in("r1") arg1,
                in("r2") arg2,
                in("r3") arg3,
                in("r4") arg4,
                in("r5") arg5);
        }
        retval
    } }
}
