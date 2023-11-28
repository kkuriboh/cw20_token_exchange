use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use contract::ExecMessage;

fn main() {
    write_api! {
        execute: ExecMessage,
        migrate: Empty,
        instantiate: Empty,
    };
}
