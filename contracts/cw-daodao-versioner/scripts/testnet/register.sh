#!/bin/sh
set -e

TXFLAG="--chain-id uni-5 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --broadcast-mode block --node https://rpc.uni.junonetwork.io:443"
REGISTRY_CONTRACT_ADDRESS=juno1k2z6m5duj8hnyc7wfk43wzxexc65zg0kp4pv2ccf83y4fe533c3qynes6j

junod query wasm contract-state smart $REGISTRY_CONTRACT_ADDRESS '{"list_registrations":{"name": "cw-code-id-registry", "chain_id": "uni-5"}}' --node https://rpc.uni.junonetwork.io:443 --chain-id uni-5
echo "-----------"

REGISTER_MSG='{"register":{"contract_name": "cw-code-id-registry", "version": "0.1.0", "chain_id": "uni-5", "code_id": 1746, "checksum": "8608F8126D64B39C10433CB09481BA09299C208FF1A5E5B3DEAF9F1DEC6B2F2A"}}'
junod tx wasm execute $REGISTRY_CONTRACT_ADDRESS "$REGISTER_MSG" --from signer --node "https://rpc.uni.junonetwork.io:443" $TXFLAG -y
echo "-----------"
junod query wasm contract-state smart $REGISTRY_CONTRACT_ADDRESS '{"get_registration":{"name": "cw-code-id-registry", "chain_id": "uni-5","version": "0.1.0"}}' --node https://rpc.uni.junonetwork.io:443 --chain-id uni-5
