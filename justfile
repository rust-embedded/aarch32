# The rust-embedded/aarch32 Just file
#
# You need to install `just` from https://github.com/casey/just to use
# this file

# If you run with `just --set v 1` then we make cargo run in verbose mode
v := "0"
verbose := if v == "1" { "--verbose" } else { "" }

# The aarch32-tests harness invocation shared by every test-qemu recipe. Each
# recipe appends a name filter; the build matrix lives in the harness itself.
qemu_test := "cargo test -p aarch32-tests --test tests -- --test-threads=1"

# Our default target. It does everything that you might want to do pre-checkin.
check: build-all build-all-examples doc-all fmt-check clippy-all test

github_info:
	echo 'msrv_targets=["armv7a-none-eabi","armv7r-none-eabi","armv7r-none-eabihf"]'
	echo 'stable_targets=["armv7a-none-eabi","armv7a-none-eabihf","armv7r-none-eabi","armv7r-none-eabihf","armv8r-none-eabihf"]'
	echo 'nightly_tier2_targets=["armv7r-none-eabi","thumbv7r-none-eabi","armv7r-none-eabihf","thumbv7r-none-eabihf","armv7a-none-eabi","thumbv7a-none-eabi","armv7a-none-eabihf","thumbv7a-none-eabihf","armv8r-none-eabihf","thumbv8r-none-eabihf"]'
	echo 'nightly_tier3_targets=["armv6-none-eabi","thumbv6-none-eabi","armv6-none-eabihf"]'
	echo 'nightly_tier3_noatomic_targets=["armv4t-none-eabi","thumbv4t-none-eabi","armv5te-none-eabi","thumbv5te-none-eabi"]'

# Cleans up all the target folders
clean:
	# The cross-compiled workspace
	cargo clean
	# The host-compiled helper library
	cd arm-targets && cargo clean
	# The cross-compiled examples
	cd examples/versatileab && cargo clean
	rm -rf examples/versatileab/target-d32
	cd examples/mps3-an536 && cargo clean
	rm -rf examples/mps3-an536/target-d32
	cd examples/xilinx-zynq-a9 && cargo clean
	rm -rf examples/xilinx-zynq-a9/target-d32

# Builds our workspace for all targets
build-all: \
	build-arm-targets \
	(build-tier3-no-atomics "armv4t-none-eabi") \
	(build-tier3-no-atomics "thumbv4t-none-eabi") \
	(build-tier3-no-atomics "armv5te-none-eabi") \
	(build-tier3-no-atomics "thumbv5te-none-eabi") \
	(build-tier3 "armv6-none-eabi") \
	(build-tier3 "thumbv6-none-eabi") \
	(build-tier3 "armv6-none-eabihf") \
	(build-tier2 "armv7r-none-eabi") \
	(build-tier2 "thumbv7r-none-eabi") \
	(build-tier2 "armv7r-none-eabihf") \
	(build-tier2 "thumbv7r-none-eabihf") \
	(build-tier2 "armv7a-none-eabi") \
	(build-tier2 "thumbv7a-none-eabi") \
	(build-tier2 "armv7a-none-eabihf") \
	(build-tier2 "thumbv7a-none-eabihf") \
	(build-tier2 "armv8r-none-eabihf") \
	(build-tier2 "thumbv8r-none-eabihf") \

# Build the arm-targets library
build-arm-targets:
	cd arm-targets && cargo build {{verbose}}

# Builds our workspace with various features, building core from source, but skipping anything that requires atomics
build-tier3-no-atomics target:
    cargo build --target {{target}} -Zbuild-std=core {{verbose}}
    cargo build --target {{target}} -Zbuild-std=core --features "serde, defmt, critical-section-single-core, check-asm" {{verbose}}

# Builds our workspace with various features, building core from source
build-tier3 target:
    cargo build --target {{target}} -Zbuild-std=core {{verbose}}
    cargo build --target {{target}} -Zbuild-std=core --features "serde, defmt, critical-section-multi-core, check-asm" {{verbose}}
    cargo build --target {{target}} -Zbuild-std=core --features "serde, defmt, critical-section-single-core, check-asm" {{verbose}}

# Builds our workspace with various features
build-tier2 target:
    cargo build --target {{target}} {{verbose}}
    cargo build --target {{target}} --features "serde, defmt, critical-section-multi-core, check-asm" {{verbose}}
    cargo build --target {{target}} --features "serde, defmt, critical-section-single-core, check-asm" {{verbose}}

# Builds our examples for each target, which also builds our cross-compiled workspace
build-all-examples: \
	(build-versatileab-tier3 "armv4t-none-eabi") \
	(build-versatileab-tier3 "thumbv4t-none-eabi") \
	(build-versatileab-tier3 "armv5te-none-eabi") \
	(build-versatileab-tier3 "thumbv5te-none-eabi") \
	(build-versatileab-tier3 "armv6-none-eabi") \
	(build-versatileab-tier3 "armv6-none-eabihf") \
	(build-versatileab-tier3 "thumbv6-none-eabi") \
	(build-versatileab-tier2 "armv7r-none-eabi") \
	(build-versatileab-tier2 "thumbv7r-none-eabi") \
	(build-versatileab-tier2 "armv7r-none-eabihf") \
	(build-versatileab-tier2 "thumbv7r-none-eabihf") \
	(build-versatileab-tier2 "armv7a-none-eabi") \
	(build-versatileab-tier2 "thumbv7a-none-eabi") \
	(build-versatileab-tier2 "armv7a-none-eabihf") \
	(build-versatileab-tier2 "thumbv7a-none-eabihf") \
	(build-mps3-tier2        "armv8r-none-eabihf") \
	(build-mps3-tier2        "thumbv8r-none-eabihf") \
	(build-zynq-tier2        "armv7a-none-eabi") \
	(build-zynq-tier2        "thumbv7a-none-eabi") \
	(build-zynq-tier2        "armv7a-none-eabihf") \
	(build-zynq-tier2        "thumbv7a-none-eabihf") \

# Builds the Versatile AB examples, building core from source
build-versatileab-tier3 target:
	cd examples/versatileab && cargo build --target={{target}} -Zbuild-std=core {{verbose}}

# Builds the Versatile AB examples, assuming core has been prebuilt
build-versatileab-tier2 target:
	cd examples/versatileab && cargo build --target={{target}} {{verbose}}

# Builds the MPS3-AN536 examples, building core from source
build-mps3-tier3 target:
	cd examples/mps3-an536 && cargo build --target={{target}} -Zbuild-std=core {{verbose}}
	cd examples/mps3-an536-el2 && cargo build --target={{target}} -Zbuild-std=core {{verbose}}

# Builds the MPS3-AN536 examples, assuming core has been prebuilt
build-mps3-tier2 target:
	cd examples/mps3-an536 && cargo build --target={{target}} {{verbose}}
	cd examples/mps3-an536-el2 && cargo build --target={{target}} {{verbose}}

# Builds the Xilinx Zynq-A9 examples, assuming core has been prebuilt
build-zynq-tier2 target:
	cd examples/xilinx-zynq-a9 && cargo build --target={{target}} {{verbose}}

# Documents our workspace for all targets
doc-all: \
	doc-arm-targets \
	(doc-tier3-no-atomics "armv4t-none-eabi") \
	(doc-tier3-no-atomics "thumbv4t-none-eabi") \
	(doc-tier3-no-atomics "armv5te-none-eabi") \
	(doc-tier3-no-atomics "thumbv5te-none-eabi") \
	(doc-tier3 "armv6-none-eabi") \
	(doc-tier3 "thumbv6-none-eabi") \
	(doc-tier3 "armv6-none-eabihf") \
	(doc-tier2 "armv7r-none-eabi") \
	(doc-tier2 "thumbv7r-none-eabi") \
	(doc-tier2 "armv7r-none-eabihf") \
	(doc-tier2 "thumbv7r-none-eabihf") \
	(doc-tier2 "armv7a-none-eabi") \
	(doc-tier2 "thumbv7a-none-eabi") \
	(doc-tier2 "armv7a-none-eabihf") \
	(doc-tier2 "thumbv7a-none-eabihf") \
	(doc-tier2 "armv8r-none-eabihf") \
	(doc-tier2 "thumbv8r-none-eabihf") \

# Document the arm-targets library
doc-arm-targets:
	cd arm-targets && RUSTDOCFLAGS=-Dwarnings cargo doc {{verbose}}

# Documents our workspace with various features, building core from source, but skipping anything that requires atomics
doc-tier3-no-atomics target:
    RUSTDOCFLAGS=-Dwarnings cargo doc --target {{target}} -Zbuild-std=core {{verbose}}
    RUSTDOCFLAGS=-Dwarnings cargo doc --target {{target}} -Zbuild-std=core --features "serde, defmt, critical-section-single-core, check-asm" {{verbose}}

# Documents our workspace with various features, building core from source
doc-tier3 target:
    RUSTDOCFLAGS=-Dwarnings cargo doc --target {{target}} -Zbuild-std=core {{verbose}}
    RUSTDOCFLAGS=-Dwarnings cargo doc --target {{target}} -Zbuild-std=core --features "serde, defmt, critical-section-multi-core, check-asm" {{verbose}}
    RUSTDOCFLAGS=-Dwarnings cargo doc --target {{target}} -Zbuild-std=core --features "serde, defmt, critical-section-single-core, check-asm" {{verbose}}

# Documents our workspace with various features
doc-tier2 target:
    RUSTDOCFLAGS=-Dwarnings cargo doc --target {{target}} {{verbose}}
    RUSTDOCFLAGS=-Dwarnings cargo doc --target {{target}} --features "serde, defmt, critical-section-multi-core, check-asm" {{verbose}}
    RUSTDOCFLAGS=-Dwarnings cargo doc --target {{target}} --features "serde, defmt, critical-section-single-core, check-asm" {{verbose}}

# Formats all the code
fmt:
	# The cross-compiled workspace
	cargo fmt {{verbose}}
	# The host-compiled helper library
	cd arm-targets && cargo fmt {{verbose}}
	# The cross-compiled examples	cargo fmt
	cd examples/versatileab && cargo fmt {{verbose}}
	cd examples/mps3-an536 && cargo fmt {{verbose}}
	cd examples/mps3-an536-el2 && cargo fmt {{verbose}}
	cd examples/xilinx-zynq-a9 && cargo fmt {{verbose}}

# Checks all the code is formatted
fmt-check:
	# The cross-compiled workspace
	cargo fmt --check
	# The host-compiled helper library
	cd arm-targets && cargo fmt --check {{verbose}}
	# The cross-compiled examples	cargo fmt
	cd examples/versatileab && cargo fmt --check {{verbose}}
	cd examples/mps3-an536 && cargo fmt --check {{verbose}}
	cd examples/mps3-an536-el2 && cargo fmt --check {{verbose}}
	cd examples/xilinx-zynq-a9 && cargo fmt --check {{verbose}}

# Checks all the cross-compiled workspace passes the clippy lints
clippy-all: \
	clippy-arm-targets \
	clippy-examples \
	(clippy-tier3-no-atomics "armv4t-none-eabi") \
	(clippy-tier3-no-atomics "thumbv4t-none-eabi") \
	(clippy-tier3-no-atomics "armv5te-none-eabi") \
	(clippy-tier3-no-atomics "thumbv5te-none-eabi") \
	(clippy-tier3 "armv6-none-eabi") \
	(clippy-tier3 "thumbv6-none-eabi") \
	(clippy-tier3 "armv6-none-eabihf") \
	(clippy-tier2 "armv7r-none-eabi") \
	(clippy-tier2 "thumbv7r-none-eabi") \
	(clippy-tier2 "armv7r-none-eabihf") \
	(clippy-tier2 "thumbv7r-none-eabihf") \
	(clippy-tier2 "armv7a-none-eabi") \
	(clippy-tier2 "thumbv7a-none-eabi") \
	(clippy-tier2 "armv7a-none-eabihf") \
	(clippy-tier2 "thumbv7a-none-eabihf") \
	(clippy-tier2 "armv8r-none-eabihf") \
	(clippy-tier2 "thumbv8r-none-eabihf") \

# Checks the arm-targets code passes the clippy lints
clippy-arm-targets:
	# The cross-compiled workspace
	cargo clippy {{verbose}}
	# The host-compiled helper library
	cd arm-targets && cargo clippy {{verbose}}

# Checks the examples pass the clippy lints
clippy-examples:
	cd examples/versatileab && cargo clippy --target=armv7r-none-eabi {{verbose}}
	cd examples/mps3-an536 && cargo clippy --target=armv8r-none-eabihf {{verbose}}
	cd examples/mps3-an536-el2 && cargo clippy --target=armv8r-none-eabihf {{verbose}}
	cd examples/xilinx-zynq-a9 && cargo clippy --target=armv7a-none-eabi {{verbose}}

# Checks all the cross-compiled workspace passes the clippy lints
clippy-tier3-no-atomics target:
    cargo clippy --target {{target}} -Zbuild-std=core {{verbose}}
    cargo clippy --target {{target}} -Zbuild-std=core --features "serde, defmt, critical-section-single-core, check-asm" {{verbose}}

# Checks all the cross-compiled workspace passes the clippy lints
clippy-tier3 target:
    cargo clippy --target {{target}} -Zbuild-std=core {{verbose}}
    cargo clippy --target {{target}} -Zbuild-std=core --features "serde, defmt, critical-section-multi-core, check-asm" {{verbose}}
    cargo clippy --target {{target}} -Zbuild-std=core --features "serde, defmt, critical-section-single-core, check-asm" {{verbose}}

# Checks all the cross-compiled workspace passes the clippy lints
clippy-tier2 target:
    cargo clippy --target {{target}} {{verbose}}
    cargo clippy --target {{target}} --features "serde, defmt, critical-section-multi-core, check-asm" {{verbose}}
    cargo clippy --target {{target}} --features "serde, defmt, critical-section-single-core, check-asm" {{verbose}}

# Run all the tests
test: test-cargo test-qemu

# Run the unit tests with cargo. Excludes aarch32-tests, whose tests drive QEMU
# (run those via `just test-qemu`).
test-cargo:
	# The cross-compiled workspace
	cargo test --workspace --exclude aarch32-tests {{verbose}}
	# The host-compiled helper library
	cd arm-targets && cargo test {{verbose}}

# qemu-based snapshot tests. The whole build matrix (targets, tiers, and the
# svc/fpu-d32 variants) lives in the aarch32-tests harness, which lists one test
# per target. `just test-qemu` runs them all; the per-arch recipes are just name
# filters so a developer can run only the arch they are working on, e.g.
# `just test-qemu-v7a`. The recipe names mirror the matrix in
# .github/workflows/build.yml. Bootstrap/refresh snapshots with INSTA_UPDATE=always.
test-qemu:
	{{qemu_test}}

# Armv4T (Versatile AB)
test-qemu-v4t:
	{{qemu_test}} v4t

# Armv5TE (Versatile AB)
test-qemu-v5te:
	{{qemu_test}} v5te

# Armv6 (Versatile AB)
test-qemu-v6:
	{{qemu_test}} v6

# Armv7-R (Versatile AB)
test-qemu-v7r:
	{{qemu_test}} v7r

# Armv7-A (Versatile AB), incl. fpu-d32 on hf targets. Two filters because the
# name must be scoped to versatileab (zynq is also v7a), which splits arm/thumb.
test-qemu-v7a:
	{{qemu_test}} versatileab/armv7a
	{{qemu_test}} versatileab/thumbv7a

# Armv7-A (Xilinx Zynq-A9)
test-qemu-v7a-zynq:
	{{qemu_test}} zynq

# Armv8-R (MPS3-AN536), incl. fpu-d32
test-qemu-v8r:
	{{qemu_test}} mps3-an536/

# Armv8-R EL2 (MPS3-AN536), incl. fpu-d32
test-qemu-v8r-el2:
	{{qemu_test}} el2
