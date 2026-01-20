# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/rust-embedded/aarch32/compare/aarch32-cpu-v0.1.0...HEAD
[aarch32-cpu v0.1.0]: https://github.com/rust-embedded/aarch32/compare/cortex-ar-v0.3.0...aarch32-cpu-v0.1.0
[cortex-ar v0.3.0]: https://github.com/rust-embedded/aarch32/compare/cortex-ar-v0.2.0...cortex-ar-v0.3.0
[cortex-ar v0.2.0]: https://github.com/rust-embedded/aarch32/compare/cortex-ar-v0.1.0...cortex-ar-v0.2.0
[cortex-ar v0.1.0]: https://github.com/rust-embedded/aarch32/releases/tag/cortex-ar-v0.1.0
