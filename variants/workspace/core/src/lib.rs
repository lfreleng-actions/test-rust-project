// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Library member of the workspace fixture; `cli` depends on it.

/// Returns a greeting for `name`.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use lfreleng_fixture_workspace_support::assert_greets;

    #[test]
    fn greets_by_name() {
        assert_greets(&super::greet("core"), "core");
    }
}
