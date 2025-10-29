//! # Build script for the Cortex-R library
//!
//! This script only executes when using `cargo` to build the project.
//!
//! Copyright (c) Ferrous Systems, 2025

fn main() {
    arm_targets::process();
}
