#!/bin/bash


# Initialize variables with default values or empty strings
RPC="http://localhost:8547"
PRIVATE_KEY="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"
CONTRACT=""
DAY=""
SOLUTION=""

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
        --contract-address)
            CONTRACT="$2"
            shift 2
            ;;
        --day)
            DAY="$2"
            shift 2
            ;;
        --solution)
            SOLUTION="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

cast send --rpc-url "$RPC" --private-key "$PRIVATE_KEY" "$CONTRACT" "function setSolution(uint32 day, address solution) external" "$DAY"
