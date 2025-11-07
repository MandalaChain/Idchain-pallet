#!/usr/bin/bash
ROOT=$(git rev-parse --show-toplevel)
ROLE="archive"
NUMBER="3"
BASE_PATH="$ROOT/id-chain-$ROLE-node-$NUMBER"
BIN=$ROOT/target/release/id-chain
PORT=30339
RPC_PORT=9951
NAME="PANDI_ARCHIVE_3"
PURGE="n"
CHAIN=$ROOT/res/raw-local.json

printf "are you running in production environment? (y/n) default(n) : "
read PROD

if [ "$PROD" = "y" ]; then
    echo "spawning an id-chain node in production mode"

    if [ ! -d $BASE_PATH ]; then
        echo "$BASE_PATH directory does not exist"
        echo "creating $BASE_PATH directory"
        mkdir $BASE_PATH
    else
        echo "$BASE_PATH directory exists"

        printf "purge db? (y/n) default(n): "
        read PURGE

        if [ "$PURGE" = "y" ]; then
            echo "purging db"
            rm -rf $BASE_PATH
        fi
    fi

    BOOTNODE_IP=""
    BOOTNODE_IDENTITY=""
    echo "enter the first node's ip address : "
    read BOOTNODE_IP
    echo "enter the first node's local identity : "
    read BOOTNODE_IDENTITY

    $BIN --base-path $BASE_PATH \
        --chain $CHAIN \
        --database paritydb \
        --port $PORT \
        --rpc-port $RPC_PORT \
        --rpc-methods Unsafe \
        --rpc-cors all \
        --rpc-external \
        --prometheus-external \
        --sync full \
        --state-pruning archive \
        --name $NAME \
        --bootnodes /ip4/$BOOTNODE_IP/tcp/30333/p2p/$BOOTNODE_IDENTITY

else
    echo "spawning an id-chain node in development mode"
    $BIN --tmp \
        --chain local \
        --database paritydb \
        --port $PORT \
        --rpc-port $RPC_PORT \
        --rpc-methods Unsafe \
        --rpc-cors all \
        --rpc-external \
        --prometheus-external \
        --sync full \
        --state-pruning archive \
        --name $NAME \
        --discover-local
fi

# ./target/release/id-chain \
#
