#!/bin/bash

# Build wasm32-unknown-unknown binary
cargo build --locked --release --target wasm32-unknown-unknown

./scripts/check-wasm.sh

# Run tests
export RPC_URL=http://localhost:8547
cargo test --locked --test "integration_tests" $@ --features "export-abi debug" -- --nocapture
