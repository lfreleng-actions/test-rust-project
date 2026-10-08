// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Builds on every Rust release from its declared `rust-version` on.

/// Returns a greeting for `name`.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn greets_by_name() {
        assert_eq!(super::greet("MSRV"), "Hello, MSRV!");
    }
}
