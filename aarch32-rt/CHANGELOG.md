# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [aarch32-rt v0.5.0]

### Added

- Improved SMP support - you now only have to supply a
  `_asm_secondary_core_park` function to park/release any secondary cores (such
  cores enter a WFE loop by default)
- New `fn _asm_stack_setup_preallocated(core_id: u32)` function exported
- New `fn _asm_init_segments()` which initialises the `.data` and `.bss` segments
- New `fn _asm_core_start()` which does some core set-up (enable FPU, wipe
  registers, etc) then calls `kmain` or `kmain_secondary`
- New internal (but exported) `_default_kmain_secondary` symbol
- New internal (but exported) `_asm_default_secondary_core_park` symbol

### Removed

- Internal (but exported) `_stack_setup_preallocated` function
- Internal (but exported) `_init_segments` function

## [aarch32-rt v0.4.0]

### Added

- `svc-stack-interrupt` feature to use SVC stack on interrupt, not SYS stack
- Increased MSRV to 1.93
- Some macros that were for internal use only have been renamed/moved

## [aarch32-rt v0.3.0]

### Added

- `el2-mode` feature, to keep CPU in EL2 mode
- Discard entry for `.ARM.exidx` and `.ARM.extab` sections
- Region alignment support, with `_region_alignment` linker symbol
- `__sXXX` and `__eXXX` linker symbols for each output section
- Support for setting up stacks for multiple cores
- Support for exception handling at EL2 (including a new `HypervisorCall` handler)
- `.pushsection` and `.popsection` to all assembly blocks to avoid accidentally changing the section of another piece of code
- New `sections` module for getting information about linker output sections at run-time

### Changed

- Default stack size now 16K, except FIQ and IRQ which are 64 bytes
- `SupervisorCall` now gets a `&Frame` argument
- `_init_segments` function now zeroes out the stack space

## [aarch32-rt v0.2.0]

### Changed

- Reworked stack allocation (PR #93)
- Changed `#[entry]`, `#[exception]` and `#[irq]` to hide the handler function
- Discard `.ARM.exidx` and `.ARM.extab` sections/symbols which are not relevant and could
  otherwise be placed at wrong locations.

## [aarch32-rt v0.1.0]

### Added

- ARMv7-A support, by merging with the old `cortex-a-rt` crate
- ARMv4T and ARMv5TE support
- Thumb mode target support
- `fpu-d32` feature (was called `vfp-dp` in the old `cortex-a-rt`)

### Changed

- Renamed from `cortex-r-rt` to `aarch32-rt`
- Restarted numbering from 0.1.0
- Fixed SVC handling from T32 mode

## [cortex-r-rt v0.2.1]

### Changed

- MSRV is now Rust 1.83
- Uses `cortex-ar` 0.3

## [cortex-r-rt v0.2.0]

### Added

- Added ABT und UND mode stack setup.
- Default exception handlers for undefined, prefetch abort and data abort exceptions
- SMP support
- Zeroing of registers on start-up
- `#[entry]` and `#[exception]` and `#[interrupt]` macros

### Changed

- Fixed interrupt handler so interrupts can be re-entrant
- Default Rust exception handler is now an empty permanent loop instead of a semihosting exit.
- The SVC asm trampoline can now be over-ridden
- The Undefined, Prefetch and Abort handlers can either return never, or can return a new address to continue executing from when the handler is over

## [cortex-r-rt v0.1.0]

Initial release

[Unreleased]: https://github.com/rust-embedded/aarch32/compare/aarch32-rt-v0.5.0...HEAD
[aarch32-rt v0.5.0]: https://github.com/rust-embedded/aarch32/compare/aarch32-rt-v0.4.0...aarch32-rt-v0.5.0
[aarch32-rt v0.4.0]: https://github.com/rust-embedded/aarch32/compare/aarch32-rt-v0.3.0...aarch32-rt-v0.4.0
[aarch32-rt v0.3.0]: https://github.com/rust-embedded/aarch32/compare/aarch32-rt-v0.2.0...aarch32-rt-v0.3.0
[aarch32-rt v0.2.0]: https://github.com/rust-embedded/aarch32/compare/aarch32-rt-v0.1.0...aarch32-rt-v0.2.0
[aarch32-rt v0.1.0]: https://github.com/rust-embedded/aarch32/compare/cortex-r-rt-v0.2.1...aarch32-rt-v0.1.0
[cortex-r-rt v0.2.1]: https://github.com/rust-embedded/aarch32/compare/cortex-r-rt-v0.2.0...cortex-r-rt-v0.2.1
[cortex-r-rt v0.2.0]: https://github.com/rust-embedded/aarch32/compare/cortex-r-rt-v0.1.0...cortex-r-rt-v0.2.0
[cortex-r-rt v0.1.0]: https://github.com/rust-embedded/aarch32/releases/tag/cortex-r-rt-v0.1.0
