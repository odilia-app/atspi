#!/bin/sh
set -u

# Host: volledige workspace, default features.
cargo check \
    --workspace --all-targets \
    --message-format=json-diagnostic-rendered-ansi \
    --keep-going

# wasm32-unknown-unknown: alleen atspi-common, geen default features.
cargo check \
    -p atspi-common --no-default-features \
    --target wasm32-unknown-unknown \
    --message-format=json-diagnostic-rendered-ansi \
    --locked --keep-going

# wasm32-wasip1: idem.
cargo check \
    -p atspi-common --no-default-features \
    --target wasm32-wasip1 \
    --message-format=json-diagnostic-rendered-ansi \
    --locked --keep-going
