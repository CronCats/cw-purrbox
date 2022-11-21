#!/bin/sh
set -e
source ~/.profile
SH_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
SH_DIR="$(cd -P "$(dirname "${SH_PATH}")";pwd)"
SC_PATH="$(cd -P "$(dirname "${SH_PATH}")/../..";pwd)"
SCRIPTS_PATH="$(cd -P "$(dirname "${SH_PATH}")/..";pwd)"

echo "CONTRACT-DIR: $SC_PATH"
echo "SCRIPT-DIR: $SH_DIR"
cd $SC_PATH
if [ -z "$1" ]
then
    echo "Must provide versioner contract address"
    exit 1
fi
VERSIONER_ADDRESS="$1"
echo "Initializing vars"
. $SH_DIR/base/init-vars.sh

REMOVE_MSG='{"remove_versioner":{"name": "cw-code-id-registry", "chain_id": "uni-5"}}'

echo $CREATE_MSG
junod tx wasm execute $VERSIONER_ADDRESS "$REMOVE_MSG" --amount 1000000ujunox --from signer $TXFLAG -y
