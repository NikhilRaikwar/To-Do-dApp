#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ListResponse, QueryMsg};
use crate::state::{Entry, Priority, Status, ENTRY_SEQ, LIST};

const CONTRACT_NAME: &str = "to-do-list";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let validated_owner = deps.api.addr_validate(&msg.owner)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    ENTRY_SEQ.save(deps.storage, &0u64)?;
    Ok(Response::default()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", validated_owner.as_str()))
}
