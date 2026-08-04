pub(crate) mod test_utils {
    use std::path::{Path, PathBuf};
    use std::process::Command;

    /// `aarch32-tests/` -> repo root -> `<name>` (e.g. "examples/versatileab").
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

    pub fn discover_targets(dir: &Path) -> Vec<String> {
        let text = [".cargo/config.toml", ".cargo/config"]
            .iter()
            .find_map(|name| std::fs::read_to_string(dir.join(name)).ok())
            .unwrap_or_default();

        let mut targets: Vec<String> = text
            .lines()
            .filter_map(|line| line.trim().strip_prefix("[target."))
            .filter_map(|rest| rest.strip_suffix(']'))
            // Skip cfg() predicates like [target.'cfg(...)']; keep plain triples.
            .filter(|triple| !triple.starts_with('\'') && !triple.starts_with('"'))
            .map(String::from)
            .collect();
        targets.sort();
        targets.dedup();
        targets
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

    pub fn run_bin(dir: &Path, bin: &str, target: &str, flags: &[String]) -> String {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(dir)
            .arg("run")
            .arg(format!("--target={target}"))
            .args(flags)
            .args(["--bin", bin]);

        if let Ok(rustflags) = std::env::var("QEMU_RUSTFLAGS") {
            cmd.env("RUSTFLAGS", rustflags);
        }

        let output = cmd.output().expect("failed to execute cargo run");

        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}
