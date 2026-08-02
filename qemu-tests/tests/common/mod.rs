pub(crate) mod TestUtils {
    use std::path::{Path, PathBuf};
    use std::process::Command;

    /// `qemu-tests/` -> repo root -> `<name>` (e.g. "examples/versatileab").
    pub fn test_dir(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(name)
    }

    pub fn discover_examples() -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(test_dir("examples"))
            .expect("cannot read examples/")
            .map(|e| e.expect("cannot read examples/ entry"))
            .filter(|e| e.path().join("Cargo.toml").is_file())
            .map(|e| {
                e.file_name()
                    .into_string()
                    .unwrap_or_else(|bad| panic!("non-UTF-8 example dir name: {bad:?}"))
            })
            .collect();
        names.sort();
        names
    }

    pub fn supports_target(dir: &Path, target: &str) -> bool {
        let needle = format!("[target.{target}]");
        [".cargo/config.toml", ".cargo/config"]
            .iter()
            .filter_map(|name| std::fs::read_to_string(dir.join(name)).ok())
            .any(|text| text.contains(&needle))
    }

    pub fn discover_bins(dir: &Path) -> Vec<String> {
        let output = Command::new("cargo")
            .current_dir(dir)
            .args(["metadata", "--format-version=1", "--no-deps"])
            .output()
            .expect("failed to run cargo metadata");
        assert!(
            output.status.success(),
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let meta: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("cargo metadata produced invalid JSON");

        let mut bins: Vec<String> = meta["packages"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(|pkg| pkg["targets"].as_array().into_iter().flatten())
            .filter(|target| {
                target["kind"]
                    .as_array()
                    .is_some_and(|kinds| kinds.iter().any(|k| k.as_str() == Some("bin")))
            })
            .filter_map(|target| target["name"].as_str().map(String::from))
            .collect();
        bins.sort();
        bins
    }

    /// Build + run one bin in QEMU via the crate's cargo runner, return stdout.
    pub fn run_bin(dir: &Path, bin: &str, target: &str, flags: &[String]) -> String {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(dir)
            .arg("run")
            .arg(format!("--target={target}"))
            .args(flags)
            .args(["--bin", bin]);

        // Forward QEMU_RUSTFLAGS to this cross build only (as RUSTFLAGS). Setting
        // RUSTFLAGS on the outer `cargo test` would compile the host test crate
        // with an ARM target-cpu and fail; here it hits only the --target build.
        if let Ok(rustflags) = std::env::var("QEMU_RUSTFLAGS") {
            cmd.env("RUSTFLAGS", rustflags);
        }

        let output = cmd.output().expect("failed to execute cargo run");

        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}
