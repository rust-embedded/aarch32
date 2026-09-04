//! QEMU snapshot tests, one per (example, target, build-variant).
//!
//! We use a custom `libtest-mimic` harness so each target shows up as its own
//! line in `cargo test`:
//!
//! ```text
//! versatileab/armv4t-none-eabi   ... ok
//! versatileab/armv5te-none-eabi  ... ok
//! ```
//!
//! Because there is no external driver, the per-target build recipe (tier3
//! `-Zbuild-std=core`, the `svc-stack-interrupt`/`fpu-d32` variants, and their
//! rustflags) lives in `MATRIX` below rather than in the justfile. Each trial
//! runs the example's bins in QEMU and snapshots stdout; feature variants assert
//! against the same `<bin>-<target>` snapshot as the plain build.
//!
//! Filter like any test binary: `cargo test -p aarch32-tests -- armv7a`.

mod common;

use common::test_utils;
use libtest_mimic::{Arguments, Trial};

/// A build variant of a target: a name suffix plus extra cargo flags. All
/// variants of a target compare against the same snapshot.
struct Variant {
    label: &'static str,
    extra_flags: &'static [&'static str],
}

const PLAIN: Variant = Variant {
    label: "",
    extra_flags: &[],
};
const SVC: Variant = Variant {
    label: "svc",
    extra_flags: &["--features=svc-stack-interrupt"],
};
const FPU: Variant = Variant {
    label: "fpu-d32",
    extra_flags: &[],
};

struct Group {
    example: &'static str,
    targets: &'static [&'static str],
    flags: &'static [&'static str],
    rustflags: Option<&'static str>,
    variants: &'static [Variant],
}

const MATRIX: &[Group] = &[
    Group {
        example: "versatileab",
        targets: &[
            "armv4t-none-eabi",
            "thumbv4t-none-eabi",
            "armv5te-none-eabi",
            "thumbv5te-none-eabi",
            "armv6-none-eabi",
            "armv6-none-eabihf",
            "thumbv6-none-eabi",
        ],
        flags: &["--release", "-Zbuild-std=core"],
        rustflags: None,
        variants: &[PLAIN, SVC],
    },
    Group {
        example: "versatileab",
        targets: &[
            "armv7r-none-eabi",
            "thumbv7r-none-eabi",
            "armv7r-none-eabihf",
            "thumbv7r-none-eabihf",
            "armv7a-none-eabi",
            "thumbv7a-none-eabi",
            "armv7a-none-eabihf",
            "thumbv7a-none-eabihf",
        ],
        flags: &["--release"],
        rustflags: None,
        variants: &[PLAIN, SVC],
    },
    Group {
        example: "versatileab",
        targets: &["armv7a-none-eabihf", "thumbv7a-none-eabihf"],
        flags: &["--release", "--features=fpu-d32", "--target-dir=target-d32"],
        rustflags: Some("-Ctarget-feature=+d32"),
        variants: &[FPU],
    },
    Group {
        example: "mps3-an536",
        targets: &["armv8r-none-eabihf", "thumbv8r-none-eabihf"],
        flags: &["--release"],
        rustflags: None,
        variants: &[PLAIN, SVC],
    },
    Group {
        example: "mps3-an536",
        targets: &["armv8r-none-eabihf", "thumbv8r-none-eabihf"],
        flags: &["--release", "--features=fpu-d32", "--target-dir=target-d32"],
        rustflags: Some("-Ctarget-cpu=cortex-r52"),
        variants: &[FPU],
    },
    Group {
        example: "mps3-an536-el2",
        targets: &["armv8r-none-eabihf", "thumbv8r-none-eabihf"],
        flags: &["--release"],
        rustflags: None,
        variants: &[PLAIN],
    },
    Group {
        example: "mps3-an536-el2",
        targets: &["armv8r-none-eabihf", "thumbv8r-none-eabihf"],
        flags: &["--release", "--features=fpu-d32", "--target-dir=target-d32"],
        rustflags: Some("-Ctarget-cpu=cortex-r52"),
        variants: &[FPU],
    },
    Group {
        example: "xilinx-zynq-a9",
        targets: &[
            "armv7a-none-eabi",
            "thumbv7a-none-eabi",
            "armv7a-none-eabihf",
            "thumbv7a-none-eabihf",
        ],
        flags: &["--release"],
        rustflags: None,
        variants: &[PLAIN],
    },
    Group {
        example: "xilinx-zynq-a9",
        targets: &[
            "armv7a-none-eabi",
            "thumbv7a-none-eabi",
            "armv7a-none-eabihf",
            "thumbv7a-none-eabihf",
        ],
        flags: &["--release", "--features=fpu-d32", "--target-dir=target-d32"],
        rustflags: Some("-Ctarget-cpu=cortex-a9"),
        variants: &[FPU],
    },
];

fn main() {
    let args = Arguments::from_args();

    let mut tests = Vec::new();
    for group in MATRIX {
        for &target in group.targets {
            for variant in group.variants {
                let example = group.example;
                let rustflags = group.rustflags;
                let dir = test_utils::test_dir(&format!("examples/{example}"));
                for bin in test_utils::discover_bins(&dir) {
                    let target = target.to_string();
                    let flags: Vec<&'static str> = group
                        .flags
                        .iter()
                        .chain(variant.extra_flags)
                        .copied()
                        .collect();
                    let mut name = format!("{}/{}/{}", group.example, target, bin);
                    if !variant.label.is_empty() {
                        name.push_str(&format!(" [{}]", variant.label));
                    }
                    tests.push(Trial::test(name, move || {
                        run_target_bin(example, &bin, &target, &flags, rustflags);
                        Ok(())
                    }));
                }
            }
        }
    }

    libtest_mimic::run(&args, tests).exit();
}

fn run_target_bin(example: &str, bin: &str, target: &str, flags: &[&str], rustflags: Option<&str>) {
    let dir = test_utils::test_dir(&format!("examples/{example}"));

    // Per-example folder: snapshots/<example>/<bin>-<target>.snap
    let mut settings = insta::Settings::clone_current();
    settings.add_filter(r"\r", "");
    settings.add_filter(r"\\\\", "/");
    settings.set_snapshot_path(format!("snapshots/{example}"));
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    let stdout = test_utils::run_bin(&dir, &bin, target, flags, rustflags);
    insta::assert_snapshot!(format!("{bin}-{target}"), stdout);
}
