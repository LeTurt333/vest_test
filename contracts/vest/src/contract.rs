#[cfg(not(feature = "library"))]
pub use cosmwasm_std::{
    coin, entry_point, from_binary, has_coins, to_binary, Addr, Binary, Coin, CosmosMsg, Deps,
    DepsMut, Empty, Env, MessageInfo, Order, QueryRequest, Reply, ReplyOn, Response, StdError,
    StdResult, SubMsg, Uint128, WasmMsg, WasmQuery,
};

pub use crate::error::*;
pub use crate::msg::*;
pub use cw2::set_contract_version;
use anybuf::Anybuf;

const CONTRACT_NAME: &str = "crates.io:vesting";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateVest { vest_to } => {
            // Validate vest to address
            let valid_vest_to = deps.api.addr_validate(&vest_to)?;
            encode_msg_create_vesting_acct(&info.sender, &valid_vest_to, env)
        }
    }
}

fn encode_msg_create_vesting_acct(vest_from: &Addr, vest_to: &Addr, env: Env) -> Result<Response, ContractError> {

    // Test with 1 junox
    // [ ] - Check if this fails when vest_from (sender) has less than 1 junox

    let one_juno: Coin = coin(1_000_000, "ujunox");

    // about 1 hr
    let end_vest = env.block.height + 600;

    // MsgCreateVestingAccount
    // https://github.com/cosmos/cosmos-sdk/blob/c0fe4f7da17b7ec17d9bea6fcb57b4644f044b7a/proto/cosmos/vesting/v1beta1/tx.proto#LL18C9-L18C9

    let proto = Anybuf::new()
        .append_string(1, vest_from)
        .append_string(2, vest_to)
        .append_message(3, &Anybuf::new()
            .append_string(1, &one_juno.denom)
            .append_string(2, &one_juno.amount.to_string())
        )
        .append_uint64(4, end_vest)
        .append_bool(5, false)
        .into_vec();

    let msg = CosmosMsg::Stargate { 
        type_url: "/cosmos.vesting.v1beta1.MsgCreateVestingAccount".to_string(), 
        value: proto.into() 
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("new", "vesting_account")
        .add_attribute("from", format!("{}", vest_from.as_str()))
        .add_attribute("to", format!("{}", vest_to.as_str()))
        .add_attribute("amount", format!("{}", one_juno.amount))
        .add_attribute("end_block", format!("{}", end_vest.to_string()))
    )
}
