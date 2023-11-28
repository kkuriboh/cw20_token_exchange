import * as seicore from "@crownfi/sei-js-core";
import { blake3 } from "@noble/hashes/blake3";

import type { ExecMessage } from "./types";
import parse_arguments from "./command_parser";
import consts from "./consts";

const parsed_arguments = parse_arguments();
process.stdout.write("\n");

const wallet = await seicore.restoreWallet(
  parsed_arguments.wallet_privkey ?? consts.DEFAULT_SEED_PHRASE
);

const client = await seicore.getSigningCosmWasmClient(
  consts.RPC_ENDPOINT,
  wallet
);

const accounts = await wallet.getAccounts();
const address = accounts[0].address;

const hex_table = "0123456789abcdef";
let subdenom = "";
const denom_hash = blake3(parsed_arguments.token_denom, { dkLen: 22 });
for (const byte of denom_hash) {
  subdenom += hex_table[byte >> 4];
  subdenom += hex_table[byte & 0xf];
}

const coin_denom = `factory/${consts.CONTRACT_ADDRESS}/${subdenom}`;

const print_balances = async () => {
  const balance_outcoming = await client.getBalance(
    address,
    parsed_arguments.token_denom
  );

  const balance_incoming = await client.getBalance(address, coin_denom);

  console.log(
    `current balance:\n\tnative tokens: ${balance_incoming.amount}\n\tCW20 tokens: ${balance_outcoming.amount}`
  );
};

await print_balances();

try {
  const [message, denom]: [ExecMessage, string] =
    parsed_arguments.operation === "wrap"
      ? ["c_w20_to_native", parsed_arguments.token_denom]
      : [
          { native_to_c_w20: { denom: parsed_arguments.token_denom } },
          coin_denom,
        ];

  await client.execute(
    address,
    consts.CONTRACT_ADDRESS,
    message,
    { gas: "999999", amount: [{ amount: "100000", denom: "usei" }] },
    undefined,
    [
      {
        amount: String(parsed_arguments.amount),
        denom,
      },
    ]
  );
  console.log("\nTRANSACTION SUCCEDED\n");
} catch (err) {
  console.log(`${err}`);
}

await print_balances();
