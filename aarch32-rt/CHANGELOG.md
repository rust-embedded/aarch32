# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Renamed from `cortex-r-rt` to `aarch32-rt`
- Added ARMv4T and ARMv5TE support
- Added `fpu-d32` feature
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

[Unreleased]: https://github.com/rust-embedded/aarch32/compare/cortex-r-rt-v0.2.1...HEAD
[cortex-r-rt v0.2.1]: https://github.com/rust-embedded/aarch32/compare/cortex-r-rt-v0.2.0...cortex-r-rt-v0.2.1
[cortex-r-rt v0.2.0]: https://github.com/rust-embedded/aarch32/compare/cortex-r-rt-v0.1.0...cortex-r-rt-v0.2.0
[cortex-r-rt v0.1.0]: https://github.com/rust-embedded/aarch32/releases/tag/cortex-r-rt-v0.1.0
