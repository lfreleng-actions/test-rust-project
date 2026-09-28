// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Runs the built binary end to end.

use std::process::Command;

fn run(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_lfreleng-test-rust-project"))
        .args(args)
        .output()
        .expect("binary should run");
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("stdout should be UTF-8")
}

#[test]
fn greets_the_world_by_default() {
    assert_eq!(run(&[]), "Hello, world!\n");
}

#[test]
fn greets_the_first_argument() {
    assert_eq!(run(&["Cargo"]), "Hello, Cargo!\n");
}
