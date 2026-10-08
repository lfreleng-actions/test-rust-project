// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Test helpers for the other members; never published.

/// Panics unless `text` is the greeting for `name`.
pub fn assert_greets(text: &str, name: &str) {
    assert_eq!(text, format!("Hello, {name}!"));
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn rejects_another_name() {
        super::assert_greets("Hello, core!", "cli");
    }
}
