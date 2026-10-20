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

## On crates.io

The Linux Foundation publishes this crate to crates.io as
[`lfreleng-test-rust-project`](https://crates.io/crates/lfreleng-test-rust-project),
for two reasons:

- Release tooling needs a real published crate to test against. For
  example, rust-crate-publish-action compares a package with the
  version already on crates.io before uploading.
- Tests that check the name keep working, because nobody else can
  publish a crate under it.

This sample crate serves testing tools, not projects that depend on
it, and its API may change in any release. Consuming workflows run
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

## Fixture variants

The root crate avoids dependencies, workspaces and features on
purpose. Directories under `variants/` cover those shapes instead.
Select one by checking out this repository as usual and passing its
directory as `path_prefix`:

<!-- markdownlint-disable MD013 -->

| `path_prefix`                             | Shape                                                   | Exercises                                         |
| ----------------------------------------- | ------------------------------------------------------- | ------------------------------------------------- |
| `test-rust-project/variants/workspace`    | Virtual workspace: library, binary, unpublished helper  | `workspace`, `packages`, `exclude`, publish order |
| `test-rust-project/variants/no-lockfile`  | Library with no `Cargo.lock` in git                     | `lockfile_required`                               |
| `test-rust-project/variants/msrv`         | `rust-version = "1.85"`, `rust-toolchain.toml` 1.90.0   | MSRV test matrix with two legs                    |
| `test-rust-project/variants/dependencies` | One crates.io dependency (`semver`), committed lockfile | Audit, SBOM and vulnerability scans               |
| `test-rust-project/variants/native`       | Links the system libsodium; `setup.sh` installs it      | `setup_script: setup.sh`                          |
| `test-rust-project/variants/features`     | Default and optional features, a test per selection     | `features`, `all_features`, `no_default_features` |

<!-- markdownlint-enable MD013 -->

The `path_prefix` values assume the checkout `path` shown above.
Each variant's own README describes what it exercises.

The variants live here, rather than in separate fixture repositories,
which leaves callers one repository and one pinned commit. They stay
apart from the root crate:

- None is a member of a root workspace. Each is its own package or
  workspace, with its own lockfile and `target` directory.
- The root `include` list keeps them out of the published `.crate`.
- Every variant package sets `publish = false`, except the two
  publishable members of the workspace variant. Their names run past
  the 64 characters crates.io accepts, so they can never collide with
  a real crate, and the registry rejects any upload.
- `.github/workflows/testing.yaml` builds and tests each variant in a
  job of its own. Of the variants, Dependabot watches
  `variants/dependencies`, the one with a crates.io dependency.

## Model B release file

`releases/workspace-0.1.0.yaml` serves rust-workflows' merge lane,
which treats a merged commit that adds a YAML file under `releases/`
as a release of the version the file names. The commit that added it
gives that lane's self-test a release to verify, with
`path_prefix: variants/workspace`: a dry run checks the version against
the workspace's crates and crates.io, and stops before publishing or
tagging.

The file names the workspace variant's version, not the root crate's.
The publishable members of that workspace have names longer than
crates.io accepts, so the registry never holds the version, every dry
run finds it absent, and no upload can succeed. The root crate would
not do: any commit after its release tag packages a different archive,
which a dry run for an already-published version rejects.

Nothing in this repository reads `releases/`. This repository releases
through tags, as [Releasing](#releasing) describes. Leave the file in
place: callers pin the commit that added it.

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

## Releasing

Each release tag names the crate version it publishes, with a leading
`v`: tag `v0.1.3` publishes version `0.1.3`. `Cargo.toml` and
`Cargo.lock` keep the placeholder `0.0.0`, and the release lane writes
the tag's version over it before building, verifying and publishing.
A release needs no version-bump pull request. Tags up to
`v0.0.3` predate this scheme and do not match their crate versions;
`v0.1.0` to `v0.1.2` came from manifests carrying the version.

Tag the tip of `main`, so the published crate carries what `main`
holds. Here `origin` names `lfreleng-actions/test-rust-project`:

```bash
git fetch origin
git tag -s v0.1.3 -m "v0.1.3" origin/main
git push origin v0.1.3
```

Sign the tag and push it. The release lane rejects an unsigned tag,
one off the current tip of `main`, and one not above every earlier
version tag.

Pushing a `v<major>.<minor>.<patch>` tag starts
`.github/workflows/build-test-release.yaml`, a thin caller of the
Rust release lane in
[rust-workflows](https://github.com/lfreleng-actions/rust-workflows)
(`build-test-release.yaml`, Model A, `version_source: tag`). The
lane validates the tag, resolves one exact toolchain for the run,
builds the crate and its binary, audits and scans the dependency graph,
runs the tests, dry-runs the publish to record the archive's SHA-256,
attaches the `.crate`, the binary, the SBOMs and `SHA256SUMS` to a
draft GitHub release with build provenance attestations and Sigstore
bundles, publishes through crates.io Trusted Publishing, and then
promotes the release. A tag that fails any earlier check never gets a
published GitHub release or a crates.io upload.

crates.io accepts uploads from this workflow file alone, running in the
`production` environment, whose deployment rule admits tags matching
`v[0-9]*.[0-9]*.[0-9]*` alone. Renaming either breaks publication
until the crate's trusted publisher settings on crates.io change to
match. crates.io matches the OIDC `workflow_ref` claim, which names
the caller's file, so the publish job can sit inside the reusable
workflow; this repository's first release through the lane is the
proof of that. The publish job alone holds a crates.io credential,
runs no crate code, and uploads nothing but an archive byte-identical
to the one `crate-verify` compiled and tested, with the same Cargo.
Re-running a release that already reached crates.io uploads nothing:
the action finds the identical archive there and skips.

This is the repository's one tag-push workflow, as in the Python
release lane. A second workflow on the same event would act on tags
this one rejects, and could publish a GitHub release for a crate
version that never reached crates.io.

[pre-commit.ci results page]: https://results.pre-commit.ci/latest/github/lfreleng-actions/test-rust-project/main
[pre-commit.ci status badge]: https://results.pre-commit.ci/badge/github/lfreleng-actions/test-rust-project/main.svg
