#!/bin/bash

MYDIR=$(realpath "$(dirname "$0")")
cd "$MYDIR" || exit
TEST_NODE_DIR="$MYDIR/../../nitro-testnode"
cd ../..

# Clone nitro-testnode
git clone -b release --recurse-submodules https://github.com/OffchainLabs/nitro-testnode.git
cd ./nitro-testnode || exit

# Initialize test node
./test-node.bash --no-run --init || exit

# Start with detached mode
./test-node.bash --detach
