<!--
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Linux Foundation
-->

# Dependencies variant

A library with one crates.io dependency,
[`semver`](https://crates.io/crates/semver), and a committed
`Cargo.lock`, so audit, SBOM and vulnerability scans have a real
package to report. `semver` has no runtime dependencies of its own and
no RustSec advisory. Never replace it with a crate that has one: this
repository would then raise security alerts against itself.
