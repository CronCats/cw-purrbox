# Boolean contract caller

This contract calls the `boolean-contract`, but it does so by creating a CronCat task, and instructing it to call the `toggle` method. (This flips the sole state variable from true to false, and vice versa.)

When you instantiate the contract, you provide the CronCat Factory contract address as well as the boolean contract address.

To create the task, call the method `make_croncat_toggle_task` with no parameters.

This example is meant to demonstrate an extremely simple CronCat task that sends a Wasm Execute message. CronCat tasks can call any smart contract using the approach demonstrated here. 

## Get contract info from Factory

There are additional methods that demonstrate how dApps integrating CronCat can call `latest_contracts` and `latest_contract` on the Factory contract. Developers should plan on always using the latest versions of the contracts in order to have a smooth experience as updates/features are added in the future.

Given a CronCat Factory address (which is not expected to change) this smart contract has two execute methods:

- `demo_latest_contracts`: queries the Factory contract at the method [`latest_contracts`](https://docs.rs/croncat-sdk-factory/latest/croncat_sdk_factory/msg/enum.FactoryQueryMsg.html#variant.LatestContracts) which returns the latest versions of all the contracts and their addresses. Takes no parameters.
- `demo_latest_contract`: queries the Factory at [`latest_contract`](https://docs.rs/croncat-sdk-factory/latest/croncat_sdk_factory/msg/enum.FactoryQueryMsg.html#variant.LatestContract) which takes one parameter, `contract_name` (`String` that's validated as an address), which will return one contract address and version. This is slightly more efficient, and useful for integrations that only need one contract. It's understandable that some contracts may only need to query for the latest version/address of the Tasks contract, for instance.

Note that these query the Factory contract, but are execute methods. This is because integrating dApps will be using execute methods, and in them using these queries. These `demo_*` methods don't do anything, returning the default response. They're meant to be copy/pasted/modified for use in your dApp's CosmWasm contracts.

## Deploying and interacting

Optimize the contract:

    cargo opt

Brew some tea, it'll take a sec.

Deploy (Juno testnet):

    junod tx wasm store artifacts/boolean_contract_caller.wasm --node https://rpc.uni.junonetwork.io:443 --chain-id uni-6 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 -b block -o json -y --from MY_ACCOUNT_HERE

Take the `code_id`, replace the 904 below with it:

    junod tx wasm instantiate 904 '{"croncat_factory_address":"juno1w89cg4y6ta7enkeh3zzpha9c2say0k3dp83ygm27fyattx6v7r9qfyy384","boolean_address":"juno1u54ndscjm8887h97sk8punnwfutg2auu759efc5568l6zt70selqfe3jc9"}' --label "cw-boolean-contract-caller" --no-admin --node https://rpc.uni.junonetwork.io:443 --chain-id uni-6 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 -b block -o json -y --from MY_ACCOUNT_HERE

This will return the contract address. Let's call each method…

### `make_croncat_toggle_task`

Calling this method will sign and send an execute message to our new contract. It asks the Factory contract for the latest Task contract address. Then the contract uses [CosmWasm submessages](https://book.cosmwasm.com/actor-model/contract-as-actor.html?highlight=submess#sending-submessages) (also known as cross-contract calls) to create a CronCat task that will execute an action every block. The action? Call the `toggle` method on a boolean contract.   

    junod tx wasm execute juno16z8qpkemf38lykq7avt9crul4gy8u4xf0mhjlwa8fc0ydwv36t9snxnzqh '{"make_croncat_toggle_task":{}}' --node https://rpc.uni.junonetwork.io:443 --chain-id uni-6 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 -b block -o json -y --amount 1000000ujunox --from MY_ACCOUNT_HERE

### `demo_latest_contracts` (plural)

    junod tx wasm execute juno1kpswxja2c6g4ku9asg0ptkmajkmy3dt55nflamzkt6tcp4epf09qcs90sk '{"demo_latest_contracts":{}}' --node https://rpc.uni.junonetwork.io:443 --chain-id uni-6 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 -b block -o json -y --from MY_ACCOUNT_HERE

### `demo_latest_contract` (singular)

    junod tx wasm execute juno1kpswxja2c6g4ku9asg0ptkmajkmy3dt55nflamzkt6tcp4epf09qcs90sk '{"demo_latest_contract":{"contract_name":"tasks"}}' --node https://rpc.uni.junonetwork.io:443 --chain-id uni-6 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 -b block -o json -y --from MY_ACCOUNT_HERE

**Note**: be sure to check the [CronCat documentation](https://docs.cron.cat) for the latest version of the Factory contract.

## Other notes

This project is organized differently than the [CosmWasm template](https://github.com/CosmWasm/cw-template) you may see elsewhere. This is intentional, and was brought up in [this video](https://twitter.com/mikedotexe/status/1597126479661654017), and some details followed up in [this post](https://medium.com/cosmwasm/dev-note-1-moving-schema-rs-from-examples-to-bin-7c1b8cde7fc8) from Simon Warta.

One of the changes is the usage of [`cargo run script`](https://crates.io/crates/cargo-run-script).

You'll need to install that if you're using the handy shortcut commands detailed below. To install it:

    cargo install cargo-run-script

If you look in the `.cargo/config` file, you'll see there are these commands available:

### Common aliases

- `cargo wasm` — build the smart contract in release mode. This is not optimized, and may yield binaries that are too large to store on Cosmos chains.
- `opt`/`optimize` — before deploying the contract, use this command. It will compile with the canonical hash.
- `cargo unit-test` — runs the unit tests.
- `cargo pretty` — formats all the code [according to `fmt`](https://github.com/rust-lang/rustfmt).
- `cargo schema` — builds the JSON Schema files. It uses logic in the `schema-maker` directory, outputting results to the `schema` directory.

### Less-common aliases
- `cargo wasm-debug` — similar to `cargo wasm` but the build type is meant for debugging. (Developers may not use this command much, but is available.)
- `optm1`/`optimize-m1` — this type of optimization will complete faster, but is not considered stable. It will also create a binary with a different code hash. Please do not deploy binaries created with this command, as it will make things difficult for the community to confirm that the source code matches the on-chain code hash.
