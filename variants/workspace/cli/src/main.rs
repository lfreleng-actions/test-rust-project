// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Prints a greeting through the `core` member.

use lfreleng_fixture_workspace_core_name_too_long_for_crates_io_by_design::greet;

fn main() {
    let name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "workspace".to_owned());
    println!("{}", greet(&name));
}
