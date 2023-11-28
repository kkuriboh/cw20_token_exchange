#!/bin/bash

account="kuriboh"
# subdenom="abcde"
account_mnemonic="sphere ship quick tool shoe erase clerk short release judge quantum floor glue innocent melt unusual west they have cloth release lemon silk rent"
account_address="sei1g33rnln6nkdfyj9f4hl00radpz7rw85c8fpmq7"
admin_address="sei1r8pk2z750p4jvl5sn9ft43alkypfck0vq2v69t"
# denom="factory/${admin_address}/${subdenom}"

contract_address="sei14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sh9m79m"

chain_id="sei-chain"
rpc_endpoint="tcp://127.0.0.1:26657"

# seid tx tokenfactory create-denom $subdenom --from $admin_address -b block -y --fees=100000usei
# seid tx tokenfactory mint 10000000$denom --from $admin_address -b block -y --fees=100000usei
# seid tx bank send $admin_address $account_address 10000000$denom -b block -y --fees=100000usei
seid tx bank send $admin_address $account_address 10000000uatom -b block -y --fees=100000usei
seid tx bank send $admin_address $account_address 10000000usei -b block -y --fees=100000usei

### you can make a binary with bun build --compile cli/index.ts --outfile cli
# export CONTRACT_ADDRESS=$contract_address
# bun run cli/index.ts --amount 1 --token-denom usei --wallet-privkey "${account_mnemonic}"

# - name: kuriboh
#   type: local
#   address: sei127qqmsczefs7h84v38a5awavgaxkff29t3y3hl
#   pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"As2VlE4dfpBcvuqp8OZD7k4Gvu4AaOPOijQ7LZ2Yj1OR"}'
#   mnemonic: ""
#
#
# **Important** write this mnemonic phrase in a safe place.
# It is the only way to recover your account if you ever forget your password.
#
# pumpkin message exhibit aunt museum shadow chef indicate trouble already crush burden damage print private hollow erode drive will cloth cotton good yard banner
