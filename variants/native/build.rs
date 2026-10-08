// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Links libsodium, which setup.sh installs.

fn main() {
    println!("cargo:rustc-link-lib=sodium");
    println!("cargo:rerun-if-changed=build.rs");
}
