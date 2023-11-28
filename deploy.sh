#!/bin/bash

contract_binary="$(pwd)/contract/target/wasm32-unknown-unknown/release/contract.wasm"
# contract_address="sei14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sh9m79m"

chain_id="sei-chain"
rpc_endpoint="tcp://127.0.0.1:26657"

seid tx wasm store $contract_binary -y --from=admin --chain-id=$chain_id --node=$rpc_endpoint --gas=10000000 --fees=1000000usei --broadcast-mode=block
seid tx wasm instantiate 1 '{}' --chain-id sei-chain --from admin --gas=4000000 --fees=1000000usei --broadcast-mode=block --label abcde --no-admin -y
