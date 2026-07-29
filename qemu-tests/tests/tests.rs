mod common;

use common::TestUtils;

/// Snapshot every example's bins in QEMU. Driven by the justfile, once per
/// target; the build matrix lives there (single source of truth).
///
///   QEMU_TARGET    required, e.g. "armv7a-none-eabi"
///   QEMU_FLAGS     optional, whitespace-separated extra cargo flags
///   QEMU_BINS      optional, comma-separated bin allowlist
///   QEMU_EXAMPLES  optional, comma-separated example-folder allowlist
///
/// Examples are auto-discovered from `examples/`; one whose cargo config has no
/// runner for QEMU_TARGET is skipped (logged, not silent). Ignored by default
/// so a bare `cargo test` is a no-op.
#[test]
#[ignore = "requires QEMU_TARGET; run via the justfile"]
fn test_examples() {
    let target = std::env::var("QEMU_TARGET")
        .expect("set QEMU_TARGET (run this via `just test-insta`, not bare cargo test)");
    let flags: Vec<String> = std::env::var("QEMU_FLAGS")
        .unwrap_or_default()
        .split_whitespace()
        .map(String::from)
        .collect();
    let only_bins = env_list("QEMU_BINS");
    let only_examples = env_list("QEMU_EXAMPLES");

    for example in TestUtils::discover_examples() {
        if matches!(&only_examples, Some(list) if !list.contains(&example)) {
            continue;
        }
        let dir = TestUtils::test_dir(&format!("examples/{example}"));
        if !TestUtils::supports_target(&dir, &target) {
            eprintln!("skipping example `{example}`: no QEMU runner for {target}");
            continue;
        }

        // Per-example folder: snapshots/<example>/<bin>-<target>.snap
        let mut settings = insta::Settings::clone_current();
        settings.add_filter(r"\r", ""); // strip CR
        settings.add_filter(r"\\\\", "/"); // Windows path sep -> UNIX
        settings.set_snapshot_path(format!("snapshots/{example}"));
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();

        for bin in TestUtils::discover_bins(&dir) {
            if matches!(&only_bins, Some(list) if !list.contains(&bin)) {
                continue;
            }
            let stdout = TestUtils::run_bin(&dir, &bin, &target, &flags);
            insta::assert_snapshot!(format!("{bin}-{target}"), stdout);
        }
    }
}

/// Parse a comma-separated env var into a trimmed allowlist, or `None` if unset.
fn env_list(var: &str) -> Option<Vec<String>> {
    std::env::var(var)
        .ok()
        .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
}
