//! CLI tool for arm-targets

use std::env;

use arm_targets::TargetInfo;

/// Entry point to the program
#[allow(deprecated)]
fn main() {
    let target_info = get_target_info();
    if let Some(isa) = target_info.isa() {
        println!("- ISA is {}", isa);
    } else {
        println!("- ISA is unknown");
    }
    if let Some(arch) = target_info.arch() {
        println!("- Architecture is {}", arch);
    } else {
        println!("- Architecture is unknown");
    }
    if let Some(abi) = target_info.abi() {
        println!("- ABI is {}", abi);
    } else {
        println!("- ABI is unknown");
    }
}

fn get_target_info() -> TargetInfo {
    if let Some(target) = env::args().nth(1) {
        println!("These are the features for the target {:?}", target);
        TargetInfo::get(&target)
    } else {
        eprintln!(r#"I need a target string as an argument, like "armv7a-none-eabi""#);
        std::process::exit(1);
    }
}
