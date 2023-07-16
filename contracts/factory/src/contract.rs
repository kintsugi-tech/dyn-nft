use badges::Badge;
use common::factory::EventInfo;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, to_binary, CosmosMsg, SubMsg, WasmMsg, StdError, Reply};
use sg_metadata::Metadata;

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg},
    state::{ADMIN, BADGE_NFT_CODE_ID, BADGE_TO_EVENT, TOTAL_BADGES},
};

use badge_nft::msg::InstantiateMsg as BadgeNftInstantiateMsg;

const INSTANTIATE_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ADMIN.save(deps.storage, &msg.admin)?;

    BADGE_NFT_CODE_ID.save(deps.storage, &msg.badge_nft_code_id)?;

    TOTAL_BADGES.save(deps.storage, &0)?;
    
    let res = Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", &msg.admin);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateBadgeNft {
            api_url,
            event_info,
            collection_info,
            metadata,
            roles,
            badge } => create_badge_nft(deps, env, info, api_url, event_info, collection_info, metadata, roles, badge),
    }
}

fn create_badge_nft(deps: DepsMut,
                    env: Env,
                    info: MessageInfo,
                    api_url: String,
                    event_info: EventInfo,
                    collection_info: sg721::CollectionInfo<sg721::RoyaltyInfoResponse>,
                    metadata: Vec<Metadata>,
                    roles: Vec<String>,
                    badge: Badge)
        -> Result<Response, ContractError> {

    let badge_nft_instantiate_msg = BadgeNftInstantiateMsg {
        factory: env.contract.address.into(),
        api_url,
        collection_info,
        metadata,
        roles,
        badge,   
    };

    let num_badge = TOTAL_BADGES.load(deps.storage)?;

    let sub_msg = SubMsg {
        id: INSTANTIATE_REPLY_ID,
        msg: CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: None,
            code_id: BADGE_NFT_CODE_ID.load(deps.storage)?,
            msg: to_binary(&badge_nft_instantiate_msg)?,
            funds: vec![],
            label: format!("badge_nft{}", num_badge)
        }),
        gas_limit: None,
        reply_on: cosmwasm_std::ReplyOn::Success
    };

    BADGE_TO_EVENT.save(deps.storage, num_badge, &(event_info, None))?;

    let res = Response::new()
        .add_submessage(sub_msg)
        .add_attribute("action", "create_badge_nft");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    // parse the reply
    let res = cw_utils::parse_reply_instantiate_data(msg).map_err(|_| {
        StdError::parse_err("MsgInstantiateContractResponse", "failed to parse data")
    })?;

    let badge_nft_addr = deps.api.addr_validate(res.contract_address.as_str())?;

    let num_badge = TOTAL_BADGES.load(deps.storage)?;

    BADGE_TO_EVENT.update(deps.storage, num_badge, |Some((event_info, mut address))| {
        address = Some(badge_nft_addr);
        Ok((event_info, address))
    })?;
    unimplemented!()
}