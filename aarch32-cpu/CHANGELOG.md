# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [aarch32-cpu v0.3.0]

### Added

- Added `Iciallu` register which allows invalidating the instruction cache.
- Added `asm::fiq_enable` and `asm::fiq_disable`
- Added `stacks::stack_used_bytes` to count how much stack has been used
- Added `svc1!`-`svc6!` macros for making syscalls
- Added `hvc!` and `hvc1!`-`hvc6!` macros for making hypercalls
- Added `mmu::L1Table` type for basic MMU L1 page-tables
- Added `Prlar::limit_address` method
- Added `Prbar::base_address` method
- `register::vbar` and `register::Vbar` are also available for ARMv7-A now.
- Added `defmt` implementations for PMSA types.

### Changed

- Updated `bitbybit` crate to version 2
- Updated `SysRegRead::read_raw` and `SysRegRead64::read_raw` to now be safe operations
- Updated `Dfsr` datatype to support a range of Arm architecture versions
- Updated Arm Generic Timer support - now works on Armv7-A as well
- Updated Hypervisor support - now works on Armv7-A as well

### Removed

- Removed `__sync_synchronize` function

## [aarch32-cpu v0.2.0]

- Mark `asm::irq_enable()` as unsafe to match `interrupt::enable()`

## [aarch32-cpu v0.1.0]

### Added

- ARMv4T and ARMv5TE support
- Thumb mode target support

### Changed

- Renamed from `cortex-ar` to `aarch32-cpu`
- Restarted numbering from 0.1.0
- All BAR register types now hold plain `u32`, not `*mut u32` - fixes issues with `serde` derives on some types

## [cortex-ar v0.3.0]

- Bumped MSRV to v1.83 to allow compatibility with `arbitrary-int` v2.

### Added

- `dmb` data memory barrier in ASM module.
- API for inner cache maintenance as part of the new `cache` module. This
  includes functions to completely clean, invalidate or clean & invalidate the
  L1 data cache or perform data cache maintenance by MVA (specific address).
- new  `L1Section::set_section_attrs` and `L1Section::section_attrs` method,
  and low-level `L1Section::new_with_addr_upper_bits_and_attrs` constructor
- `Debug`, `Copy`, `Clone` derives for all system register types
- optional `serde` derives behind a `serde` feature gate
- optional `defmt::Format` derives behind a `defmt` feature gate

### Changed

- MMU code: Use more `arbitrary-int` types for MMU configuration bits.
- Renamed `L1Section::new` to `L1Section::new_with_addr_and_attrs`.
- Bumped `defmt` to v1
- Bumped `arbitrary-int` to v2

## [cortex-ar v0.2.0]

### Added

- General support for the Cortex-A architecture.
- New `sev` function in ASM module.
- Added multi-core-safe critical-section implementation
- Additional EL1 MPU methods `set_region`, `set_attributes` and `background_region_enable`

### Changed

- Timer methods only need `&self` not `&mut self`
- The `dsb` and `isb` functions now include compiler fences
- Added `nomem`, `nostack` and `preserves_flags` options for ASM where applicable.

## [cortex-ar v0.1.0]

Initial release

[Unreleased]: https://github.com/rust-embedded/aarch32/compare/aarch32-cpu-v0.3.0...HEAD
[aarch32-cpu v0.3.0]: https://github.com/rust-embedded/aarch32/compare/aarch32-cpu-v0.2.0...aarch32-cpu-v0.3.0
[aarch32-cpu v0.2.0]: https://github.com/rust-embedded/aarch32/compare/aarch32-cpu-v0.1.0...aarch32-cpu-v0.2.0
[aarch32-cpu v0.1.0]: https://github.com/rust-embedded/aarch32/compare/cortex-ar-v0.3.0...aarch32-cpu-v0.1.0
[cortex-ar v0.3.0]: https://github.com/rust-embedded/aarch32/compare/cortex-ar-v0.2.0...cortex-ar-v0.3.0
[cortex-ar v0.2.0]: https://github.com/rust-embedded/aarch32/compare/cortex-ar-v0.1.0...cortex-ar-v0.2.0
[cortex-ar v0.1.0]: https://github.com/rust-embedded/aarch32/releases/tag/cortex-ar-v0.1.0
