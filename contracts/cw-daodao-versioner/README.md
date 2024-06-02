## Set registry contract address

```bash
NODE="--node https://rpc.uni.junonetwork.io:443"
TXFLAG="--node https://juno-testnet-rpc.polkachu.com:443 --chain-id uni-5 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --broadcast-mode block"
REGISTRY_CONTRACT_ADDRESS=❓
CRONCAT_ADDRESS=❓
DAODAO_ADDR=❓
SIGNER_ADDR=$(junod keys show signer --address)

export REGISTRY_CONTRACT_ADDRESS
export CRONCAT_ADDRESS
export DAODAO_ADDR
```

## Query registrations
```bash
junod query wasm contract-state smart $REGISTRY_CONTRACT_ADDRESS '{"get_registration":{"name": "cw-code-id-registry", "chain_id": "uni-5"}}' --node "https://rpc.uni.junonetwork.io:443"
```
## Register new version in registrar

```bash
REGISTER_MSG='{"register":{"contract_name": "cw-code-id-registry", "version": "0.1.1", "chain_id": "uni-5", "code_id": 1749, "checksum": "8608F8126D64B39C10433CB09481BA09299C208FF1A5E5B3DEAF9F1DEC6B2F2A"}}'
junod tx wasm execute $REGISTRY_CONTRACT_ADDRESS "$REGISTER_MSG" --from signer --node https://juno-testnet-rpc.polkachu.com:443 --chain-id uni-5 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --broadcast-mode block -y
```
## Deploy versioner
```bash
./scripts/testnet/deploy.sh -w -c
```

## Create versioner new entry

```bash
VERSIONER_ADDRESS=❓
./scripts/testnet/create.sh $VERSIONER_ADDRESS $DAODAO_ADDR
```
## Get croncat tasks

```bash
./scripts/testnet/get-tasks.sh $CRONCAT_ADDRESS
```
## Call for task execution on croncat.  See more 👉 https://github.com/CronCats/cw-croncat

## Remove versioner if needed

```bash
./scripts/testnet/remove.sh $VERSIONER_ADDRESS
```
