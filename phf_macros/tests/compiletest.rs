#![cfg(compiletest)]

extern crate compiletest_rs as compiletest;

use compiletest::Config;
use std::path::Path;

#[allow(dead_code)]
fn run_mode(directory: &'static str, mode: &'static str) {
    let mut config = Config::default();
    let cfg_mode = mode.parse().ok().expect("Invalid mode");

    config.mode = cfg_mode;
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    config.target_rustcflags = Some(format!("-L {}", dir.join("../target/debug/deps").display()));
    config.src_base = dir.join("tests").join(directory);

    compiletest::run_tests(&config);
}

#[cfg(feature = "unicase_support")]
#[test]
#[ignore]
fn compile_test_unicase() {
    run_mode("compile-fail-unicase", "compile-fail");
}

#[test]
#[ignore]
fn compile_fail() {
    run_mode("compile-fail", "compile-fail");
}
