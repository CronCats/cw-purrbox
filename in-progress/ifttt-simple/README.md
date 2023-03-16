# IFTTT Simple

This is a template to interact with [cw-croncat]() showing the power of rule triggered transactions.

If you have seen or are familiar with "If This, Then That", croncat allows rules to trigger paid txns on chain. This gives you the flexibility to mix/match composability with multiple blockchains and services to automate tasks that you do a lot!

## Key Logic

The goal of this contract is to show how to chain 1 or 2 queries into a complex decision matrix before performing an `ExecuteMsg`.

Take a look at 2 core functions within this code:

* [Check block modulo, responds with Binary](./src/contract.rs#L84)
* [Check count modulo from msg, responds with Binary](./src/contract.rs#L104)

Having standardized response format of `(bool, T)`, contracts can chain any amount of boolean cascading logic.

### Caution! Example Code Only

All of the code within this example should be taken for example sake, not ready for production or redistribution.