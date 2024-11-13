#!/usr/bin/env -S just --justfile

# Select a Rust toolchain - default is stable
toolchain := "stable"

# Internal variable for toolchain selection
rust-toolchain := "+" + toolchain

#
# Setup the environment:
#

setup-cargo-hack:
    cargo install --locked cargo-hack

setup-cargo-audit:
    cargo install --locked cargo-audit

setup-git:
    git config pull.rebase true
    git config branch.autoSetupRebase always

setup-cargo-tools:
    cargo install --locked typos-cli
    cargo install --locked cargo-version-util

setup-cocogitto:
    cargo install --locked cocogitto
    cog install-hook --overwrite commit-msg

setup: setup-git setup-cargo-hack setup-cargo-audit setup-cargo-tools setup-cocogitto self-update
    @echo "Done"

setup-ci: setup-cargo-hack setup-cargo-audit setup-cargo-tools
    git config --global --add safe.directory $(pwd)

#
# Recipes for cargo:
#

test:
    cargo {{rust-toolchain}} test --no-fail-fast --workspace --locked --all-features --all-targets

hack: setup-cargo-hack
    cargo {{rust-toolchain}} hack --feature-powerset --no-dev-deps check

clippy:
    cargo {{rust-toolchain}} clippy --quiet --release --all-targets --all-features

audit: setup-cargo-audit
    cargo audit

cargo-fmt:
    cargo fmt --all

cargo-fmt-check:
    cargo fmt --check

#
# Misc recipes:
#

self-update:
    cargo install --locked just

clean:
    cargo clean
