fn main() {
    // Build the C code into libc_code.a
    cc::Build::new().file("src/library.c").compile("c_code");
    // Ensure the C library isn't stale
    println!("cargo:rerun-if-changed=src/library.c");
    println!("cargo:rerun-if-changed=src/library.h");
    // Make Rust bindings to library.h
    let bindings = bindgen::Builder::default()
        .header("src/library.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .clang_arg("-fshort-enums")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.rs");
}
