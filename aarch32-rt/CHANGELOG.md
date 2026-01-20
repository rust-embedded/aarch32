# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [aarch32-rt v0.2.0]

### Changed

- Reworked stack allocation (PR #93)
- Changed `#[entry]`, `#[exception]` and `#[irq]` to hide the handler function

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

[Unreleased]: https://github.com/rust-embedded/aarch32/compare/aarch32-rt-v0.2.0...HEAD
[aarch32-rt v0.2.0]: https://github.com/rust-embedded/aarch32/compare/aarch32-rt-v0.1.0...aarch32-rt-v0.2.0
[aarch32-rt v0.1.0]: https://github.com/rust-embedded/aarch32/compare/cortex-r-rt-v0.2.1...aarch32-rt-v0.1.0
[cortex-r-rt v0.2.1]: https://github.com/rust-embedded/aarch32/compare/cortex-r-rt-v0.2.0...cortex-r-rt-v0.2.1
[cortex-r-rt v0.2.0]: https://github.com/rust-embedded/aarch32/compare/cortex-r-rt-v0.1.0...cortex-r-rt-v0.2.0
[cortex-r-rt v0.1.0]: https://github.com/rust-embedded/aarch32/releases/tag/cortex-r-rt-v0.1.0
