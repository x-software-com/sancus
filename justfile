#!/usr/bin/env -S just --justfile

# Default Rust toolchain
rust-toolchain := "stable"

#
# Setup the environment:
#

setup-cargo-hack:
    cargo install --locked cargo-hack

setup-cargo-audit:
    cargo install --locked cargo-audit

setup-cargo-typos-cli:
   cargo install --locked typos-cli

setup-cargo-machete:
    cargo install cargo-machete

setup-git:
    git config pull.rebase true
    git config branch.autoSetupRebase always

setup-cargo-tools:
    cargo install --locked typos-cli
    cargo install --locked cargo-version-util

setup-cocogitto:
    cargo install --locked cocogitto
    cog install-hook --overwrite commit-msg

setup: setup-git setup-cargo-hack setup-cargo-audit setup-cargo-typos-cli setup-cargo-machete setup-cargo-tools setup-cocogitto self-update
    @echo "Done"

setup-ci: setup-cargo-hack setup-cargo-audit setup-cargo-tools
    git config --global --add safe.directory $(pwd)

#
# Recipes for cargo:
#

test rust-toolchain=rust-toolchain:
    cargo +{{rust-toolchain}} test --no-fail-fast --workspace --locked --all-features --all-targets

hack rust-toolchain=rust-toolchain: setup-cargo-hack
    cargo +{{rust-toolchain}} hack --feature-powerset --no-dev-deps check

clippy rust-toolchain=rust-toolchain:
    cargo +{{rust-toolchain}} clippy --quiet --release --all-targets --all-features

typos: setup-cargo-typos-cli
    typos

audit: setup-cargo-audit
    cargo audit

machete: setup-cargo-machete
    cargo machete --with-metadata

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
