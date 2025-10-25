//! # Build script for the Cortex-R Runtime
//!
//! This script only executes when using `cargo` to build the project.
//!
//! Copyright (c) Ferrous Systems, 2025

use std::io::Write;

fn main() {
    arm_targets::process();
    write("link.x", include_bytes!("link.x"));
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
