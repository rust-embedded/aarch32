//! CLI tool for arm-targets

use std::env;

/// Entry point to the program
fn main() {
    if let Some(target) = env::args().nth(1) {
        println!("// These are the features for the target '{}'", target);
        #[allow(deprecated)]
        arm_targets::process_target(&target);
    } else {
        println!("// These are the features this crate enables:");
        if env::var("TARGET").is_ok() {
            arm_targets::process();
        } else {
            #[allow(deprecated)]
            arm_targets::process_target("");
        }
    }
}
