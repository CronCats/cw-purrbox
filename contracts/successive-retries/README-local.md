# CronCat Successive Retries

This simple CosmWasm contract will be instantiated with two addresses:

1. The CronCat factory address of the given network. (See: https://docs.cron.cat/docs/deployed-contracts)
2. A DAO address for public goods funding.

Then, during instantiation, it'll create a task to call itself in 6 blocks. In our pretend scenario, we expect the contract to fail a couple times, and have retry logic. 

In this example, we'll check for the existence of funds in the smart contract. If the funds are there before the maximum retries, we send it to the provided public funding DAO address. If, however, the maximum number of retries is reached, the contract ceases to retry, which is its failure logic.

## Usage

Deploy:

    osmosisd tx wasm store artifacts/successive_retries.wasm --gas auto --gas-prices 0.1uosmo --gas-adjustment 1.7 --from mikereg -y

Instantiate:

(Remember, this will create the initial CronCat task)

    osmosisd tx wasm instantiate 172 --label "CronCat successive retries" '{"croncat_factory_address":"osmo12r3fm9rdhae5v68pn6dju39y4tp3qd5mwaqcku9een8fnm2pjv0sa0n4gm","public_funding_address":"osmo1yhqft6d2msmzpugdjtawsgdlwvgq3samajy9jq"}' --no-admin --gas auto --gas-prices 0.1uosmo --gas-adjustment 1.7 --from mikereg --amount 600000uosmo -y

    osmosisd tx osmo1au3xz4u65s9pjjuy98pjqpzns86nvf7hy0jcwm3mtx43c0jlkmeqz3f7j3

Check the last task execution info:

    osmosisd q wasm contract-state raw juno1jezn738p6v2a49gh0w2azzuh0mqa4cgkenlygdgehd9fkfrxynzqn0qy2s 6c6173745f7461736b5f657865637574696f6e5f696e666f
    
    osmosisd q wasm contract-state raw juno1jezn738p6v2a49gh0w2azzuh0mqa4cgkenlygdgehd9fkfrxynzqn0qy2s 6c6173745f7461736b5f657865637574696f6e5f696e666f | jq -r '.data | @base64d' | jq