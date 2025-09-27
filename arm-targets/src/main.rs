//! CLI tool for arm-targets

/// Entry point to the program
fn main() {
    if let Some(target) = std::env::args().skip(1).next() {
        println!("// These are the features for the target '{}'", target);
        arm_targets::process_target(&target);
    } else {
        println!("// These are the features this crate enables:");
        arm_targets::process_target("");
    }
}
