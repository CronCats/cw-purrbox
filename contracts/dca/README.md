# DCA Junoswap

This is a template to interact with [cw-croncat](https://github.com/CronCats/cw-croncat) showing the power of scheduled transactions.
It also shows examples of how to interact with [JunoSwap](https://www.junoswap.com/) - the DEX on juno network.

## Key Logic

This contract shows how to dollar cost average with funds, using pre-defined "baskets". Each basket allows the user to specify the swap pool, how much to swap, interval and a max balance.

Core workflow is as follows:

  1. Deploy & Instantiate a DCA Vault (`res/dca_junoswap.wasm`)
  2. Add a "Caller" for CronCat - [Use the testnet deployed manager address](https://docs.cron.cat/docs/deployed-contracts/#manager)
  3. Add a new Basket, see below example of "JUNO-CRAB", change the variables to your needs
  4. Create a new automation task with CronCat! See below example for "create_task".
  5. Sit back & let the DCA automation flow!

Notice there are also some helper/query msgs below, if you want to check status of things along the way. Happy DCAing!

### Caution! Example Code Only

All of the code within this example should be taken for example sake, not ready for production or redistribution.

## Commands & Msgs

Instantiate

```
# lol - literally nothing to do here ;)
{}
```
```
# Query
{
  "get_config": {}
}
```

Add Caller: Croncat

```
# Exec
{
  "add_caller": {
    "caller": ""
  }
}
```

Add Basket: Croncat

```
# Exec
{
  "add_basket": {
    "id": "JUNO-CRAB",
    "basket": {
      "balance": {
        "amount": "1000000",
        "denom": "ujunox"
      },
      "swap_amount": "100000",
      "swap_address": "juno1j4ezvp80mnn75hlngak35n6wwzemxqjxdncdxc5n9dfw6s0q080qyhh9zl",
      "recipient": "YOUR_RECIPIENT",
      "input_token": "Token1",
      "min_interval": 15,
      "last_interval": 10
    }
  }
}
```

```
# Query
{
  "get_basket_ids": {}
}
```

```
# Query
{
  "get_basket_by_id": {
    "id": "JUNO-CRAB"
  }
}
```


Execute DCA directly:

```
# Exec
{
  "dca_swap_by_id": {
    "id": "JUNO-CRAB"
  }
}
```

## Create CronCat task for DCA: dca_swap_by_id

```
BASE64_TRANSFER=$(echo '{ "dca_swap_by_id": { "id": "JUNO-CRAB" } }' | base64)

{
  "create_task": {
    "task": {
      "interval": {
        "Block": 30
      },
      "boundary": null,
      "cw20_coins": [],
      "stop_on_fail": false,
      "actions": [
        {
            "msg": {
                "wasm": {
                    "execute": {
                        "contract_addr": "YOUR_DCA_CONTRACT_ADDR",
                        "msg": "eyAiZGNhX3N3YXBfYnlfaWQiOiB7ICJpZCI6ICJKVU5PLUNSQUIiIH0gfQ==",
                        "funds": []
                    }
                }
            },
            "gas_limit": null
        }
      ],
      "rules": []
    }
  }
}
```