#!/bin/bash

junod tx wasm execute juno1q95stejlkevtwmxyacphr8hzfzl3q8ckqh4g5kwylty96lncfwqqf3sfcy '{"proxy_call":{}}' --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --from agent -y

