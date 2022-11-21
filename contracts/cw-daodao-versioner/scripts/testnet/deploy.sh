#!/bin/sh

set -e
SH_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
SH_DIR="$(cd -P "$(dirname "${SH_PATH}")";pwd)"
SC_PATH="$(cd -P "$(dirname "${SH_PATH}")/../..";pwd)"
SCRIPTS_PATH="$(cd -P "$(dirname "${SH_PATH}")/..";pwd)"

echo "CONTRACT-DIR: $SC_PATH"
echo "SCRIPT-DIR: $SH_DIR"
cd $SC_PATH

$SCRIPTS_PATH/build.sh
echo "Initializing vars"
. $SH_DIR/base/init-vars.sh

usage() {
  printf "Usage: $SH_DIR/versioner.sh -w -c"
}
flags() {
  while test $# -gt 0; do
    case "$1" in
    -w | --recreate-artifacts)
      RECREATE_ARTIFACTS=1
      ;;
    -c | --recreate-containers)
      RECREATE_CONTAINERS=1
      ;;
    -a | --all)
      RECREATE_ARTIFACTS=1
      RECREATE_CONTAINERS=1
      ;;
    -\? | -h | --help)
      usage
      exit
      ;;
    --) # Stop option processing
      usage
      exit 1
      ;;
    -*)
      usage
      exit 1
      ;;
    *)
      usage
      exit 1
      ;;
    esac

    # and here we shift to the next argument
    shift
  done
}

if [[ -z "$@" ]]; then
  RECREATE_ARTIFACTS=0
  RECREATE_CONTAINERS=0
else
  flags "$@"
fi
if [[ -z "$RECREATE_ARTIFACTS" ]]; then
  RECREATE_ARTIFACTS=0
fi
if [[ -z "$RECREATE_CONTAINERS" ]]; then
  RECREATE_CONTAINERS=0
fi
echo "RECREATE_ARTIFACTS " $RECREATE_ARTIFACTS
echo "RECREATE_CONTAINERS " $RECREATE_CONTAINERS

#Recreate artifacts
if [ $RECREATE_ARTIFACTS == 1 ]; then
  #Remove local artifacts folder
  echo "deleting artifacts..."
  rm -rf "artifacts"
  # build optimized binary if it doesn't exist
  if [ ! -f "$WASM" ]; then
    echo "building optimized binary..."
    docker run --rm -v "$(pwd)":/code \
      --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
      --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
      --platform linux/arm64 \
      cosmwasm/rust-optimizer$PLATFORM:0.12.8
  fi
fi
#Recreate containers
if [ $RECREATE_CONTAINERS == 1 ]; then
  . $SH_DIR/base/init-addresses.sh
fi

echo "${Yellow}Instantiating smart contract...${NoColor}"
echo "REGISTRY_ADDR: $REGISTRY_CONTRACT_ADDRESS"
echo "CRONCAT_ADDR: $CRONCAT_ADDRESS"

RES=$(junod tx wasm store artifacts/cw_daodao_versioner$WASM_POSTFIX.wasm --from owner $TXFLAG -y --output json -b block)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "${Cyan}CODE_ID :" $CODE_ID "${NoColor}"

INIT='{"registrar_addr":"'$REGISTRY_CONTRACT_ADDRESS'","croncat_addr":"'$CRONCAT_ADDRESS'"}'

$BINARY tx wasm instantiate $CODE_ID "$INIT" --from owner --label "cw_dadao_versioner" $TXFLAG -y --no-admin
VERSIONER_CONTRACT_ADDRESS=$($BINARY q wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')
echo "${Cyan}VERSIONER_ADDRESS :" $VERSIONER_CONTRACT_ADDRESS "${NoColor}"
