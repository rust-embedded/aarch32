#!/usr/bin/env bash

# Runs a series of sample programs in QEMU and checks that the standard output
# is as expected.

rustup target add armv7r-none-eabi
rustup target add armv7r-none-eabihf
rustup target add armv7a-none-eabi
rustup component add rust-src

FAILURE=0

fail() {
    echo "***************************************************"
    echo "test.sh MISMATCH: Binary $1 for target $2 mismatched"
    echo "***************************************************"
    FAILURE=1
}

mkdir -p ./target

versatile_ab_cargo="--manifest-path examples/versatileab/Cargo.toml"
mps3_an536_cargo="--manifest-path examples/mps3-an536/Cargo.toml"

my_diff() {
    file_a=$1
    file_b=$2
    # - Fix Windows path separators (\\) to look like UNIX ones (/) in the QEMU
    # output
    # - Fix the CRLF line endings in the files on disk, because git adds them to
    # text files.
    diff <(cat $file_a | tr -d '\r') <(cat $file_b | sed 's~\\\\~/~g')
}

# armv7r-none-eabi tests
for bin_path in $(ls examples/versatileab/src/bin/*.rs); do
    filename=${bin_path##*/}
    binary=${filename%.rs}
    cargo run ${versatile_ab_cargo} --target=armv7r-none-eabi --bin $binary | tee ./target/$binary-armv7r-none-eabi.out
    my_diff ./examples/versatileab/reference/$binary-armv7r-none-eabi.out ./target/$binary-armv7r-none-eabi.out || fail $binary "armv7r-none-eabi"
done

# armv7r-none-eabihf tests
for bin_path in $(ls examples/versatileab/src/bin/*.rs); do
    filename=${bin_path##*/}
    binary=${filename%.rs}
    cargo run ${versatile_ab_cargo} --target=armv7r-none-eabihf --bin $binary | tee ./target/$binary-armv7r-none-eabihf.out
    my_diff ./examples/versatileab/reference/$binary-armv7r-none-eabihf.out ./target/$binary-armv7r-none-eabihf.out || fail $binary "armv7r-none-eabihf"
done

# armv7a-none-eabi tests
for bin_path in $(ls examples/versatileab/src/bin/*.rs); do
    filename=${bin_path##*/}
    binary=${filename%.rs}
    cargo run ${versatile_ab_cargo} --target=armv7a-none-eabi --bin $binary | tee ./target/$binary-armv7a-none-eabi.out
    my_diff ./examples/versatileab/reference/$binary-armv7a-none-eabi.out ./target/$binary-armv7a-none-eabi.out || fail $binary "armv7a-none-eabi"
done

# armv7a-none-eabihf tests
for bin_path in $(ls examples/versatileab/src/bin/*.rs); do
    filename=${bin_path##*/}
    binary=${filename%.rs}
    RUSTC_BOOTSTRAP=1 cargo run ${versatile_ab_cargo} --target=armv7a-none-eabihf --bin $binary | tee ./target/$binary-armv7a-none-eabihf.out
    my_diff ./examples/versatileab/reference/$binary-armv7a-none-eabihf.out ./target/$binary-armv7a-none-eabihf.out || fail $binary "armv7a-none-eabihf"
done

# These tests only run on QEMU 9 or higher.
# Ubuntu 24.04 supplies QEMU 8, which doesn't support the machine we have configured for this target
if qemu-system-arm --version | grep "version \(9\|10\)"; then
    # armv8r-none-eabihf tests
    for bin_path in $(ls examples/mps3-an536/src/bin/*.rs); do
        filename=${bin_path##*/}
        binary=${filename%.rs}
        RUSTC_BOOTSTRAP=1 cargo run ${mps3_an536_cargo} --target=armv8r-none-eabihf --bin $binary --features=gic -Zbuild-std=core | tee ./target/$binary-armv8r-none-eabihf.out
        my_diff ./examples/mps3-an536/reference/$binary-armv8r-none-eabihf.out ./target/$binary-armv8r-none-eabihf.out || fail $binary "armv8r-none-eabihf"
    done
    RUSTC_BOOTSTRAP=1 cargo run ${mps3_an536_cargo} --target=armv8r-none-eabihf --bin smp_test --features=gic -Zbuild-std=core -- -smp 2 | tee ./target/smp_test-armv8r-none-eabihf_smp2.out
    my_diff ./examples/mps3-an536/reference/smp_test-armv8r-none-eabihf_smp2.out ./target/smp_test-armv8r-none-eabihf_smp2.out || fail smp_test "armv8r-none-eabihf"
fi

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
