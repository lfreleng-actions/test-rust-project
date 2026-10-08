// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Runs the built binary end to end.

use std::process::Command;

#[test]
fn greets_the_workspace_by_default() {
    let output = Command::new(env!("CARGO_BIN_EXE_fixture-workspace-cli"))
        .output()
        .expect("binary should run");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"Hello, workspace!\n");
}
