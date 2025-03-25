# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [unreleased]

## Changed

- Entry point renamed from `kmain` to `boot_core`. `boot_core` expects one argument
  which will be the CPU ID.

## Added

- Added ABT und UND mode stack setup.
- Default exception handlers for undefined, prefetch and data abort exceptions

## Changed

- Default Rust exception handler is now an empty permanent loop instead of a semihosting exit.

## [v0.1.0]

Initial release

[unreleased]: https://github.com/rust-embedded/cortex-ar/compare/cortex-r-rt-v0.1.0...HEAD
[v0.1.0]: https://github.com/rust-embedded/cortex-ar/releases/tag/cortex-r-rt-v0.1.0
