#!/bin/sh

cargo build --target wasm32-unknown-unknown --release;
cat neardev/dev-account >> neardev/history;
rm ./neardev/dev-account.env;
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/sharing_shard.wasm;
DEV_WALLET=`cat ./neardev/dev-account`;
export DEV_WALLET;
near call $DEV_WALLET new --accountId $DEV_WALLET
