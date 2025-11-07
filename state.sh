#! bash

CHAIN=$1
CHAINS=("local")

if [ -z "$1" ]; then
    echo "Usage: state.sh <chain> [local, all]"
    exit 1
fi

ROOT=$(git rev-parse --show-toplevel)

CHAIN_PATH=$ROOT/res/$CHAIN.json
BIN=$ROOT/target/release/id-chain

cd $ROOT
echo "Building node"
cargo build --release > /dev/null 2>&1
echo "Node built"

gen_state() {
    CHAIN=$1
    PATH=$ROOT/res/$CHAIN.json
    RAW_PATH=$ROOT/res/raw-$CHAIN.json

    $BIN build-spec --chain $PATH --raw --disable-default-bootnode >$RAW_PATH
}

if [ "$1" == "all" ]; then

    for chain in "${CHAINS[@]}"; do
        echo "Generating state for $chain"
        gen_state $chain
    done
    exit 0
else
    gen_state $CHAIN

fi
