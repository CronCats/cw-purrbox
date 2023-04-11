# Get contracts from Factory demo

## Usage

Deploy:

    junod tx wasm store artifacts/create_task_handle_tick.wasm --gas-prices 0.04ujunox --gas-adjustment 1.3 --gas auto -b block -y --from mikereg

Instantiate:

    junod tx wasm instantiate 1410 --label "create-tick-croncat-task" '{"croncat_factory_address":"juno13su56h2jk52h0qh5hddpldyl7085t3vhs3vm92afplpvpmswuajsv4kxgg"}' --no-admin --gas-prices 0.04ujunox --gas-adjustment 1.3 --gas auto -b block -y --from mikereg

Create a normal tick task:

    junod tx wasm execute juno1jezn738p6v2a49gh0w2azzuh0mqa4cgkenlygdgehd9fkfrxynzqn0qy2s '{"make_croncat_tick_task":{}}' --gas-prices 0.04ujunox --gas-adjustment 1.3 --gas auto -b block -y --amount 1000000ujunox --from mikereg

Check the last task execution info:

    junod q wasm contract-state raw juno1jezn738p6v2a49gh0w2azzuh0mqa4cgkenlygdgehd9fkfrxynzqn0qy2s 6c6173745f7461736b5f657865637574696f6e5f696e666f
    
    junod q wasm contract-state raw juno1jezn738p6v2a49gh0w2azzuh0mqa4cgkenlygdgehd9fkfrxynzqn0qy2s 6c6173745f7461736b5f657865637574696f6e5f696e666f | jq -r '.data | @base64d' | jq

Call `tick` directly:


Call it directly, expecting a failure:

    junod tx wasm execute juno1jezn738p6v2a49gh0w2azzuh0mqa4cgkenlygdgehd9fkfrxynzqn0qy2s '{"tick":{}}' --gas-prices 0.04ujunox --gas-adjustment 1.3 --gas auto -b block -y --from mikereg

TODO: add node and chain id flags

    junod tx wasm execute juno1jezn738p6v2a49gh0w2azzuh0mqa4cgkenlygdgehd9fkfrxynzqn0qy2s '{"tick_fail":{}}' --gas-prices 0.04ujunox --gas-adjustment 1.3 --gas auto -b block -y --from mikereg

Create CronCat task that'll call `tick_fail`:

    junod tx wasm execute juno1jezn738p6v2a49gh0w2azzuh0mqa4cgkenlygdgehd9fkfrxynzqn0qy2s '{"make_croncat_tick_fail_task":{}}' --node https://rpc.uni.junonetwork.io:443 --chain-id uni-6 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 -b block -o json -y --amount 1000000ujunox --from mikereg
    