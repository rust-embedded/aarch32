# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.4.0]

### Added

- Added `Arch::Armv6`

### Changed

- Targets starting with `thumb` are identified as T32 targets

## [v0.3.0]

### Added

- Support for legacy Arm targets: Armv5TE and Armv4T
- Additional documentation
- ABI type (either EABI or EABIHF)
- Simple CLI tool
- Some unit tests

## [v0.2.0]

### Added

* `TargetInfo` struct
* Armv7-A support

### Changed

* The `process_target` function returns a `TargetInfo`

## [v0.1.0]

Initial release

[Unreleased]: https://github.com/rust-embedded/aarch32/compare/arm-targets-v0.4.0...HEAD
[v0.4.0]: https://github.com/rust-embedded/aarch32/compare/arm-targets-v0.3.0...arm-targets-v0.4.0
[v0.3.0]: https://github.com/rust-embedded/aarch32/compare/arm-targets-v0.2.0...arm-targets-v0.3.0
[v0.2.0]: https://github.com/rust-embedded/aarch32/compare/arm-targets-v0.1.0...arm-targets-v0.2.0
[v0.1.0]: https://github.com/rust-embedded/aarch32/releases/tag/arm-targets-v0.1.0
