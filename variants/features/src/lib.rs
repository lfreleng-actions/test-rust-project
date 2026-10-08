// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Linux Foundation

//! Greets in a language the enabled features choose.

/// Returns a greeting for `name`. `french` wins over `english`; with
/// neither the greeting is plain, and `shout` upper-cases it.
pub fn greet(name: &str) -> String {
    let text = format!("{}, {name}!", salutation());
    if cfg!(feature = "shout") {
        text.to_uppercase()
    } else {
        text
    }
}

fn salutation() -> &'static str {
    if cfg!(feature = "french") {
        "Bonjour"
    } else if cfg!(feature = "english") {
        "Hello"
    } else {
        "Hi"
    }
}

/// Only compiled with the `shout` feature.
#[cfg(feature = "shout")]
pub fn shout(text: &str) -> String {
    text.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    #[cfg(not(any(feature = "english", feature = "french", feature = "shout")))]
    fn plain_without_features() {
        assert_eq!(greet("Cargo"), "Hi, Cargo!");
    }

    #[test]
    #[cfg(all(feature = "english", not(feature = "french"), not(feature = "shout")))]
    fn english_by_default() {
        assert_eq!(greet("Cargo"), "Hello, Cargo!");
    }

    #[test]
    #[cfg(all(feature = "french", not(feature = "shout")))]
    fn french_when_enabled() {
        assert_eq!(greet("Cargo"), "Bonjour, Cargo!");
    }

    #[test]
    #[cfg(all(feature = "french", feature = "shout"))]
    fn every_feature_together() {
        assert_eq!(greet("Cargo"), super::shout("Bonjour, Cargo!"));
        assert_eq!(greet("Cargo"), "BONJOUR, CARGO!");
    }
}
