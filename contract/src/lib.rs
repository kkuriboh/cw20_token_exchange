use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, BankMsg, DepsMut, Empty, Env, MessageInfo, Response, StdError, SubMsg,
};
use cw_storage_plus::Set;
use sei_cosmwasm::SeiMsg;

const TOKEN_HISTORY: Set<&str> = Set::new("TOKEN_HISTORY");

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMessage,
) -> Result<Response<SeiMsg>, Errors> {
    let mut hasher = blake3::Hasher::new();
    let mut make_subdenom = |data: &[u8]| {
        hasher.update(data);
        let mut out_reader = hasher.finalize_xof();
        let mut buffer = [0; 22];
        out_reader.fill(&mut buffer);

        let mut hex = arrayvec::ArrayString::<44>::new();
        let table = b"0123456789abcdef";
        for &b in buffer.iter() {
            hex.push(table[(b >> 4) as usize] as char);
            hex.push(table[(b & 0xf) as usize] as char);
        }
        hex
    };

    let return_denom = |subdenom: &str| format!("factory/{}/{subdenom}", env.contract.address);

    match msg {
        ExecMessage::CW20ToNative => {
            let mut create_denom_messages = vec![];
            let mut mint_messages = vec![];
            let mut coins_to_send = vec![];

            for coin in info.funds {
                let subdenom = make_subdenom(coin.denom.as_bytes()).to_string();
                let new_denom = return_denom(&subdenom);

                if !TOKEN_HISTORY.exists(deps.storage, &new_denom) {
                    create_denom_messages.push(SeiMsg::CreateDenom { subdenom });
                    let _ = TOKEN_HISTORY.save(deps.storage, &new_denom);
                }

                let new_coin = cosmwasm_std::Coin {
                    denom: new_denom,
                    amount: coin.amount,
                };
                mint_messages.push(SeiMsg::MintTokens {
                    amount: new_coin.clone(),
                });
                coins_to_send.push(new_coin);
            }

            let send_tokens_msg = SubMsg::new(BankMsg::Send {
                amount: coins_to_send,
                to_address: info.sender.to_string(),
            });

            Ok(Response::new()
                .add_messages(create_denom_messages)
                .add_messages(mint_messages)
                .add_submessage(send_tokens_msg))
        }

        ExecMessage::NativeToCW20 { denom } => {
            let new_denom = return_denom(&make_subdenom(denom.as_bytes()));
            if !TOKEN_HISTORY.exists(deps.storage, &new_denom) {
                return Err(Errors::UnexchangedToken);
            }

            let amount = info.funds.iter().map(|c| c.amount).sum();
            let send_tokens_msg = BankMsg::Send {
                to_address: info.sender.to_string(),
                amount: vec![cosmwasm_std::Coin { denom, amount }],
            };

            let coins_to_burn = info
                .funds
                .into_iter()
                .map(|coin| SeiMsg::BurnTokens { amount: coin });

            Ok(Response::new()
                .add_messages(coins_to_burn)
                .add_submessage(SubMsg::new(send_tokens_msg)))
        }
    }
}

#[cw_serde]
pub enum ExecMessage {
    CW20ToNative,
    NativeToCW20 { denom: String },
}

#[cw_serde]
pub enum Errors {
    LowBalance,
    InternalError(String),
    UnexchangedToken,
    StorageFailed(&'static str),
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::LowBalance => {
                write!(
                    f,
                    "Sender or contract doesnt' have enough tokens to perform this operation"
                )
            }
            Errors::InternalError(str) => write!(f, "Internal error: {str}"),
            Errors::StorageFailed(context) => write!(f, "Storage failed: {context}"),
            Errors::UnexchangedToken => write!(f, "This token has no history of being exchanged"),
        }
    }
}
