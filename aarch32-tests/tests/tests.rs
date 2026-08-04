mod common;

use common::test_utils;

/// Snapshot every example's bins in AARCH32, once per target the example supports.
/// Targets are auto-discovered from each example's cargo config, so the harness
/// is no longer handed a single target from the environment — it runs them all.
///
///   AARCH32_FLAGS     optional, whitespace-separated extra cargo flags
///   AARCH32_RUSTFLAGS optional, forwarded as RUSTFLAGS to the cross build only
///   AARCH32_TARGETS   optional, comma-separated target allowlist (restricts the
///                  discovered set — used by the tier3/fpu-d32 invocations)
///   AARCH32_BINS      optional, comma-separated bin allowlist
///   AARCH32_EXAMPLES  optional, comma-separated example-folder allowlist
///
/// Examples and targets are auto-discovered; an example whose config lists no
/// targets is skipped (logged, not silent). Ignored by default so a bare
/// `cargo test` is a no-op.
#[test]
#[ignore = "runs AARCH32; drive it via `just test-qemu`, not bare cargo test"]
fn test_examples() {
    let flags: Vec<String> = std::env::var("AARCH32_FLAGS")
        .unwrap_or_default()
        .split_whitespace()
        .map(String::from)
        .collect();
    let only_targets = env_list("AARCH32_TARGETS");
    let only_bins = env_list("AARCH32_BINS");
    let only_examples = env_list("AARCH32_EXAMPLES");

    for example in test_utils::discover_examples() {
        if matches!(&only_examples, Some(list) if !list.contains(&example)) {
            continue;
        }
        let dir = test_utils::test_dir(&format!("examples/{example}"));

        let targets: Vec<String> = test_utils::discover_targets(&dir)
            .into_iter()
            .filter(|t| !matches!(&only_targets, Some(list) if !list.contains(t)))
            .collect();
        if targets.is_empty() {
            eprintln!("skipping example `{example}`: no targets in cargo config");
            continue;
        }

        // Per-example folder: snapshots/<example>/<bin>-<target>.snap
        let mut settings = insta::Settings::clone_current();
        settings.add_filter(r"\r", ""); // strip CR
        settings.add_filter(r"\\\\", "/"); // Windows path sep -> UNIX
        settings.set_snapshot_path(format!("snapshots/{example}"));
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();

        let bins = test_utils::discover_bins(&dir);
        for target in &targets {
            for bin in &bins {
                if matches!(&only_bins, Some(list) if !list.contains(bin)) {
                    continue;
                }
                let stdout = test_utils::run_bin(&dir, bin, target, &flags);
                insta::assert_snapshot!(format!("{bin}-{target}"), stdout);
            }
        }
    }
}

/// Parse a comma-separated env var into a trimmed allowlist, or `None` if unset.
fn env_list(var: &str) -> Option<Vec<String>> {
    std::env::var(var)
        .ok()
        .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
}
