<!--
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Linux Foundation
-->

# Native variant

A library that links the system libsodium through its build script
and calls it over the C ABI, with no crate dependencies. The runner
image lacks libsodium, so the build fails until `setup.sh` installs
`libsodium-dev` with apt. It exercises `setup_script: setup.sh`.

`setup.sh` supports Debian and Ubuntu and needs `sudo`. It does
nothing when the package is already present. On another system,
install libsodium yourself and, where the linker cannot find it, pass
its directory: `RUSTFLAGS="-L /opt/homebrew/lib"` on macOS with
Homebrew.
