#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Linux Foundation

# Installs libsodium-dev, which the native variant links against, on a
# Debian or Ubuntu host. Safe to run more than once: it does nothing
# when the package is already installed.

set -euo pipefail

package="libsodium-dev"

if ! command -v apt-get > /dev/null; then
    echo "setup.sh: needs apt-get (Debian or Ubuntu)" >&2
    exit 1
fi

# shellcheck disable=SC2016 # dpkg-query expands ${Status}, not the shell
if dpkg-query -W -f '${Status}' "$package" 2> /dev/null \
    | grep -q "install ok installed"; then
    echo "setup.sh: $package already installed"
    exit 0
fi

sudo apt-get update
sudo DEBIAN_FRONTEND=noninteractive \
    apt-get install -y --no-install-recommends "$package"
echo "setup.sh: installed $package"
