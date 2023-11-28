#!/bin/bash

account="kuriboh"
# subdenom="abcde"
account_mnemonic="crucial decide eager peasant dinner frog coral thing boss card illness expire approve amount insect elevator fix orange raccoon language cereal whisper home around"
account_address="sei1kazt8cuamcvz3j2vlc3k9clhzsj30lpy28fsal"
admin_address="sei1ae5z7p4rdh2f976vjzs5u9f8pkt29kashkty5p"
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
