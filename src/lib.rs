// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Sample library used to exercise Rust tooling in lfreleng-actions.

/// Returns a greeting for `name`.
///
/// ```
/// assert_eq!(lfreleng_test_rust_project::greet("world"), "Hello, world!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Adds two numbers, returning `None` when the sum overflows.
///
/// ```
/// use lfreleng_test_rust_project::checked_sum;
///
/// assert_eq!(checked_sum(2, 3), Some(5));
/// assert_eq!(checked_sum(u64::MAX, 1), None);
/// ```
pub fn checked_sum(a: u64, b: u64) -> Option<u64> {
    a.checked_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn greets_an_empty_name() {
        assert_eq!(greet(""), "Hello, !");
    }

    #[test]
    fn sums_within_range() {
        assert_eq!(checked_sum(40, 2), Some(42));
    }

    #[test]
    fn reports_overflow() {
        assert_eq!(checked_sum(u64::MAX, 1), None);
    }
}
