# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0]

### Added

- General support for the Cortex-A architecture.
- New `sev` function in ASM module.
- Added multi-core-safe critical-section implementation
- Additional EL1 MPU methods `set_region`, `set_attributes` and `background_region_enable`

### Changed

- Timer methods only need `&self` not `&mut self`
- The `dsb` and `isb` functions now include compiler fences
- Added `nomem`, `nostack` and `preserves_flags` options for ASM where applicable.

## [v0.1.0]

Initial release

[Unreleased]: https://github.com/rust-embedded/cortex-ar/compare/cortex-ar-v0.2.0...HEAD
[v0.2.0]: https://github.com/rust-embedded/cortex-ar/compare/cortex-ar-v0.1.0...cortex-ar-v0.2.0
[v0.1.0]: https://github.com/rust-embedded/cortex-ar/releases/tag/cortex-ar-v0.1.0
