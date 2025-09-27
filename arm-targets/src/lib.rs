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
//! #[cfg(arm_architecture = "armv7m")]
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
//! ```

#[derive(Default)]
pub struct TargetInfo {
    isa: Option<Isa>,
    arch: Option<Arch>,
    profile: Option<Profile>,
}

impl TargetInfo {
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
        self.profile
    }
}

/// Process the ${TARGET} environment variable, and emit cargo configuration to
/// standard out.
pub fn process() -> TargetInfo {
    let target = std::env::var("TARGET").expect("build script TARGET variable");
    process_target(&target)
}

/// Process a given target string, and emit cargo configuration to standard out.
pub fn process_target(target: &str) -> TargetInfo {
    let mut target_info = TargetInfo::default();
    if let Some(isa) = Isa::get(target) {
        println!(r#"cargo:rustc-cfg=arm_isa="{}""#, isa);
        target_info.isa = Some(isa);
    }
    println!(
        r#"cargo:rustc-check-cfg=cfg(arm_isa, values({}))"#,
        Isa::values()
    );

    if let Some(arch) = Arch::get(target) {
        println!(r#"cargo:rustc-cfg=arm_architecture="{}""#, arch);
        target_info.arch = Some(arch);
    }
    println!(
        r#"cargo:rustc-check-cfg=cfg(arm_architecture, values({}))"#,
        Arch::values()
    );

    if let Some(profile) = Profile::get(target) {
        println!(r#"cargo:rustc-cfg=arm_profile="{}""#, profile);
        target_info.profile = Some(profile);
    }
    println!(
        r#"cargo:rustc-check-cfg=cfg(arm_profile, values({}))"#,
        Profile::values()
    );
    target_info
}

/// The Arm Instruction Set
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Isa {
    /// A64 instructions are executed by Arm processors in Aarch64 mode
    A64,
    /// A32 instructions are executed by Arm processors in Aarch32 Arm mode
    A32,
    /// T32 instructions are executed by Arm processors in Aarch32 Thumb mode
    T32,
}

impl Isa {
    /// Decode a target string
    pub fn get(target: &str) -> Option<Isa> {
        let arch = Arch::get(target)?;
        Some(match arch {
            Arch::Armv4T | Arch::Armv5TE => Isa::A32,
            Arch::Armv6M => Isa::T32,
            Arch::Armv7M => Isa::T32,
            Arch::Armv7EM => Isa::T32,
            Arch::Armv8MBase => Isa::T32,
            Arch::Armv8MMain => Isa::T32,
            Arch::Armv7R => Isa::A32,
            Arch::Armv8R => Isa::A32,
            Arch::Armv7A => Isa::A32,
            Arch::Armv8A => Isa::A64,
        })
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
    /// Armv4T (legacy, also known as ARMv4T)
    Armv4T,
    /// Armv5TE (also known as ARMv5TE)
    Armv5TE,
    /// Armv6-M (also known as ARMv6-M)
    Armv6M,
    /// Armv7-M (also known as ARMv7-M)
    Armv7M,
    /// Armv7E-M (also known as ARMv7E-M)
    Armv7EM,
    /// Armv8-M Baseline
    Armv8MBase,
    /// Armv8-M with Mainline extensions
    Armv8MMain,
    /// Armv7-R (also known as ARMv7-R)
    Armv7R,
    /// Armv8-R
    Armv8R,
    /// Armv7-A (also known as ARMv7-A)
    Armv7A,
    /// Armv8-A
    Armv8A,
}

impl Arch {
    /// Decode a target string
    pub fn get(target: &str) -> Option<Arch> {
        if target.starts_with("armv4t-") {
            Some(Arch::Armv4T)
        } else if target.starts_with("armv5te-") {
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
        } else if target.starts_with("armv7r-") || target.starts_with("armebv7r") {
            Some(Arch::Armv7R)
        } else if target.starts_with("armv8r-") {
            Some(Arch::Armv8R)
        } else if target.starts_with("armv7a-") {
            Some(Arch::Armv7A)
        } else if target.starts_with("aarch64-") || target.starts_with("aarch64be-") {
            Some(Arch::Armv8A)
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
            Arch::Armv4T | Arch::Armv5TE => Profile::Legacy,
            Arch::Armv7R | Arch::Armv8R => Profile::R,
            Arch::Armv7A | Arch::Armv8A => Profile::A,
        }
    }

    /// Get a comma-separated list of values, suitable for cfg-check
    pub fn values() -> String {
        let string_versions: Vec<String> = [
            Arch::Armv4T,
            Arch::Armv5TE,
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
