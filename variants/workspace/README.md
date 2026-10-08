<!--
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Linux Foundation
-->

# Workspace variant

A virtual workspace with three members that inherit their version,
edition, licence and repository from `[workspace.package]`:

| Member    | Kind            | Publishable | Depends on                     |
| --------- | --------------- | ----------- | ------------------------------ |
| `core`    | library         | yes         | `support` (dev, by path)       |
| `cli`     | binary          | yes         | `core` (by path, with version) |
| `support` | library         | no          | nothing                        |

It exercises `workspace`, `packages` and `exclude`, and publishing
logic that has to skip a `publish = false` member and order the rest.

The publishable members carry names longer than the 64 characters
crates.io accepts, so they can never collide with a real crate and a
real upload fails at the registry. `cargo package --workspace` and
`cargo publish --dry-run --workspace` still succeed locally.
