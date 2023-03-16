# CW20 Basic

Note: this project is a modified version of the [cw20_base](https://github.com/CosmWasm/cw-plus/tree/645c781bcdd86c68448a039ac4881768255bd09c/contracts/cw20-base) contract.

It adds `unsafe_mint` which takes identical parameters to the `mint` method, meaning anyone can mint tokens. This will be useful for our end-to-end scripts since we're checking the behavior of cw20s in CronCat.

------

This is a basic implementation of a cw20 contract. It implements
the [CW20 spec](../../packages/cw20/README.md) and is designed to
be deployed as is, or imported into other contracts to easily build
cw20-compatible tokens with custom logic.

Implements:

- [x] CW20 Base
- [x] Mintable extension
- [x] Allowances extension

## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via: 

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw20_base.wasm .
ls -l cw20_base.wasm
sha256sum cw20_base.wasm
```

Or for a production-ready (optimized) build, run a build command in 
the repository root: https://github.com/CosmWasm/cw-plus#compiling.

## Importing this contract

You can also import much of the logic of this contract to build another
ERC20-contract, such as a bonding curve, overriding or extending what you
need.

Basically, you just need to write your handle function and import 
`cw20_base::contract::handle_transfer`, etc and dispatch to them.
This allows you to use custom `ExecuteMsg` and `QueryMsg` with your additional
calls, but then use the underlying implementation for the standard cw20
messages you want to support. The same with `QueryMsg`. You *could* reuse `instantiate`
as it, but it is likely you will want to change it. And it is rather simple.

Look at [`cw20-staking`](https://github.com/CosmWasm/cw-tokens/tree/main/contracts/cw20-staking) for an example of how to "inherit"
all this token functionality and combine it with custom logic.
