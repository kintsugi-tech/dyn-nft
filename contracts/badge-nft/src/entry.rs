use cosmwasm_std::{DepsMut, Env, MessageInfo};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use sg721_base::ContractError;
use sg_std::Response;

use crate::{contract::BadgeContract, msg::InstantiateMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    BadgeContract::default().instantiate(deps, env, info, msg)
}