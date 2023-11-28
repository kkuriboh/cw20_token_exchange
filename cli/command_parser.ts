import figlet from "figlet";
import kleur from "kleur";
import { program } from "commander";
import { bail } from "./utils";

type input = {
  amount: number;
  token_denom: string;
  operation: "wrap" | "unwrap";
  // user_address: string;
  wallet_privkey?: string;
};

program
  .option("--amount <amount>")
  .option("--operation <wrap|unwrap>")
  .option("--token-denom <denom>")
  // .option("--user-address <address>")
  .option("--wallet-privkey <privkey>");

export default (): input => {
  const banner = figlet.textSync("CROWNFI", {
    font: "Ogre",
  });
  console.log(kleur.cyan(banner));

  program.parse(process.argv);
  const opts = program.opts();

  const amount = Number(opts["amount"]);
  if (isNaN(amount)) bail("amount must be a number");

  const check_empty = (prop_name: string): string => {
    const value = opts[prop_name];
    if (value) return value;
    return bail(prop_name + " cannot be empty");
  };

  const operation = check_empty("operation") as any;
  if (operation !== "wrap" && operation !== "unwrap")
    bail("invalid option for operation");

  const token_denom = check_empty("tokenDenom");

  // const user_address = check_empty("userAddress");
  const wallet_privkey = opts["walletPrivkey"];

  return { amount, token_denom, operation, wallet_privkey };
};
