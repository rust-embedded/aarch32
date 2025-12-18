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
//! cargo:rustc-check-cfg=cfg(arm_abi, values("eabi", "eabihf"))
//! ```

#[derive(Default)]
pub struct TargetInfo {
    isa: Option<Isa>,
    arch: Option<Arch>,
    profile: Option<Profile>,
    abi: Option<Abi>,
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

    /// Get the ABI of the target
    pub fn abi(&self) -> Option<Abi> {
        self.abi
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

    if let Some(abi) = Abi::get(target) {
        println!(r#"cargo:rustc-cfg=arm_abi="{}""#, abi);
        target_info.abi = Some(abi);
    }
    println!(
        r#"cargo:rustc-check-cfg=cfg(arm_abi, values({}))"#,
        Abi::values()
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
        } else if target.starts_with("armv8r-") || target.starts_with("thumbv8r-") {
            Some(Arch::Armv8R)
        } else if target.starts_with("armv7a-") || target.starts_with("thumbv7a-") {
            Some(Arch::Armv7A)
        } else if target.starts_with("aarch64-") || target.starts_with("aarch64be-") {
            Some(Arch::Armv8A)
        } else if target.starts_with("arm-")
            || target.starts_with("armv6-")
            || target.starts_with("thumbv6-")
        {
            // If not specified, assume Armv6
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

    #[test]
    fn armv4t_none_eabi() {
        let target = "armv4t-none-eabi";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::A32));
        assert_eq!(target_info.arch(), Some(Arch::Armv4T));
        assert_eq!(target_info.profile(), Some(Profile::Legacy));
        assert_eq!(target_info.abi(), Some(Abi::Eabi));
    }

    #[test]
    fn armv5te_none_eabi() {
        let target = "armv5te-none-eabi";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::A32));
        assert_eq!(target_info.arch(), Some(Arch::Armv5TE));
        assert_eq!(target_info.profile(), Some(Profile::Legacy));
        assert_eq!(target_info.abi(), Some(Abi::Eabi));
    }

    #[test]
    fn armv6_none_eabi() {
        let target = "armv6-none-eabi";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::A32));
        assert_eq!(target_info.arch(), Some(Arch::Armv6));
        assert_eq!(target_info.profile(), Some(Profile::Legacy));
        assert_eq!(target_info.abi(), Some(Abi::Eabi));
    }

    #[test]
    fn armv6_none_eabihf() {
        let target = "armv6-none-eabihf";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::A32));
        assert_eq!(target_info.arch(), Some(Arch::Armv6));
        assert_eq!(target_info.profile(), Some(Profile::Legacy));
        assert_eq!(target_info.abi(), Some(Abi::EabiHf));
    }

    #[test]
    fn arm_unknown_linux_gnueabi() {
        let target = "arm-unknown-linux-gnueabi";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::A32));
        assert_eq!(target_info.arch(), Some(Arch::Armv6));
        assert_eq!(target_info.profile(), Some(Profile::Legacy));
        assert_eq!(target_info.abi(), Some(Abi::Eabi));
    }

    #[test]
    fn thumbv6m_none_eabi() {
        let target = "thumbv6m-none-eabi";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::T32));
        assert_eq!(target_info.arch(), Some(Arch::Armv6M));
        assert_eq!(target_info.profile(), Some(Profile::M));
        assert_eq!(target_info.abi(), Some(Abi::Eabi));
    }

    #[test]
    fn thumbv7m_none_eabi() {
        let target = "thumbv7m-none-eabi";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::T32));
        assert_eq!(target_info.arch(), Some(Arch::Armv7M));
        assert_eq!(target_info.profile(), Some(Profile::M));
        assert_eq!(target_info.abi(), Some(Abi::Eabi));
    }

    #[test]
    fn thumbv7em_nuttx_eabihf() {
        let target = "thumbv7em-nuttx-eabihf";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::T32));
        assert_eq!(target_info.arch(), Some(Arch::Armv7EM));
        assert_eq!(target_info.profile(), Some(Profile::M));
        assert_eq!(target_info.abi(), Some(Abi::EabiHf));
    }

    #[test]
    fn thumbv8m_base_none_eabi() {
        let target = "thumbv8m.base-none-eabi";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::T32));
        assert_eq!(target_info.arch(), Some(Arch::Armv8MBase));
        assert_eq!(target_info.profile(), Some(Profile::M));
        assert_eq!(target_info.abi(), Some(Abi::Eabi));
    }

    #[test]
    fn thumbv8m_main_none_eabihf() {
        let target = "thumbv8m.main-none-eabihf";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::T32));
        assert_eq!(target_info.arch(), Some(Arch::Armv8MMain));
        assert_eq!(target_info.profile(), Some(Profile::M));
        assert_eq!(target_info.abi(), Some(Abi::EabiHf));
    }

    #[test]
    fn armv7r_none_eabi() {
        let target = "armv7r-none-eabi";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::A32));
        assert_eq!(target_info.arch(), Some(Arch::Armv7R));
        assert_eq!(target_info.profile(), Some(Profile::R));
        assert_eq!(target_info.abi(), Some(Abi::Eabi));
    }

    #[test]
    fn armv8r_none_eabihf() {
        let target = "armv8r-none-eabihf";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::A32));
        assert_eq!(target_info.arch(), Some(Arch::Armv8R));
        assert_eq!(target_info.profile(), Some(Profile::R));
        assert_eq!(target_info.abi(), Some(Abi::EabiHf));
    }

    #[test]
    fn thumbv8r_none_eabihf() {
        let target = "thumbv8r-none-eabihf";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::T32));
        assert_eq!(target_info.arch(), Some(Arch::Armv8R));
        assert_eq!(target_info.profile(), Some(Profile::R));
        assert_eq!(target_info.abi(), Some(Abi::EabiHf));
    }

    #[test]
    fn armv7a_none_eabi() {
        let target = "armv7a-none-eabi";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::A32));
        assert_eq!(target_info.arch(), Some(Arch::Armv7A));
        assert_eq!(target_info.profile(), Some(Profile::A));
        assert_eq!(target_info.abi(), Some(Abi::Eabi));
    }

    #[test]
    fn aarch64_none_eabihf() {
        let target = "aarch64-unknown-none";
        let target_info = process_target(target);
        assert_eq!(target_info.isa(), Some(Isa::A64));
        assert_eq!(target_info.arch(), Some(Arch::Armv8A));
        assert_eq!(target_info.profile(), Some(Profile::A));
        assert_eq!(target_info.abi(), None);
    }
}
