<!--
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Linux Foundation
-->

# MSRV variant

A dependency-free edition 2024 library that declares
`rust-version = "1.85"` and pins Rust 1.90.0 in `rust-toolchain.toml`.
The two differ, so an MSRV test matrix runs two legs: the project
toolchain and the declared MSRV.
