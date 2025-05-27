# Compile-time Support for Arm Targets

This crate looks at your build target (using the `$TARGET` environment variable
that `cargo` sets) and provides a selection of `--cfg` values to `rustc` that
might be useful.

Add to your build dependencies and make a `build.rs` file like this:

```rust
fn main() {
    arm_targets::process();
}
```

Cargo will be given configuration like this:

```text
cargo:rustc-cfg=arm_architecture="v7-r"
cargo:rustc-check-cfg=cfg(arm_architecture, values("v6-m", "v7-m", "v7e-m", "v8-m.base", "v8-m.main", "v7-r", "v8-r", "v7-a", "v8-a"))
cargo:rustc-cfg=arm_isa="A32"
cargo:rustc-check-cfg=cfg(arm_isa, values("A64", "A32", "T32"))
```

This allows you to write Rust code in your firmware like:

```rust
#[cfg(any(arm_architecture = "v7-r", arm_architecture = "v8-r"))]
```

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.59.0 and up, as recorded
by the `package.rust-version` property in `Cargo.toml`.

Increasing the MSRV is not considered a breaking change and may occur in a
minor version release (e.g. from `0.3.0` to `0.3.1`, because this is still a
`0.x` release).

## Licence

* Copyright (c) Ferrous Systems
* Copyright (c) The Rust Embedded Devices Working Group developers

Licensed under either [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
