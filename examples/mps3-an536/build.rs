//! # Build script for the MPS3-AN536 Examples
//!
//! This script only executes when using `cargo` to build the project.
//!
//! Copyright (c) Ferrous Systems, 2025

use std::io::Write;

fn main() {
    arm_targets::process();
    write("memory.x", include_bytes!("memory.x"));
    // Use the cortex-m-rt linker script
    println!("cargo:rustc-link-arg=-Tlink.x");
}

fn write(file: &str, contents: &[u8]) {
    // Put linker file in our output directory and ensure it's on the
    // linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join(file))
        .unwrap()
        .write_all(contents)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", file);
}
