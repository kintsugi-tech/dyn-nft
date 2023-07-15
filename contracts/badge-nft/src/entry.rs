use common::badge_nft::QueryMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Deps, StdResult, Binary, to_binary};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use sg721_base::ContractError;
use sg_std::Response;
use crate::{contract::BadgeContract, msg::{InstantiateMsg, ExecuteMsg}, state::{ID_TO_SPECIAL_ROLE, METADATA}};
use sg721_base::ExecuteMsg as ExecuteMsgLegacy;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    BadgeContract::default().instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let badge_nft = BadgeContract::default();
    match msg {
        ExecuteMsg::TurnMetadata { token_id, role } => turn_metadata(deps, token_id, role),
        ExecuteMsg::Legacy (legacy_msg) => {
            match legacy_msg.clone() {
                ExecuteMsgLegacy::TransferNft {
                    token_id,
                    ..
                } => badge_nft.assert_transferrable(deps.as_ref(), token_id)?,
                ExecuteMsgLegacy::SendNft {
                    token_id,
                    ..
                } => badge_nft.assert_transferrable(deps.as_ref(), token_id)?,
                ExecuteMsgLegacy::Approve {
                    token_id,
                    ..
                } => badge_nft.assert_transferrable(deps.as_ref(), token_id)?,
                _ => (),
            }
            badge_nft.parent.execute(deps, env, info, legacy_msg)
        }
    }
}

fn turn_metadata(deps: DepsMut, token_id: String, role: String) -> Result<Response, ContractError> {
    ID_TO_SPECIAL_ROLE.save(deps.storage, token_id, &role)?;
    Ok(Response::new().add_attribute("action", "turn_metadata"))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let badge_nft = BadgeContract::default();
    // We implement two custom query methods: `nft_info` and `all_nft_info`.
    // For all other queries, simply dispatch them to the parent.
    match msg {
        QueryMsg::NftInfo {
            token_id,
        } => to_binary(&badge_nft.nft_info(deps, token_id)?),
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => to_binary(&badge_nft.all_nft_info(deps, env, token_id, include_expired)?),
        _ => badge_nft.parent.query(deps, env, msg),
    }
}