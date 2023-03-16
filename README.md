# Purrbox 😽

Sample contracts for both testing and reference on-chain contracts.

## Contracts

Those contracts are made for example purposes only. For more realistic example contracts, see [cosmwasm-examples](https://github.com/CosmWasm/cosmwasm-examples).

**Examples**

| Title | Description |
|---|---|
| [IFTTT Simple](./contracts/ifttt-simple) | Check if boolean is true, then increment an integer |
| [DCA JunoSwap](./contracts/dca) | Configure pools you want to dollar cost average swap |

### Builds

You can [build and deploy](https://github.com/CronCats/cw-purrbox/releases) by the CI for every release tag. In case you need to build them manually for some reason, use the following commands:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.8 ./contracts/*