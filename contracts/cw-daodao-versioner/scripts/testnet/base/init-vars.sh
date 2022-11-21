#!/bin/bash
CHAIN_ID="uni-5"
BINARY="junod"
PLATFORM="-arm64"
WASM_POSTFIX="-aarch64"
DIR=$(pwd)
JUNO_DIR="$HOME/juno"
DIR_NAME=$(basename "$PWD")

IMAGE_NAME="juno-node-1"
DIR_NAME_SNAKE=$(echo $DIR_NAME | tr '-' '_')
WASM="artifacts/$DIR_NAME_SNAKE$WASM_POSTFIX.wasm"
STAKE_TOKEN=ujunox
STAKE=${STAKE_TOKEN:-ustake}
TXFLAG="--gas-prices 0.075$STAKE --gas auto --gas-adjustment 1.3 -y -b block --chain-id $CHAIN_ID --node $RPC"
RECREATE_ARTIFACTS=0
RECREATE_CONTAINERS=0
RPC="https://rpc.uni.junonetwork.io:443"
NODE="--node $RPC"
TXFLAG="--node $RPC --chain-id uni-5 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --broadcast-mode block"

# Reset
NoColor='\033[0m' # Text Reset
# Regular Colors
Black='\033[0;30m'  # Black
Red='\033[0;31m'    # Red
Green='\033[0;32m'  # Green
Yellow='\033[0;33m' # {Yellow}
Blue='\033[0;34m'   # Blue
Purple='\033[0;35m' # Purple
Cyan='\033[0;36m'   # Cyan
White='\033[0;37m'  # White

