# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- Added support for legacy Arm targets: Armv4 and Armv4T, including proper arch, isa, and profile handling.

## [v0.2.0]

### Added

* `TargetInfo` struct
* Armv7-A support

### Changed

* The `process_target` function returns a `TargetInfo`

## [v0.1.0]

Initial release

[Unreleased]: https://github.com/rust-embedded/cortex-ar/compare/arm-targets-v0.2.0...HEAD
[v0.2.0]: https://github.com/rust-embedded/cortex-ar/compare/arm-targets-v0.1.0...arm-targets-v0.2.0
[v0.1.0]: https://github.com/rust-embedded/cortex-ar/releases/tag/arm-targets-v0.1.0
