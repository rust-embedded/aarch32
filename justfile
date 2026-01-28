# The rust-embedded/aarch32 Just file
#
# You need to install `just` from https://github.com/casey/just to use
# this file

# We only need this for some targets but we set it globally to avoid
# dependencies like proc-macro2 from being rebuilt
export RUSTC_BOOTSTRAP := "1"


# If you run with `just v=1` then we make cargo run in verbose mode
v := "0"
verbose := if v == "1" { "--verbose" } else { "" }

# Our default target. It does everything that you might want to do pre-checkin.
check: build-all build-all-examples fmt-check clippy-examples clippy-targets clippy-host test

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

# Builds our workspace for all targets
build-all: \
	build-arm-targets \
	(build-tier3-no-atomics "armv4t-none-eabi") \
	(build-tier3-no-atomics "thumbv4t-none-eabi") \
	(build-tier3-no-atomics "armv5te-none-eabi") \
	(build-tier3-no-atomics "thumbv5te-none-eabi") \
	(build-tier2 "armv7r-none-eabi") \
	(build-tier3 "thumbv7r-none-eabi") \
	(build-tier2 "armv7r-none-eabihf") \
	(build-tier3 "thumbv7r-none-eabihf") \
	(build-tier2 "armv7a-none-eabi") \
	(build-tier3 "thumbv7a-none-eabi") \
	(build-tier2 "armv7a-none-eabihf") \
	(build-tier3 "thumbv7a-none-eabihf") \
	(build-tier2 "armv8r-none-eabihf") \
	(build-tier3 "thumbv8r-none-eabihf") \

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
	(build-versatileab-tier3 "thumbv7r-none-eabi") \
	(build-versatileab-tier3 "thumbv7r-none-eabihf") \
	(build-versatileab-tier3 "thumbv7a-none-eabi") \
	(build-versatileab-tier3 "thumbv7a-none-eabihf") \
	(build-mps3-tier3 "thumbv8r-none-eabihf") \
	(build-versatileab-tier2 "armv7r-none-eabi") \
	(build-versatileab-tier2 "armv7r-none-eabihf") \
	(build-versatileab-tier2 "armv7a-none-eabi") \
	(build-versatileab-tier2 "armv7a-none-eabihf") \
	(build-mps3-tier2 "armv8r-none-eabihf") \

# Builds the Versatile AB examples, building core from source
build-versatileab-tier3 target:
	cd examples/versatileab && cargo build --target={{target}} -Zbuild-std=core {{verbose}}

# Builds the Versatile AB examples, assuming core has been prebuilt
build-versatileab-tier2 target:
	cd examples/versatileab && cargo build --target={{target}} {{verbose}}

# Builds the MPS3-AN536 examples, building core from source
build-mps3-tier3 target:
	cd examples/mps3-an536 && cargo build --target={{target}} -Zbuild-std=core {{verbose}}

# Builds the MPS3-AN536 examples, assuming core has been prebuilt
build-mps3-tier2 target:
	cd examples/mps3-an536 && cargo build --target={{target}} {{verbose}}

# Formats all the code
fmt:
	# The cross-compiled workspace
	cargo fmt {{verbose}}
	# The host-compiled helper library
	cd arm-targets && cargo fmt {{verbose}}
	# The cross-compiled examples	cargo fmt
	cd examples/versatileab && cargo fmt {{verbose}}
	cd examples/mps3-an536 && cargo fmt {{verbose}}

# Checks all the code is formatted
fmt-check:
	# The cross-compiled workspace
	cargo fmt --check
	# The host-compiled helper library
	cd arm-targets && cargo fmt --check {{verbose}}
	# The cross-compiled examples	cargo fmt
	cd examples/versatileab && cargo fmt --check {{verbose}}
	cd examples/mps3-an536 && cargo fmt --check {{verbose}}

# Checks all the cross-compiled workspace passes the clippy lints
clippy-targets: \
	(clippy-target "armv7r-none-eabi") \
	(clippy-target "armv7r-none-eabihf") \
	(clippy-target "armv7a-none-eabi") \
	(clippy-target "armv7a-none-eabihf") \
	(clippy-target "armv8r-none-eabihf") \

# Checks all the cross-compiled workspace passes the clippy lints
clippy-target target:
	cargo clippy --target={{target}} {{verbose}}

# Checks the examples pass the clippy lints
clippy-examples:
	cd examples/versatileab && cargo clippy --target=armv7r-none-eabi {{verbose}}
	cd examples/mps3-an536 && cargo clippy --target=armv8r-none-eabihf {{verbose}}

# Checks the host code passes the clippy lints
clippy-host:
	# The cross-compiled workspace
	cargo clippy {{verbose}}
	# The host-compiled helper library
	cd arm-targets && cargo clippy {{verbose}}

# Run all the tests
test: test-cargo test-qemu test-smp

# Run the unit tests with cargo
test-cargo:
	# The cross-compiled workspace
	cargo test {{verbose}}
	# The host-compiled helper library
	cd arm-targets && cargo test {{verbose}}

# Run the integration tests in QEMU
test-qemu:
	#!/bin/bash
	FAIL=0
	./tests.sh examples/versatileab armv4t-none-eabi -Zbuild-std=core {{verbose}} || FAIL=1
	./tests.sh examples/versatileab thumbv4t-none-eabi -Zbuild-std=core {{verbose}} || FAIL=1
	./tests.sh examples/versatileab armv5te-none-eabi -Zbuild-std=core {{verbose}} || FAIL=1
	./tests.sh examples/versatileab thumbv5te-none-eabi -Zbuild-std=core {{verbose}} || FAIL=1
	./tests.sh examples/versatileab armv7r-none-eabi {{verbose}} || FAIL=1
	./tests.sh examples/versatileab thumbv7r-none-eabi -Zbuild-std=core {{verbose}} || FAIL=1
	./tests.sh examples/versatileab armv7r-none-eabihf {{verbose}} || FAIL=1
	./tests.sh examples/versatileab thumbv7r-none-eabihf -Zbuild-std=core {{verbose}} || FAIL=1
	./tests.sh examples/versatileab armv7a-none-eabi {{verbose}} || FAIL=1
	./tests.sh examples/versatileab thumbv7a-none-eabi -Zbuild-std=core {{verbose}} || FAIL=1
	./tests.sh examples/versatileab armv7a-none-eabihf {{verbose}} || FAIL=1
	./tests.sh examples/versatileab thumbv7a-none-eabihf -Zbuild-std=core {{verbose}} || FAIL=1
	RUSTFLAGS=-Ctarget-feature=+d32 ./tests.sh examples/versatileab armv7a-none-eabihf --features=fpu-d32 --target-dir=target-d32 {{verbose}} || FAIL=1
	RUSTFLAGS=-Ctarget-feature=+d32 ./tests.sh examples/versatileab thumbv7a-none-eabihf -Zbuild-std=core --features=fpu-d32 --target-dir=target-d32 {{verbose}} || FAIL=1
	./tests.sh examples/mps3-an536 armv8r-none-eabihf {{verbose}} || FAIL=1
	./tests.sh examples/mps3-an536 thumbv8r-none-eabihf -Zbuild-std=core {{verbose}} || FAIL=1
	RUSTFLAGS=-Ctarget-cpu=cortex-r52 ./tests.sh examples/mps3-an536 armv8r-none-eabihf --features=fpu-d32 --target-dir=target-d32 {{verbose}} || FAIL=1
	RUSTFLAGS=-Ctarget-cpu=cortex-r52 ./tests.sh examples/mps3-an536 thumbv8r-none-eabihf -Zbuild-std=core --features=fpu-d32 --target-dir=target-d32 {{verbose}} || FAIL=1
	if [ "${FAIL}" == "1" ]; then exit 1; fi

# Run the special SMP test
#
# You can't run the normal examples with two CPUs because nothing stops the second CPU from running :/. So we have
# a special test for SMP mode on the MPS3-AN536
test-smp:
	cd examples/mps3-an536 && cargo run --target=armv8r-none-eabihf --bin smp_test {{verbose}} -- --smp 2
