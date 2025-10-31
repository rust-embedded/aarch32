#!/usr/bin/env bash

# Runs a series of sample programs in QEMU and checks that the standard output
# is as expected.

# Set this to 1 to exit on the first error
EXIT_FAST=0

FAILURE=0

fail() {
    echo "***************************************************"
    echo "test.sh MISMATCH: Binary $1 for target $2 mismatched"
    echo "***************************************************"
    FAILURE=1
    if [ $EXIT_FAST == "1" ]; then
        exit 1
    fi
}

mkdir -p ./target

my_diff() {
    file_a=$1
    file_b=$2
    # - Fix Windows path separators (\\) to look like UNIX ones (/) in the QEMU
    # output
    # - Fix the CRLF line endings in the files on disk, because git adds them to
    # text files.
    if [ ! -f $1 ]; then
        echo "File $1 is missing?!"
        return 1
    elif [ ! -f $1 ]; then
        echo "File $2 is missing?!"
        return 1
    else
        diff <(cat $file_a | tr -d '\r') <(cat $file_b | sed 's~\\\\~/~g')
    fi
}

run_tests() {
    directory=$1
    target="$2"
    flags=$3
    echo "Running directory=$directory target=$target flags=$flags"
    pushd $directory
    cargo build --target=$target $flags || exit 1
    for bin_path in src/bin/*.rs; do
        filename=$(basename $bin_path)
        binary=${filename%.rs}
        cargo run --target=$target --bin $binary $flags > ./target/$binary-$target.out
        my_diff ./reference/$binary-$target.out ./target/$binary-$target.out || fail $binary $target
    done
    popd
}

run_tests examples/versatileab armv7r-none-eabi ""
run_tests examples/versatileab armv7r-none-eabihf ""
run_tests examples/versatileab armv7a-none-eabi ""
run_tests examples/versatileab armv7a-none-eabihf ""
RUSTFLAGS="-Ctarget-feature=+d32" run_tests examples/versatileab armv7a-none-eabihf "--features=fpu-d32"
run_tests examples/versatileab armv5te-none-eabi "-Zbuild-std=core"
run_tests examples/versatileab armv4t-none-eabi "-Zbuild-std=core"
run_tests examples/versatileab thumbv5te-none-eabi "-Zbuild-std=core"
run_tests examples/versatileab thumbv4t-none-eabi "-Zbuild-std=core"
run_tests examples/mps3-an536 armv8r-none-eabihf ""
RUSTFLAGS="-Ctarget-cpu=cortex-r52" run_tests examples/mps3-an536 armv8r-none-eabihf "--features=fpu-d32"

# Special case the SMP test. You can't run the normal examples with two CPUs because nothing stops the second CPU from running :/
pushd examples/mps3-an536
cargo run --target=armv8r-none-eabihf --bin smp_test -- --smp 2 > ./target/smp_test-armv8r-none-eabihf_smp2.out
my_diff ./reference/smp_test-armv8r-none-eabihf_smp2.out ./target/smp_test-armv8r-none-eabihf_smp2.out || fail smp_test armv8r-none-eabihf
popd

if [ "$FAILURE" == "1" ]; then
    echo "***************************************************"
    echo "test.sh: Output comparison failed!"
    echo "***************************************************"
    exit 1
else
    echo "***************************************************"
    echo "test.sh: Everything matches :)"
    echo "***************************************************"
fi
