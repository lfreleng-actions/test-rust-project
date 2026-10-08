<!--
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Linux Foundation
-->

# Features variant

A dependency-free library with three optional features, one of them
on by default. Each combination below compiles a different test, so
the name of the test that ran shows which build a workflow tested:

| Selection             | Greeting          | Test that runs           |
| --------------------- | ----------------- | ------------------------ |
| default (`english`)   | `Hello, Cargo!`   | `english_by_default`     |
| `no_default_features` | `Hi, Cargo!`      | `plain_without_features` |
| `features: french`    | `Bonjour, Cargo!` | `french_when_enabled`    |
| `all_features`        | `BONJOUR, CARGO!` | `every_feature_together` |
