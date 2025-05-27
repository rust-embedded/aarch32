# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0]

## Added

- Added ABT und UND mode stack setup.
- Default exception handlers for undefined, prefetch abort and data abort exceptions
- SMP support
- Zeroing of registers on start-up
- `#[entry]` and `#[exception]` and `#[interrupt]` macros

## Changed

- Fixed interrupt handler so interrupts can be re-entrant
- Default Rust exception handler is now an empty permanent loop instead of a semihosting exit.
- The SVC asm trampoline can now be over-ridden
- The Undefined, Prefetch and Abort handlers can either return never, or can return a new address to continue executing from when the handler is over

## [v0.1.0]

Initial release

[Unreleased]: https://github.com/rust-embedded/cortex-ar/compare/cortex-r-rt-v0.2.0...HEAD
[v0.2.0]: https://github.com/rust-embedded/cortex-ar/compare/cortex-r-rt-v0.1.0...cortex-r-rt-v0.2.0
[v0.1.0]: https://github.com/rust-embedded/cortex-ar/releases/tag/cortex-r-rt-v0.1.0
