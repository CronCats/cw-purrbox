#!/bin/bash

junod q wasm contract-state raw juno1q95stejlkevtwmxyacphr8hzfzl3q8ckqh4g5kwylty96lncfwqqf3sfcy 6c6173745f7461736b5f657865637574696f6e5f696e666f | jq -r '.data | @base64d' | jq


