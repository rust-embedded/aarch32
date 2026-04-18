//! Useful cfg helpers for when you are building Arm code
//!
//! Hopefully Rust will stabilise these kinds of target features in the
//! future, and this won't be required. But until this, arm-targets is here to
//! help you conditionally compile your code based on the specific Arm
//! platform you are compiling for.
//!
//! In your application, do something like this:
//!
//! ```console
//! $ cargo add --build arm-targets
//! $ cat > build.rs << EOF
//! fn main() {
//!     arm_targets::process();
//! }
//! EOF
//! ```
//!
//! This will then let you write application code like:
//!
//! ```rust
//! #[cfg(arm_architecture = "v7-m")]
//! fn only_for_cortex_m3() { }
//!
//! #[cfg(arm_isa = "a32")]
//! fn can_use_arm_32bit_asm_here() { }
//! ```
//!
//! Without this crate, you are limited to `cfg(target_arch = "arm")`, which
//! isn't all that useful given how many 'Arm' targets there are.
//!
//! To see a full list of the features created by this crate, run the CLI tool:
//!
//! ```console
//! $ cargo install arm-targets
//! $ arm-targets
//! cargo:rustc-check-cfg=cfg(arm_isa, values("a64", "a32", "t32"))
//! cargo:rustc-check-cfg=cfg(arm_architecture, values("v4t", "v5te", "v6-m", "v7-m", "v7e-m", "v8-m.base", "v8-m.main", "v7-r", "v8-r", "v7-a", "v8-a"))
//! cargo:rustc-check-cfg=cfg(arm_profile, values("a", "r", "m", "legacy"))
//! cargo:rustc-check-cfg=cfg(arm_abi, values("eabi", "eabihf"))
//! ```

use std::{collections::HashSet, env};

#[derive(Default, Debug)]
pub struct TargetInfo {
    isa: Option<Isa>,
    arch: Option<Arch>,
    abi: Option<Abi>,
}

impl TargetInfo {
    /// Parses the target information from the Cargo environment.
    pub fn from_cargo_env() -> Self {
        Self {
            isa: Isa::from_cargo_env(),
            arch: Arch::from_cargo_env(),
            abi: Abi::from_cargo_env(),
        }
    }

    /// Decode a target string
    pub fn get(target: &str) -> Self {
        Self {
            isa: Isa::get(target),
            arch: Arch::get(target),
            abi: Abi::get(target),
        }
    }

    /// Get the Arm Instruction Set Architecture of the target
    pub fn isa(&self) -> Option<Isa> {
        self.isa
    }

    /// Get the Arm Architecture version of the target
    pub fn arch(&self) -> Option<Arch> {
        self.arch
    }

    /// Get the Arm Architecture Profile of the target
    pub fn profile(&self) -> Option<Profile> {
        self.arch.map(|arch| arch.profile())
    }

    /// Get the ABI of the target
    pub fn abi(&self) -> Option<Abi> {
        self.abi
    }

    fn dump(&self) {
        if let Some(isa) = self.isa() {
            println!(r#"cargo:rustc-cfg=arm_isa="{}""#, isa);
        }
        println!(
            r#"cargo:rustc-check-cfg=cfg(arm_isa, values({}))"#,
            Isa::values()
        );

        if let Some(arch) = self.arch() {
            println!(r#"cargo:rustc-cfg=arm_architecture="{}""#, arch);
        }
        println!(
            r#"cargo:rustc-check-cfg=cfg(arm_architecture, values({}))"#,
            Arch::values()
        );

        if let Some(profile) = self.profile() {
            println!(r#"cargo:rustc-cfg=arm_profile="{}""#, profile);
        }
        println!(
            r#"cargo:rustc-check-cfg=cfg(arm_profile, values({}))"#,
            Profile::values()
        );

        if let Some(abi) = self.abi() {
            println!(r#"cargo:rustc-cfg=arm_abi="{}""#, abi);
        }
        println!(
            r#"cargo:rustc-check-cfg=cfg(arm_abi, values({}))"#,
            Abi::values()
        );
    }
}

/// Process the `${TARGET}` environment variable, and emit cargo configuration
/// to standard out.
///
/// You probably want to call this from your build script.
///
/// When `${TARGET}` isn't known to this library, it falls back to using
/// `CARGO_CFG_TARGET_*` variables. These are only really useful on nightly Rust
/// currently, because the ones that give us details about the architecture are
/// not yet stable.
pub fn process() -> TargetInfo {
    let target = std::env::var("TARGET").expect("build script TARGET variable");
    let target_info_from_target = TargetInfo::get(&target);

    let target_info_from_cargo_env = TargetInfo::from_cargo_env();

    let target_info = TargetInfo {
        isa: target_info_from_target
            .isa()
            .or(target_info_from_cargo_env.isa()),
        arch: target_info_from_target
            .arch()
            .or(target_info_from_cargo_env.arch()),
        abi: target_info_from_target
            .abi()
            .or(target_info_from_cargo_env.abi()),
    };

    target_info.dump();

    target_info
}

/// Process a given target string, and emit cargo configuration to standard out.
///
/// Note that this function does not take `CARGO_CFG_TARGET_*` variables into
/// account so you probably do not want to call this from your build script.
#[deprecated(
    since = "0.4.2",
    note = "This function does not take `CARGO_CFG_TARGET_*` variables into account."
)]
pub fn process_target(target: &str) -> TargetInfo {
    let target_info = TargetInfo::get(target);
    target_info.dump();
    target_info
}

/// The Arm Instruction Set
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Isa {
    /// A64 instructions are executed by Arm processors in AArch64 mode
    A64,
    /// A32 instructions are executed by Arm processors in AArch32 Arm mode
    A32,
    /// T32 instructions are executed by Arm processors in AArch32 Thumb mode
    T32,
}

impl Isa {
    /// Parses the ISA from the Cargo environment.
    pub fn from_cargo_env() -> Option<Self> {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").ok()?;
        let features = env::var("CARGO_CFG_TARGET_FEATURE").ok()?;
        let features: HashSet<&str> = features.split(",").collect();

        match arch.as_str() {
            "arm" if features.contains(&"thumb-mode") => Some(Self::T32),
            "arm" => Some(Self::A32),
            "aarch64" => Some(Self::A64),
            _ => None,
        }
    }

    /// Decode a target string
    pub fn get(target: &str) -> Option<Isa> {
        if target.starts_with("arm") {
            Some(Isa::A32)
        } else if target.starts_with("thumb") {
            Some(Isa::T32)
        } else if target.starts_with("aarch64") {
            Some(Isa::A64)
        } else {
            None
        }
    }

    /// Get a comma-separated list of values, suitable for cfg-check
    pub fn values() -> String {
        let string_versions: Vec<String> = [Isa::A64, Isa::A32, Isa::T32]
            .iter()
            .map(|i| format!(r#""{i}""#))
            .collect();
        string_versions.join(", ")
    }
}

impl core::fmt::Display for Isa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Isa::A64 => "a64",
                Isa::A32 => "a32",
                Isa::T32 => "t32",
            }
        )
    }
}

/// The Arm Architecture
///
/// As defined by a particular revision of the Arm Architecture Reference Manual (ARM).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Arch {
    /// Arm Architecture version 4, with Thumb support (e.g. ARM7TDMI)
    Armv4T,
    /// Arm Architecture version 5, with Thumb support and Enhanced DSP Instructions (e.g. ARM926EJ-S)
    Armv5TE,
    /// Arm Architecture version 6 (e.g. ARM1176JZF-S)
    Armv6,
    /// Armv6-M (e.g. Cortex-M0+)
    Armv6M,
    /// Armv7-M (e.g. Cortex-M3)
    Armv7M,
    /// Armv7E-M (e.g. Cortex-M4)
    Armv7EM,
    /// Armv8-M Baseline (e.g. Cortex-M23)
    Armv8MBase,
    /// Armv8-M with Mainline extensions (e.g. Cortex-M33)
    Armv8MMain,
    /// Armv7-R (e.g. Cortex-R5)
    Armv7R,
    /// Armv8-R (e.g. Cortex-R52)
    Armv8R,
    /// Armv7-A (e.g. Cortex-A8)
    Armv7A,
    /// Armv8-A (e.g. Cortex-A53)
    Armv8A,
}

impl Arch {
    /// Parses the architecture from the Cargo environment.
    pub fn from_cargo_env() -> Option<Self> {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").ok()?;
        let features = env::var("CARGO_CFG_TARGET_FEATURE").ok()?;
        let features: HashSet<&str> = features.split(",").collect();

        if (arch == "arm" && features.contains(&"v8")) || arch == "aarch64" {
            if features.contains(&"mclass") {
                if features.contains(&"thumb2") {
                    return Some(Self::Armv8MMain);
                }

                return Some(Self::Armv8MBase);
            }

            if features.contains(&"rclass") {
                return Some(Self::Armv8R);
            }

            Some(Self::Armv8A)
        } else if arch == "arm" && features.contains(&"v7") {
            if features.contains(&"aclass") {
                return Some(Self::Armv7A);
            }

            if features.contains(&"mclass") {
                if features.contains(&"dsp") {
                    return Some(Self::Armv7EM);
                }

                return Some(Self::Armv7M);
            }

            if features.contains(&"rclass") {
                return Some(Self::Armv7R);
            }

            None
        } else if arch == "arm" && features.contains(&"v6") {
            if features.contains(&"mclass") {
                return Some(Self::Armv6M);
            }

            Some(Self::Armv6)
        } else if arch == "arm" && features.contains(&"v5te") {
            Some(Self::Armv5TE)
        } else if arch == "arm" {
            Some(Self::Armv4T)
        } else {
            None
        }
    }

    /// Decode a target string
    pub fn get(target: &str) -> Option<Arch> {
        if target.starts_with("armv4t-") || target.starts_with("thumbv4t-") {
            Some(Arch::Armv4T)
        } else if target.starts_with("armv5te-") || target.starts_with("thumbv5te-") {
            Some(Arch::Armv5TE)
        } else if target.starts_with("thumbv6m-") {
            Some(Arch::Armv6M)
        } else if target.starts_with("thumbv7m-") {
            Some(Arch::Armv7M)
        } else if target.starts_with("thumbv7em-") {
            Some(Arch::Armv7EM)
        } else if target.starts_with("thumbv8m.base-") {
            Some(Arch::Armv8MBase)
        } else if target.starts_with("thumbv8m.main-") {
            Some(Arch::Armv8MMain)
        } else if target.starts_with("armv7r-")
            || target.starts_with("armebv7r-")
            || target.starts_with("thumbv7r-")
        {
            Some(Arch::Armv7R)
        } else if target.starts_with("armv8r-")
            || target.starts_with("aarch64r82-")
            || target.starts_with("aarch64v8r-")
            || target.starts_with("thumbv8r-")
        {
            Some(Arch::Armv8R)
        } else if target.starts_with("armv7a-") || target.starts_with("thumbv7a-") {
            Some(Arch::Armv7A)
        } else if target.starts_with("aarch64-") || target.starts_with("aarch64be-") {
            Some(Arch::Armv8A)
        } else if target.starts_with("arm-")
            || target.starts_with("armv6-")
            || target.starts_with("thumbv6-")
        {
            // If not specified, assume ARMv6.
            Some(Arch::Armv6)
        } else {
            None
        }
    }

    /// Get the Arm Architecture Profile
    pub fn profile(&self) -> Profile {
        match self {
            Arch::Armv6M | Arch::Armv7M | Arch::Armv7EM | Arch::Armv8MBase | Arch::Armv8MMain => {
                Profile::M
            }
            Arch::Armv4T | Arch::Armv5TE | Arch::Armv6 => Profile::Legacy,
            Arch::Armv7R | Arch::Armv8R => Profile::R,
            Arch::Armv7A | Arch::Armv8A => Profile::A,
        }
    }

    /// Get a comma-separated list of values, suitable for cfg-check
    pub fn values() -> String {
        let string_versions: Vec<String> = [
            Arch::Armv4T,
            Arch::Armv5TE,
            Arch::Armv6,
            Arch::Armv6M,
            Arch::Armv7M,
            Arch::Armv7EM,
            Arch::Armv8MBase,
            Arch::Armv8MMain,
            Arch::Armv7R,
            Arch::Armv8R,
            Arch::Armv7A,
            Arch::Armv8A,
        ]
        .iter()
        .map(|i| format!(r#""{i}""#))
        .collect();
        string_versions.join(", ")
    }
}

impl core::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Arch::Armv4T => "v4t",
                Arch::Armv5TE => "v5te",
                Arch::Armv6 => "v6",
                Arch::Armv6M => "v6-m",
                Arch::Armv7M => "v7-m",
                Arch::Armv7EM => "v7e-m",
                Arch::Armv7R => "v7-r",
                Arch::Armv8R => "v8-r",
                Arch::Armv8MBase => "v8-m.base",
                Arch::Armv8MMain => "v8-m.main",
                Arch::Armv7A => "v7-a",
                Arch::Armv8A => "v8-a",
            }
        )
    }
}

/// The Arm Architecture Profile.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Profile {
    /// Microcontrollers
    M,
    /// Real-Time
    R,
    /// Applications
    A,
    /// Legacy
    Legacy,
}

impl Profile {
    /// Parses the profile from the Cargo environment.
    pub fn from_cargo_env() -> Option<Self> {
        let arch = Arch::from_cargo_env()?;
        Some(arch.profile())
    }

    /// Decode a target string
    pub fn get(target: &str) -> Option<Profile> {
        let arch = Arch::get(target)?;
        Some(arch.profile())
    }

    /// Get a comma-separated list of values, suitable for cfg-check
    pub fn values() -> String {
        let string_versions: Vec<String> = [Profile::A, Profile::R, Profile::M, Profile::Legacy]
            .iter()
            .map(|i| format!(r#""{i}""#))
            .collect();
        string_versions.join(", ")
    }
}

impl core::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Profile::M => "m",
                Profile::R => "r",
                Profile::A => "a",
                Profile::Legacy => "legacy",
            }
        )
    }
}

/// The ABI
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Abi {
    /// Arm Embedded ABI
    Eabi,
    /// Arm Embedded ABI with Hard Float
    EabiHf,
}

impl Abi {
    /// Parses the ABI from the Cargo environment.
    pub fn from_cargo_env() -> Option<Self> {
        let abi = env::var("CARGO_CFG_TARGET_ABI").ok()?;

        match abi.as_str() {
            "eabi" => Some(Self::Eabi),
            "eabihf" => Some(Self::EabiHf),
            _ => None,
        }
    }

    /// Decode a target string
    pub fn get(target: &str) -> Option<Abi> {
        let _ = Arch::get(target)?;
        if target.ends_with("eabi") {
            Some(Abi::Eabi)
        } else if target.ends_with("eabihf") {
            Some(Abi::EabiHf)
        } else {
            None
        }
    }

    /// Get a comma-separated list of values, suitable for cfg-check
    pub fn values() -> String {
        let string_versions: Vec<String> = [Abi::Eabi, Abi::EabiHf]
            .iter()
            .map(|i| format!(r#""{i}""#))
            .collect();
        string_versions.join(", ")
    }
}

impl core::fmt::Display for Abi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Abi::Eabi => "eabi",
                Abi::EabiHf => "eabihf",
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn with_target(target: &str, f: impl FnOnce()) {
        temp_env::with_vars(
            [
                ("TARGET", Some(target)),
                ("CARGO_CFG_TARGET_ARCH", None),
                ("CARGO_CFG_TARGET_ABI", None),
                ("CARGO_CFG_TARGET_FEATURE", None),
            ],
            f,
        )
    }

    fn with_cargo_env(target_arch: &str, target_feature: &str, target_abi: &str, f: impl FnOnce()) {
        temp_env::with_vars(
            [
                ("TARGET", Some("ensure-fallback-to-cargo-env-is-used")),
                ("CARGO_CFG_TARGET_ARCH", Some(target_arch)),
                ("CARGO_CFG_TARGET_FEATURE", Some(target_feature)),
                ("CARGO_CFG_TARGET_ABI", Some(target_abi)),
            ],
            f,
        )
    }

    fn with_target_and_cargo_env(
        target: &str,
        target_arch: &str,
        target_feature: &str,
        target_abi: &str,
        f: impl FnOnce(),
    ) {
        temp_env::with_vars(
            [
                ("TARGET", Some(target)),
                ("CARGO_CFG_TARGET_ARCH", Some(target_arch)),
                ("CARGO_CFG_TARGET_FEATURE", Some(target_feature)),
                ("CARGO_CFG_TARGET_ABI", Some(target_abi)),
            ],
            f,
        )
    }

    #[test]
    fn armv4t_none_eabi() {
        let target = "armv4t-none-eabi";
        let target_arch = "arm";
        let target_feature = "";
        let target_abi = "eabi";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A32));
            assert_eq!(target_info.arch(), Some(Arch::Armv4T));
            assert_eq!(target_info.profile(), Some(Profile::Legacy));
            assert_eq!(target_info.abi(), Some(Abi::Eabi));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn armv5te_none_eabi() {
        let target = "armv5te-none-eabi";
        let target_arch = "arm";
        let target_feature = "v5te";
        let target_abi = "eabi";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A32));
            assert_eq!(target_info.arch(), Some(Arch::Armv5TE));
            assert_eq!(target_info.profile(), Some(Profile::Legacy));
            assert_eq!(target_info.abi(), Some(Abi::Eabi));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn armv6_none_eabi() {
        let target = "armv6-none-eabi";
        let target_arch = "arm";
        let target_feature = "v6";
        let target_abi = "eabi";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A32));
            assert_eq!(target_info.arch(), Some(Arch::Armv6));
            assert_eq!(target_info.profile(), Some(Profile::Legacy));
            assert_eq!(target_info.abi(), Some(Abi::Eabi));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn armv6_none_eabihf() {
        let target = "armv6-none-eabihf";
        let target_arch = "arm";
        let target_feature = "v6";
        let target_abi = "eabihf";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A32));
            assert_eq!(target_info.arch(), Some(Arch::Armv6));
            assert_eq!(target_info.profile(), Some(Profile::Legacy));
            assert_eq!(target_info.abi(), Some(Abi::EabiHf));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn arm_unknown_linux_gnueabi() {
        let target = "arm-unknown-linux-gnueabi";
        let target_arch = "arm";
        let target_feature = "v6";
        let target_abi = "eabi";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A32));
            assert_eq!(target_info.arch(), Some(Arch::Armv6));
            assert_eq!(target_info.profile(), Some(Profile::Legacy));
            assert_eq!(target_info.abi(), Some(Abi::Eabi));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn thumbv6m_none_eabi() {
        let target = "thumbv6m-none-eabi";
        let target_arch = "arm";
        let target_feature = "thumb-mode,mclass,v6";
        let target_abi = "eabi";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::T32));
            assert_eq!(target_info.arch(), Some(Arch::Armv6M));
            assert_eq!(target_info.profile(), Some(Profile::M));
            assert_eq!(target_info.abi(), Some(Abi::Eabi));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn thumbv7m_none_eabi() {
        let target = "thumbv7m-none-eabi";
        let target_arch = "arm";
        let target_feature = "thumb-mode,mclass,v7";
        let target_abi = "eabi";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::T32));
            assert_eq!(target_info.arch(), Some(Arch::Armv7M));
            assert_eq!(target_info.profile(), Some(Profile::M));
            assert_eq!(target_info.abi(), Some(Abi::Eabi));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn thumbv7em_nuttx_eabihf() {
        let target = "thumbv7em-nuttx-eabihf";
        let target_arch = "arm";
        let target_feature = "thumb-mode,mclass,v7,dsp";
        let target_abi = "eabihf";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::T32));
            assert_eq!(target_info.arch(), Some(Arch::Armv7EM));
            assert_eq!(target_info.profile(), Some(Profile::M));
            assert_eq!(target_info.abi(), Some(Abi::EabiHf));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn thumbv8m_base_none_eabi() {
        let target = "thumbv8m.base-none-eabi";
        let target_arch = "arm";
        let target_feature = "thumb-mode,mclass,v8";
        let target_abi = "eabi";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::T32));
            assert_eq!(target_info.arch(), Some(Arch::Armv8MBase));
            assert_eq!(target_info.profile(), Some(Profile::M));
            assert_eq!(target_info.abi(), Some(Abi::Eabi));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn thumbv8m_main_none_eabihf() {
        let target = "thumbv8m.main-none-eabihf";
        let target_arch = "arm";
        let target_feature = "thumb-mode,thumb2,mclass,v8";
        let target_abi = "eabihf";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::T32));
            assert_eq!(target_info.arch(), Some(Arch::Armv8MMain));
            assert_eq!(target_info.profile(), Some(Profile::M));
            assert_eq!(target_info.abi(), Some(Abi::EabiHf));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn armv7r_none_eabi() {
        let target = "armv7r-none-eabi";
        let target_arch = "arm";
        let target_feature = "rclass,v7";
        let target_abi = "eabi";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A32));
            assert_eq!(target_info.arch(), Some(Arch::Armv7R));
            assert_eq!(target_info.profile(), Some(Profile::R));
            assert_eq!(target_info.abi(), Some(Abi::Eabi));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn armv8r_none_eabihf() {
        let target = "armv8r-none-eabihf";
        let target_arch = "arm";
        let target_feature = "rclass,v8";
        let target_abi = "eabihf";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A32));
            assert_eq!(target_info.arch(), Some(Arch::Armv8R));
            assert_eq!(target_info.profile(), Some(Profile::R));
            assert_eq!(target_info.abi(), Some(Abi::EabiHf));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn thumbv8r_none_eabihf() {
        let target = "thumbv8r-none-eabihf";
        let target_arch = "arm";
        let target_feature = "thumb-mode,rclass,v8";
        let target_abi = "eabihf";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::T32));
            assert_eq!(target_info.arch(), Some(Arch::Armv8R));
            assert_eq!(target_info.profile(), Some(Profile::R));
            assert_eq!(target_info.abi(), Some(Abi::EabiHf));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn armv7a_none_eabi() {
        let target = "armv7a-none-eabi";
        let target_arch = "arm";
        let target_feature = "aclass,v7";
        let target_abi = "eabi";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A32));
            assert_eq!(target_info.arch(), Some(Arch::Armv7A));
            assert_eq!(target_info.profile(), Some(Profile::A));
            assert_eq!(target_info.abi(), Some(Abi::Eabi));
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn aarch64_none() {
        let target = "aarch64-unknown-none";
        let target_arch = "aarch64";
        let target_feature = "aclass,v8";
        let target_abi = "";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A64));
            assert_eq!(target_info.arch(), Some(Arch::Armv8A));
            assert_eq!(target_info.profile(), Some(Profile::A));
            assert_eq!(target_info.abi(), None);
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn aarch64v8r_none() {
        let target = "aarch64v8r-unknown-none";
        let target_arch = "aarch64";
        let target_feature = "rclass,v8";
        let target_abi = "";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A64));
            assert_eq!(target_info.arch(), Some(Arch::Armv8R));
            assert_eq!(target_info.profile(), Some(Profile::R));
            assert_eq!(target_info.abi(), None);
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }

    #[test]
    fn aarch64r82_none() {
        let target = "aarch64r82-unknown-none";
        let target_arch = "aarch64";
        let target_feature = "rclass,v8";
        let target_abi = "";

        let test = || {
            let target_info = process();
            assert_eq!(target_info.isa(), Some(Isa::A64));
            assert_eq!(target_info.arch(), Some(Arch::Armv8R));
            assert_eq!(target_info.profile(), Some(Profile::R));
            assert_eq!(target_info.abi(), None);
        };

        with_target(target, test);
        with_cargo_env(target_arch, target_feature, target_abi, test);
        with_target_and_cargo_env(target, target_arch, target_feature, target_abi, test);
    }
}
