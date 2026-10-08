// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Calls into libsodium through the C ABI, without a -sys crate.

use std::ffi::{c_char, c_int, CStr};

extern "C" {
    fn sodium_init() -> c_int;
    fn sodium_version_string() -> *const c_char;
}

/// Initialises libsodium and returns its version string.
pub fn sodium_version() -> String {
    // SAFETY: sodium_init is safe to call repeatedly from any thread.
    // sodium_version_string returns a pointer to a static C string.
    unsafe {
        assert!(sodium_init() >= 0, "libsodium failed to initialise");
        CStr::from_ptr(sodium_version_string())
            .to_string_lossy()
            .into_owned()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reports_a_version() {
        let version = super::sodium_version();
        assert!(version.starts_with(|c: char| c.is_ascii_digit()));
    }
}
