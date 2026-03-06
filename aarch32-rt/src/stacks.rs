//! Code for examining linker allocated stacks

/// Represents one of the AArch32 stacks
#[derive(Debug, Copy, Clone)]
pub enum Stack {
    /// UND mode stack
    Und,
    /// SVC mode stack
    Svc,
    /// ABT mode stack, for data abort and prefetch abort
    Abt,
    /// HYP mode stack, for EL2
    Hyp,
    /// IRQ mode stack, for interrupts
    Irq,
    /// FIQ mode stack, for fast interrupts
    Fiq,
    /// SYS mode stack, for the main thread
    Sys,
}

impl core::fmt::Display for Stack {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Stack::Und => "UND",
                Stack::Svc => "SVC",
                Stack::Abt => "ABT",
                Stack::Hyp => "HYP",
                Stack::Irq => "IRQ",
                Stack::Fiq => "FIQ",
                Stack::Sys => "SYS",
            }
        )
    }
}

impl Stack {
    /// Create an iterator over all the stacks
    pub fn iter() -> impl Iterator<Item = Stack> {
        [
            Stack::Und,
            Stack::Svc,
            Stack::Abt,
            Stack::Hyp,
            Stack::Irq,
            Stack::Fiq,
            Stack::Sys,
        ]
        .iter()
        .cloned()
    }

    /// Get the highest address of this stack, for the given core
    pub fn top(&self, core: usize) -> Option<*const u32> {
        if core > Self::num_cores() {
            return None;
        }
        let top = self.stack_top();
        let core = core as isize;
        let per_core_size_words = self.per_core_size_words();
        Some(unsafe { top.offset(-per_core_size_words * core) })
    }

    /// Get the lowest address of this stack, for the given core
    pub fn bottom(&self, core: usize) -> Option<*const u32> {
        self.top(core)
            .map(|p| unsafe { p.offset(-self.per_core_size_words()) })
    }

    /// Get the range of this stack, for the given core
    pub fn range(&self, core: usize) -> Option<core::ops::Range<*const u32>> {
        if let (Some(bottom), Some(top)) = (self.bottom(core), self.top(core)) {
            Some(bottom..top)
        } else {
            None
        }
    }

    /// Get number of cores in this system
    pub fn num_cores() -> usize {
        unsafe extern "C" {
            safe static _num_cores: u8;
        }
        core::ptr::addr_of!(_num_cores) as usize
    }

    /// Get the total size of this stack across all cores
    pub fn per_core_size_words(&self) -> isize {
        use core::ptr::addr_of;
        unsafe extern "C" {
            static _und_stack_size: u8;
            static _svc_stack_size: u8;
            static _abt_stack_size: u8;
            static _hyp_stack_size: u8;
            static _irq_stack_size: u8;
            static _fiq_stack_size: u8;
            static _sys_stack_size: u8;
        }
        let size_bytes = match self {
            Stack::Und => addr_of!(_und_stack_size) as isize,
            Stack::Svc => addr_of!(_svc_stack_size) as isize,
            Stack::Abt => addr_of!(_abt_stack_size) as isize,
            Stack::Hyp => addr_of!(_hyp_stack_size) as isize,
            Stack::Irq => addr_of!(_irq_stack_size) as isize,
            Stack::Fiq => addr_of!(_fiq_stack_size) as isize,
            Stack::Sys => addr_of!(_sys_stack_size) as isize,
        };
        size_bytes / 4
    }

    /// Get the top address for this stack
    fn stack_top(&self) -> *const u32 {
        use core::ptr::addr_of;
        unsafe extern "C" {
            static _und_stack_high_end: u32;
            static _svc_stack_high_end: u32;
            static _abt_stack_high_end: u32;
            static _hyp_stack_high_end: u32;
            static _irq_stack_high_end: u32;
            static _fiq_stack_high_end: u32;
            static _sys_stack_high_end: u32;
        }
        match self {
            Stack::Und => addr_of!(_und_stack_high_end),
            Stack::Svc => addr_of!(_svc_stack_high_end),
            Stack::Abt => addr_of!(_abt_stack_high_end),
            Stack::Hyp => addr_of!(_hyp_stack_high_end),
            Stack::Irq => addr_of!(_irq_stack_high_end),
            Stack::Fiq => addr_of!(_fiq_stack_high_end),
            Stack::Sys => addr_of!(_sys_stack_high_end),
        }
    }
}
