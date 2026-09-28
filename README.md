<!--
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Linux Foundation
-->

# 🦀 Test Rust Project

<!-- prettier-ignore-start -->
<!-- markdownlint-disable-next-line MD013 -->
[![Linux Foundation](https://img.shields.io/badge/Linux-Foundation-blue)](https://linuxfoundation.org/) [![Source Code](https://img.shields.io/badge/GitHub-100000?logo=github&logoColor=white&color=blue)](https://github.com/lfreleng-actions/test-rust-project) [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![pre-commit.ci status badge]][pre-commit.ci results page] [![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/lfreleng-actions/test-rust-project/badge)](https://scorecard.dev/viewer/?uri=github.com/lfreleng-actions/test-rust-project)
<!-- prettier-ignore-end -->

Sample Rust project used for testing actions that build, package or
publish Rust crates, such as
[rust-crate-publish-action](https://github.com/lfreleng-actions/rust-crate-publish-action).

## test-rust-project

A single crate, `lfreleng-test-rust-project`, with a library, a small
binary and tests. It stays minimal on purpose, so that actions tested
against it exercise their own behaviour rather than a complex build:

- No dependencies, so building it needs no crate downloads.
- A committed `Cargo.lock`, so consumers that pass `--locked`, as
  rust-crate-publish-action does, find the lockfile they require.
- Complete crates.io metadata, so packaging reports no warnings.
- An `include` list that keeps the packaged `.crate` to the sources,
  tests, this README and the licence.

The crate is not for publication. Consuming workflows run
`cargo publish --dry-run` against it, which reads the crates.io index
but uploads nothing.

## Usage in action tests

<!-- markdownlint-disable MD013 MD046 -->

```yaml
- name: "Checkout test project"
  uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
  with:
    repository: "lfreleng-actions/test-rust-project"
    ref: "<commit-sha>"
    path: "test-rust-project"
    persist-credentials: false
```

<!-- markdownlint-enable MD013 MD046 -->

Pin `ref` to a commit SHA, so a change here cannot alter the results
of a consuming workflow unannounced.

## Building locally

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo package --locked
cargo run --locked -- Cargo
```

`.github/workflows/testing.yaml` runs the same checks on every pull
request, followed by `cargo publish --dry-run`.

[pre-commit.ci results page]: https://results.pre-commit.ci/latest/github/lfreleng-actions/test-rust-project/main
[pre-commit.ci status badge]: https://results.pre-commit.ci/badge/github/lfreleng-actions/test-rust-project/main.svg
