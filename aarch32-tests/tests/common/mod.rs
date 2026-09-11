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

    pub fn run_bin<S1, S2>(
        dir: &Path,
        bin: &str,
        target: &str,
        flags: impl IntoIterator<Item = S1>,
        rustflags: impl IntoIterator<Item = S2>,
    ) -> String
    where
        S1: AsRef<std::ffi::OsStr>,
        S2: AsRef<str>,
    {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(dir)
            .arg("run")
            .arg(format!("--target={target}"))
            .args(flags)
            .args(["--bin", bin]);

        let mut rustflags_str = String::new();
        for rustflag in rustflags {
            rustflags_str.push_str(rustflag.as_ref());
            rustflags_str.push_str(" ");
        }
        // Some variants (e.g. fpu-d32) need a target-feature/-cpu; pass it only to
        // this cross build, never to the host test binary.
        if !rustflags_str.is_empty() {
            cmd.env("RUSTFLAGS", rustflags_str);
        }

        if std::env::var("TEST_VERBOSE").is_ok() {
            eprintln!("\nRunning {:?}", cmd);
        }

        let output = cmd.output().expect("failed to execute cargo run");

        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}
