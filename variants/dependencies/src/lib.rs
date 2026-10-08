// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Uses one crates.io dependency, `semver`.

use semver::Version;

/// Returns whether `version` parses as a stable semantic version.
pub fn is_stable(version: &str) -> bool {
    Version::parse(version).is_ok_and(|v| v.pre.is_empty())
}

#[cfg(test)]
mod tests {
    use super::is_stable;

    #[test]
    fn accepts_a_release() {
        assert!(is_stable("1.2.3"));
    }

    #[test]
    fn rejects_a_pre_release_and_garbage() {
        assert!(!is_stable("1.2.3-rc.1"));
        assert!(!is_stable("not a version"));
    }
}
