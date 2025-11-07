#!/usr/bin/bash
ROOT=$(git rev-parse --show-toplevel)
ROLE="validator"
NUMBER="2"
BASE_PATH="$ROOT/id-chain-$ROLE-node-$NUMBER"
BIN=$ROOT/target/release/id-chain
PORT=30334
RPC_PORT=9946
NAME="PANDI_2"
VALIDATOR="bob"
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
        --$VALIDATOR \
        --database paritydb \
        --port $PORT \
        --rpc-port $RPC_PORT \
        --validator \
        --rpc-methods Unsafe \
        --rpc-cors all \
        --rpc-external \
        --prometheus-external \
        --force-authoring \
        --name $NAME \
        --bootnodes /ip4/$BOOTNODE_IP/tcp/30333/p2p/$BOOTNODE_IDENTITY

else
    echo "spawning an id-chain node in development mode"
    $BIN --tmp \
        --chain local \
        --$VALIDATOR \
        --database paritydb \
        --port $PORT \
        --rpc-port $RPC_PORT \
        --validator \
        --rpc-methods Unsafe \
        --rpc-cors all \
        --rpc-external \
        --prometheus-external \
        --force-authoring \
        --name $NAME \
        --discover-local
fi

# ./target/release/id-chain \
#
