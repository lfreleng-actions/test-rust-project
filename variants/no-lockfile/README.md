<!--
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Linux Foundation
-->

# No-lockfile variant

A dependency-free library whose `.gitignore` keeps `Cargo.lock` out
of git, so a checkout has no lockfile. It exercises
`lockfile_required`: with `false` an action generates the lockfile and
warns, with `true` it fails.
