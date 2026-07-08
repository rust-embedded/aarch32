# The rust-embedded/aarch32 Just file
#
# You need to install `just` from https://github.com/casey/just to use
# this file

# If you run with `just --set v 1` then we make cargo run in verbose mode
v := "0"
verbose := if v == "1" { "--verbose" } else { "" }

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
	cd examples/mps3-an536-smp && cargo clean
	rm -rf examples/mps3-an536-smp/target-d32

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

# Builds the Versatile AB examples, building core from source
build-versatileab-tier3 target:
	cd examples/versatileab && cargo build --target={{target}} -Zbuild-std=core {{verbose}}

# Builds the Versatile AB examples, assuming core has been prebuilt
build-versatileab-tier2 target:
	cd examples/versatileab && cargo build --target={{target}} {{verbose}}

# Builds the MPS3-AN536 examples, building core from source
build-mps3-tier3 target:
	cd examples/mps3-an536 && cargo build --target={{target}} -Zbuild-std=core {{verbose}}
	cd examples/mps3-an536-smp && cargo build --target={{target}} -Zbuild-std=core {{verbose}}
	cd examples/mps3-an536-el2 && cargo build --target={{target}} -Zbuild-std=core {{verbose}}

# Builds the MPS3-AN536 examples, assuming core has been prebuilt
build-mps3-tier2 target:
	cd examples/mps3-an536 && cargo build --target={{target}} {{verbose}}
	cd examples/mps3-an536-smp && cargo build --target={{target}} {{verbose}}
	cd examples/mps3-an536-el2 && cargo build --target={{target}} {{verbose}}

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
	cd examples/mps3-an536-smp && cargo fmt {{verbose}}
	cd examples/mps3-an536-el2 && cargo fmt {{verbose}}

# Checks all the code is formatted
fmt-check:
	# The cross-compiled workspace
	cargo fmt --check
	# The host-compiled helper library
	cd arm-targets && cargo fmt --check {{verbose}}
	# The cross-compiled examples	cargo fmt
	cd examples/versatileab && cargo fmt --check {{verbose}}
	cd examples/mps3-an536 && cargo fmt --check {{verbose}}
	cd examples/mps3-an536-smp && cargo fmt --check {{verbose}}
	cd examples/mps3-an536-el2 && cargo fmt --check {{verbose}}

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
	cd examples/mps3-an536-smp && cargo clippy --target=armv8r-none-eabihf {{verbose}}
	cd examples/mps3-an536-el2 && cargo clippy --target=armv8r-none-eabihf {{verbose}}

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
    cargo build --target {{target}} {{verbose}}
    cargo build --target {{target}} --features "serde, defmt, critical-section-multi-core, check-asm" {{verbose}}
    cargo build --target {{target}} --features "serde, defmt, critical-section-single-core, check-asm" {{verbose}}

# Run all the tests
test: test-cargo test-qemu

# Run the unit tests with cargo
test-cargo:
	# The cross-compiled workspace
	cargo test {{verbose}}
	# The host-compiled helper library
	cd arm-targets && cargo test {{verbose}}

# Run the integration tests in QEMU
test-qemu: test-qemu-v4t test-qemu-v5te test-qemu-v6 test-qemu-v7a test-qemu-v7r test-qemu-v8r test-qemu-v8r-smp test-qemu-v8r-el2

test-qemu-v4t:
	#!/bin/bash
	FAIL=0
	./tests.sh examples/versatileab armv4t-none-eabi -Zbuild-std=core {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab thumbv4t-none-eabi -Zbuild-std=core {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab armv4t-none-eabi -Zbuild-std=core {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/versatileab thumbv4t-none-eabi -Zbuild-std=core {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	if [ "${FAIL}" == "1" ]; then exit 1; fi

test-qemu-v5te:
	#!/bin/bash
	FAIL=0
	./tests.sh examples/versatileab armv5te-none-eabi -Zbuild-std=core {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab thumbv5te-none-eabi -Zbuild-std=core {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab armv5te-none-eabi -Zbuild-std=core {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/versatileab thumbv5te-none-eabi -Zbuild-std=core {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	if [ "${FAIL}" == "1" ]; then exit 1; fi

test-qemu-v6:
	#!/bin/bash
	FAIL=0
	./tests.sh examples/versatileab armv6-none-eabi -Zbuild-std=core {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab armv6-none-eabihf -Zbuild-std=core {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab thumbv6-none-eabi -Zbuild-std=core {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab armv6-none-eabi -Zbuild-std=core {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/versatileab armv6-none-eabihf -Zbuild-std=core {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/versatileab thumbv6-none-eabi -Zbuild-std=core {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	if [ "${FAIL}" == "1" ]; then exit 1; fi

test-qemu-v7a:
	#!/bin/bash
	FAIL=0
	./tests.sh examples/versatileab armv7a-none-eabi {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab thumbv7a-none-eabi {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab armv7a-none-eabihf {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab thumbv7a-none-eabihf {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab armv7a-none-eabi {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/versatileab thumbv7a-none-eabi {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/versatileab armv7a-none-eabihf {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/versatileab thumbv7a-none-eabihf {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	RUSTFLAGS=-Ctarget-feature=+d32 ./tests.sh examples/versatileab armv7a-none-eabihf --features=fpu-d32 --target-dir=target-d32 {{verbose}} --release || FAIL=1
	RUSTFLAGS=-Ctarget-feature=+d32 ./tests.sh examples/versatileab thumbv7a-none-eabihf --features=fpu-d32 --target-dir=target-d32 {{verbose}} --release || FAIL=1
	if [ "${FAIL}" == "1" ]; then exit 1; fi

test-qemu-v7r:
	#!/bin/bash
	FAIL=0
	./tests.sh examples/versatileab armv7r-none-eabi {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab thumbv7r-none-eabi {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab armv7r-none-eabihf {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab thumbv7r-none-eabihf {{verbose}} --release || FAIL=1
	./tests.sh examples/versatileab armv7r-none-eabi {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/versatileab thumbv7r-none-eabi {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/versatileab armv7r-none-eabihf {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/versatileab thumbv7r-none-eabihf {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	if [ "${FAIL}" == "1" ]; then exit 1; fi

test-qemu-v8r:
	#!/bin/bash
	FAIL=0
	./tests.sh examples/mps3-an536 armv8r-none-eabihf {{verbose}} --release || FAIL=1
	./tests.sh examples/mps3-an536 thumbv8r-none-eabihf {{verbose}} --release || FAIL=1
	./tests.sh examples/mps3-an536 armv8r-none-eabihf {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/mps3-an536 thumbv8r-none-eabihf {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	RUSTFLAGS=-Ctarget-cpu=cortex-r52 ./tests.sh examples/mps3-an536 armv8r-none-eabihf --features=fpu-d32 --target-dir=target-d32 {{verbose}} --release || FAIL=1
	RUSTFLAGS=-Ctarget-cpu=cortex-r52 ./tests.sh examples/mps3-an536 thumbv8r-none-eabihf --features=fpu-d32 --target-dir=target-d32 {{verbose}} --release || FAIL=1
	if [ "${FAIL}" == "1" ]; then exit 1; fi

test-qemu-v8r-smp:
	#!/bin/bash
	FAIL=0
	./tests.sh examples/mps3-an536-smp armv8r-none-eabihf {{verbose}} --release || FAIL=1
	./tests.sh examples/mps3-an536-smp thumbv8r-none-eabihf {{verbose}} --release || FAIL=1
	./tests.sh examples/mps3-an536-smp armv8r-none-eabihf {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	./tests.sh examples/mps3-an536-smp thumbv8r-none-eabihf {{verbose}} --features=svc-stack-interrupt --release || FAIL=1
	RUSTFLAGS=-Ctarget-cpu=cortex-r52 ./tests.sh examples/mps3-an536-smp armv8r-none-eabihf --features=fpu-d32 --target-dir=target-d32 {{verbose}} --release || FAIL=1
	RUSTFLAGS=-Ctarget-cpu=cortex-r52 ./tests.sh examples/mps3-an536-smp thumbv8r-none-eabihf --features=fpu-d32 --target-dir=target-d32 {{verbose}} --release || FAIL=1
	if [ "${FAIL}" == "1" ]; then exit 1; fi

test-qemu-v8r-el2:
	#!/bin/bash
	FAIL=0
	./tests.sh examples/mps3-an536-el2 armv8r-none-eabihf {{verbose}} --release || FAIL=1
	./tests.sh examples/mps3-an536-el2 thumbv8r-none-eabihf {{verbose}} --release || FAIL=1
	RUSTFLAGS=-Ctarget-cpu=cortex-r52 ./tests.sh examples/mps3-an536-el2 armv8r-none-eabihf --features=fpu-d32 --target-dir=target-d32 {{verbose}} --release || FAIL=1
	RUSTFLAGS=-Ctarget-cpu=cortex-r52 ./tests.sh examples/mps3-an536-el2 thumbv8r-none-eabihf --features=fpu-d32 --target-dir=target-d32 {{verbose}} --release || FAIL=1
	if [ "${FAIL}" == "1" ]; then exit 1; fi
