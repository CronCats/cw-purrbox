#!/bin/sh
set -e
SH_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
SH_DIR="$(cd -P "$(dirname "${SH_PATH}")";pwd)"
SC_PATH="$(cd -P "$(dirname "${SH_PATH}")/../..";pwd)"
SCRIPTS_PATH="$(cd -P "$(dirname "${SH_PATH}")/..";pwd)"

echo "CONTRACT-DIR: $SC_PATH"
echo "SCRIPT-DIR: $SH_DIR"
cd $SC_PATH

echo "Initializing vars"
. $SH_DIR/base/init-vars.sh

if [ -z "$1" ]
then
    echo "Must provide versioner contract address"
    exit 1
fi
if [ -z "$2" ]
then
    echo "Must provide daodao address contract address"
    exit 1
fi
VERSIONER_ADDRESS="$1"
DAODAO_ADDR="$2"

echo "DAODAO ADDRESS - " $DAODAO_ADDR
echo "VERSIONER ADDRESS - "$VERSIONER_ADDRESS
CREATE_MSG='{
	"create_versioner": {
		"daodao_addr": "'$DAODAO_ADDR'",
		"name": "cw-code-id-registry",
		"chain_id": "uni-5"
	}
}'
#CREATE_MSG='{"query_result":{}}'
echo $CREATE_MSG
junod tx wasm execute $VERSIONER_ADDRESS "$CREATE_MSG" --amount 1000000ujunox --from signer $TXFLAG -y
