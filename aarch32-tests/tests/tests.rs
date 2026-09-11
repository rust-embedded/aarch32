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
//!
//! Set `TEST_VERBOSE=1` in your environment to see details of the commands executed.

mod common;

use common::test_utils;
use libtest_mimic::{Arguments, Trial};

/// A build variant of a target: a name suffix plus extra cargo flags.
///
/// All variants within a group are compared against the same snapshot.
struct Variant {
    label: &'static str,
    flags: &'static [&'static str],
    rustflags: &'static [&'static str],
}

/// Build with no features
const PLAIN: Variant = Variant {
    label: "",
    flags: &[],
    rustflags: &[],
};

/// Build with the `svc-stack-interrupt` feature
const SVC: Variant = Variant {
    label: "svc",
    flags: &["--features=svc-stack-interrupt"],
    rustflags: &[],
};

/// Build for a Cortex-R5
const R5_CPU: Variant = Variant {
    label: "r5-cpu",
    flags: &[
        // Tell the runtime we're using the FPU
        "--features=eabi-fpu",
    ],
    rustflags: &[
        // Optimise for Cortex-R5
        "-Ctarget-cpu=cortex-r5",
    ],
};

/// Build for a Cortex-R5 no FPU
const R5_CPU_NOFPU_FEAT: Variant = Variant {
    label: "r5-cpu-nofpu",
    flags: &[],
    rustflags: &[
        // Optimise for Cortex-R5
        "-Ctarget-cpu=cortex-r5",
        // Turn off the FPU that cortex-r5 implies
        "-Ctarget-feature=-fpregs",
    ],
};

/// Build for a Cortex-R5 no double precision
const R5_CPU_NODP_FEAT: Variant = Variant {
    label: "r5-cpu-nodp",
    flags: &[
        // Tell the runtime we're using the FPU
        "--features=eabi-fpu",
    ],
    rustflags: &[
        // Optimise for Cortex-R5
        "-Ctarget-cpu=cortex-r5",
        // Turn off the DP support that cortex-r5 implies
        "-Ctarget-feature=-fp64",
    ],
};

/// Build for a generic CPU with no double precision
const NODP_FEAT: Variant = Variant {
    label: "no-dp",
    flags: &[],
    rustflags: &[
        // Turn off the DP support that is enabled by default
        "-Ctarget-feature=-fp64",
    ],
};

/// Build for a generic Arm CPU with 32 DP registers
const D32_FEAT: Variant = Variant {
    label: "fpu-d32",
    flags: &[
        // Tell the aarch32-rt assembly to stack the high FPU registers,
        "--features=fpu-d32",
    ],
    rustflags: &[
        // Enable usage of all 32 double precision FPU registers
        "-Ctarget-feature=+d32",
    ],
};

/// Build for a Cortex-R52 with 32 DP registers
const R52_CPU: Variant = Variant {
    label: "r52-cpu",
    flags: &[
        // Tell the aarch32-rt assembly to stack the high FPU registers,
        "--features=fpu-d32",
    ],
    rustflags: &[
        // Optimise for Cortex-R52 (which also enables usage of all 32 DP registers)
        "-Ctarget-cpu=cortex-r52",
    ],
};

/// A group of programs to build and test
struct Group {
    name: &'static str,
    example: &'static str,
    targets: &'static [&'static str],
    flags: &'static [&'static str],
    variants: &'static [Variant],
}

const MATRIX: &[Group] = &[
    Group {
        name: "versatileab-legacy",
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
        variants: &[PLAIN, SVC],
    },
    Group {
        name: "versatileab-v7r",
        example: "versatileab",
        targets: &["armv7r-none-eabi", "thumbv7r-none-eabi"],
        flags: &["--release"],
        variants: &[PLAIN, SVC, R5_CPU, R5_CPU_NOFPU_FEAT, R5_CPU_NODP_FEAT],
    },
    Group {
        name: "versatileab-v7r-hf",
        example: "versatileab",
        targets: &["armv7r-none-eabihf", "thumbv7r-none-eabihf"],
        flags: &["--release"],
        variants: &[PLAIN, SVC, R5_CPU, R5_CPU_NODP_FEAT, NODP_FEAT],
    },
    Group {
        name: "versatileab-v7a",
        example: "versatileab",
        targets: &["armv7a-none-eabihf", "thumbv7a-none-eabihf"],
        flags: &["--release"],
        variants: &[PLAIN, SVC, D32_FEAT],
    },
    Group {
        name: "mps3-an536",
        example: "mps3-an536",
        targets: &["armv8r-none-eabihf", "thumbv8r-none-eabihf"],
        flags: &["--release"],
        variants: &[PLAIN, SVC, R52_CPU],
    },
    Group {
        name: "mps3-an536-el2",
        example: "mps3-an536-el2",
        targets: &["armv8r-none-eabihf", "thumbv8r-none-eabihf"],
        flags: &["--release"],
        variants: &[PLAIN, R52_CPU],
    },
    Group {
        name: "xilinx-zynq-a9",
        example: "xilinx-zynq-a9",
        targets: &[
            "armv7a-none-eabi",
            "thumbv7a-none-eabi",
            "armv7a-none-eabihf",
            "thumbv7a-none-eabihf",
        ],
        flags: &["--release"],
        variants: &[PLAIN],
    },
];

fn main() {
    let args = Arguments::from_args();

    let mut tests = Vec::new();
    for group in MATRIX {
        for &target in group.targets {
            for variant in group.variants {
                let dir = test_utils::test_dir(&format!("examples/{}", group.example));
                for bin in test_utils::discover_bins(&dir) {
                    let target = target.to_string();
                    let mut flags: Vec<String> = Vec::new();
                    for flag in group.flags {
                        flags.push(flag.to_string());
                    }
                    for flag in variant.flags {
                        flags.push(flag.to_string());
                    }
                    if !variant.rustflags.is_empty() {
                        // nonstandard build gets special target dir
                        flags.push(format!("--target-dir=target-{}", variant.label));
                    }
                    let mut name = format!("{}/{}/{}", group.name, target, bin);
                    if !variant.label.is_empty() {
                        name.push_str(&format!(" [{}]", variant.label));
                    }
                    tests.push(Trial::test(name, move || {
                        run_target_bin(group.example, &bin, &target, &flags, &variant.rustflags);
                        Ok(())
                    }));
                }
            }
        }
    }

    libtest_mimic::run(&args, tests).exit();
}

fn run_target_bin(example: &str, bin: &str, target: &str, flags: &[String], rustflags: &[&str]) {
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
