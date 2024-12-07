#!/bin/bash

# Initialize variables with default values or empty strings
RPC="http://localhost:8547"
PRIVATE_KEY="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"
CONTRACT=""

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --rpc)
            RPC="$2"
            shift 2
            ;;
        --private-key)
            PRIVATE_KEY="$2"
            shift 2
            ;;
        --contract-name)
            CONTRACT="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Your deployment logic here
echo "RPC: [$RPC]"
echo "Private key: [$PRIVATE_KEY]"
# echo "Contract: [$CONTRACT]"

./scripts/check-wasm.sh

cargo build --release --target wasm32-unknown-unknown

cargo stylus deploy --endpoint "$RPC" --private-key "$PRIVATE_KEY" --wasm-file ./target/wasm32-unknown-unknown/release/"$CONTRACT".wasm
