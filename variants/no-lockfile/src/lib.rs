// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Builds without a committed `Cargo.lock`.

/// Returns a greeting for `name`.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn greets_by_name() {
        assert_eq!(super::greet("lockfile"), "Hello, lockfile!");
    }
}
