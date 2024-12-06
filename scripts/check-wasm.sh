#!/bin/bash
set -e

mydir=$(dirname "$0")
cd "$mydir" || exit
cd ..

# Check contract wasm binary by crate name
check_wasm () {
  local CONTRACT_CRATE_NAME=$1
  local CONTRACT_BIN_NAME="${CONTRACT_CRATE_NAME//-/_}.wasm"

  echo
  echo "Checking contract $CONTRACT_CRATE_NAME"
  cargo stylus check --wasm-file ./target/wasm32-unknown-unknown/release/"$CONTRACT_BIN_NAME"
}

# Retrieve all alphanumeric contract's crate names in `./contracts/solutions` directory.
get_solution_crate_names () {
  # shellcheck disable=SC2038
  # NOTE: optimistically relying on the 'name = ' string at Cargo.toml file
  find ./contracts/solutions -maxdepth 2 -type f -name "Cargo.toml" | xargs grep 'name = ' | grep -oE '".*"' | tr -d "'\""
}

cargo build --release --target wasm32-unknown-unknown

# Checks solutions
for CRATE_NAME in $(get_solution_crate_names)
do
  check_wasm "$CRATE_NAME"
done

# Checks orchestrator
check_wasm "orchestrator"
