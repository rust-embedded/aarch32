#!/usr/bin/env bash

# Runs a series of sample programs in QEMU and checks that the standard output
# is as expected.

# Set this to 1 to exit on the first error
EXIT_FAST=0

FAILURE=0

fail_build() {
    echo "***************************************************"
    echo "test.sh FAIL: Binary $1 for target $2 returned non-zero"
    echo "***************************************************"
    FAILURE=1
    if [ $EXIT_FAST == "1" ]; then
        exit 1
    fi
}

fail_diff() {
    echo "***************************************************"
    echo "test.sh MISMATCH: Binary $1 for target $2 mismatched"
    echo "(You can re-run with UPDATE_OUT=1 to replace the reference file)"
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
        result=$?
        if [ "${UPDATE_OUT}" == "1" ]; then
            cp $file_b $file_a
        fi
        return $result
    fi
}

directory=$1
shift
target=$1
shift
flags="$*"
echo "Running directory=$directory target=$target flags=$flags"
pushd $directory
cargo build --target=$target $flags || exit 1
for bin_path in src/bin/*.rs; do
    filename=$(basename $bin_path)
    binary=${filename%.rs}
    cargo run --target=$target --bin $binary $flags > ./target/$binary-$target.out || fail_build $binary $target
    my_diff ./reference/$binary-$target.out ./target/$binary-$target.out || fail_diff $binary $target
done
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
