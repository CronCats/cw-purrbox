# CronCat Successive Retries

This simple CosmWasm contract will be instantiated with two addresses:

1. The CronCat factory address of the given network. (See: https://docs.cron.cat/docs/deployed-contracts)
2. A DAO address for public goods funding.

Then, during instantiation, it'll create a task to call itself in 6 blocks. In our pretend scenario, we expect the contract to fail a couple times, and have retry logic. 

In this example, we'll check for the existence of funds in the smart contract. This being a simple example, we'll wait for double the funds to be in the new contract's balance. If double the funds are there before the maximum retries, we send it to the provided public funding DAO address. If, however, the maximum number of retries is reached, the contract ceases to retry, which is its failure logic. Then it's just… basically inert.

So it's all or nothing! In this silly example, if you don't reach your goal, the funds are trapped by the final retry forever. Brutal.

## Usage

Deploy:

    junod tx wasm store artifacts/successive_retries.wasm --gas-prices 0.04ujunox --gas-adjustment 1.3 --gas auto -b block -y --from mikereg

Instantiate:

(Remember, this will create the initial CronCat task)

    junod tx wasm instantiate 1410 --label "CronCat successive retries" '{"croncat_factory_address":"juno13su56h2jk52h0qh5hddpldyl7085t3vhs3vm92afplpvpmswuajsv4kxgg","public_funding_address":"juno178zy27yelrnr8d86u8q5yhgtr7d6jfyadrrsw9kqr78ykxh897gssas2v2"}' --no-admin --gas-prices 0.04ujunox --gas-adjustment 1.3 --gas auto -b block -y --from mikereg

## Testing

    cargo t

The above will run the tests, hiding any warnings.
